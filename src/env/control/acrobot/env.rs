use std::f64::consts::{PI, TAU};

use nalgebra::SVector;
use rand::{
    distr::{Distribution, Uniform},
    rngs::StdRng,
    Rng, SeedableRng,
};

use crate::{
    env::environment::{Environment, Error, Experience},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};

use super::{config::DynamicsMode, AcrobotConfig};

const ACTION_SIZE: usize = 1; // Discrete(3) or optionally continuous
const RAW_STATE_SIZE: usize = 4;
const STATE_SIZE: usize = 6;

type RawState = SVector<f64, RAW_STATE_SIZE>;
type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

#[derive(Debug, Clone)]
pub struct Acrobot {
    pub config: AcrobotConfig,
    t: u32,
    raw_state: Option<RawState>,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl Acrobot {
    pub fn new(config: AcrobotConfig) -> Result<Self, Error> {
        let ha = SVector::from_element(1.0);
        let hs = SVector::from_vec(vec![1., 1., 1., 1., 4. * PI, 9. * PI]);
        let space = EnvSpace {
            state: Boxed::new(-hs, hs)?,
            action: match config.continuous {
                true => Mixed::continuous(-ha, ha)?,
                false => Mixed::discrete(3)?,
            },
        };
        Ok(Self {
            config,
            space,
            t: 0,
            raw_state: None,
        })
    }

    fn raw(&self) -> Result<RawState, Error> {
        match self.raw_state {
            Some(s) => Ok(s),
            None => Err(Error::NotInitialized),
        }
    }

    fn tau(&self, action: &Action) -> Result<f64, Error> {
        let mut tau = match (&self.space.action, action) {
            (Mixed::Discrete(space), MixedItem::Discrete(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                (*act - 1) as f64
            }
            (Mixed::Continuous(space), MixedItem::Continuous(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                act[0]
            }
            _ => return Err(Error::InvalidAction),
        };

        if self.config.torque_noise_max >= 0.0 {
            tau += StdRng::from_os_rng()
                .random_range(-self.config.torque_noise_max..self.config.torque_noise_max);
        }
        Ok(tau)
    }

    fn ds_dt(&self, state: RawState, tau: f64) -> SVector<f64, RAW_STATE_SIZE> {
        let g = self.config.g;
        let m1 = self.config.link_mass_1;
        let m2 = self.config.link_mass_2;
        let l1 = self.config.link_length_1;
        let c1 = self.config.link_com_pos_1;
        let c2 = self.config.link_com_pos_2;
        let moi = self.config.link_moi;
        let th1 = state[0];
        let th2 = state[1];
        let dt1 = state[2];
        let dt2 = state[3];

        let (sin_th2, cos_th2) = th2.sin_cos();

        let d1 = m1 * c1 * c1 + m2 * (l1 * l1 + c2 * c2 + 2.0 * l1 * c2 * cos_th2) + moi * 2.0;
        let d2 = m2 * (c2 * c2 + l1 * c2 * cos_th2) + moi;

        let phi2 = m2 * c2 * g * (th1 + th2 - PI / 2.0).cos();
        let phi1 = -m2 * l1 * c2 * dt2 * dt2 * sin_th2 - 2.0 * m2 * l1 * c2 * dt2 * dt1 * sin_th2
            + (m1 * c1 + m2 * l1) * g * (th1 - PI / 2.0).cos()
            + phi2;

        let alpha2 = match self.config.dynamics_mode {
            DynamicsMode::Nips => {
                // Consistent with the paper
                (tau + d2 / d1 * phi1 - phi2) / (m2 * c2 * c2 + moi - d2 * d2 / d1)
            }
            DynamicsMode::Book => {
                // Consistent with java implementation and book
                (tau + d2 / d1 * phi1 - m2 * l1 * c2 * dt1 * dt1 * sin_th2 - phi2)
                    / (m2 * c2 * c2 + moi - d2 * d2 / d1)
            }
        };
        let alpha1 = -(d2 * alpha2 + phi1) / d1;

        SVector::<f64, RAW_STATE_SIZE>::new(dt1, dt2, alpha1, alpha2)
    }

    fn constraint(&self, mut state: RawState) -> RawState {
        state[0] = (state[0] + PI).rem_euclid(TAU) - PI;
        state[1] = (state[1] + PI).rem_euclid(TAU) - PI;
        state[2] = state[2].clamp(-self.config.max_vel_1, self.config.max_vel_1);
        state[3] = state[3].clamp(-self.config.max_vel_2, self.config.max_vel_2);

        state
    }

    fn rk4(&self, state: RawState, tau: f64) -> RawState {
        let dt = self.config.dt;
        let dt2 = dt / 2.0;

        let k1 = self.ds_dt(state, tau);
        let k2 = self.ds_dt(state + dt2 * k1, tau);
        let k3 = self.ds_dt(state + dt2 * k2, tau);
        let k4 = self.ds_dt(state + dt * k3, tau);

        self.constraint(state + dt / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4))
    }
}

impl Environment for Acrobot {
    type Action = Action;
    type State = State;
    type Info = ();

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.t = 0;
        let mut rng = match seed {
            Some(state) => StdRng::seed_from_u64(state),
            None => StdRng::from_rng(&mut rand::rng()),
        };
        let dist = Uniform::new(-0.1, 0.1)?;
        let raw_state: SVector<f64, RAW_STATE_SIZE> =
            SVector::from_fn(|_, _| dist.sample(&mut rng));

        self.raw_state = Some(raw_state);
        Ok((self.state()?, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }

        let curr_state = self.state()?;

        self.raw_state = Some(self.rk4(self.raw()?, self.tau(&action)?));
        let next_state = self.state()?;

        self.t += 1;

        Ok(Experience::new(
            curr_state,
            if self.is_terminal()? { 0.0 } else { -1.0 },
            action,
            next_state,
            (),
            self.to_terminal()?,
            self.t,
        ))
    }

    fn state(&self) -> Result<Self::State, Error> {
        let s = self.raw()?;
        let (sin_s0, cos_s0) = s[0].sin_cos();
        let (sin_s1, cos_s1) = s[1].sin_cos();

        Ok(SVector::from_vec(vec![
            cos_s0, sin_s0, cos_s1, sin_s1, s[2], s[3],
        ]))
    }

    #[inline]
    fn is_terminal(&self) -> Result<bool, Error> {
        let s = self.raw()?;
        Ok(s[0].cos() + (s[0] + s[1]).cos() < -1.)
    }

    #[inline]
    fn is_truncated(&self) -> bool {
        self.t >= self.config.max_t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::environment::Environment;
    use crate::spaces::{Mixed, MixedItem};
    use nalgebra::SVector;
    use std::f64::consts::PI;

    #[test]
    fn test_initialization_discrete() {
        let config = AcrobotConfig::default().with_discrete_action();
        let acrobot = Acrobot::new(config).unwrap();

        if let Mixed::Discrete(d) = &acrobot.space.action {
            assert_eq!(d.size(), 3);
        } else {
            panic!("Expected discrete action space");
        }
    }

    #[test]
    fn test_initialization_continuous() {
        let config = AcrobotConfig::default().with_continuous_action();
        let acrobot = Acrobot::new(config).unwrap();

        if let Mixed::Continuous(c) = &acrobot.space.action {
            assert_eq!(c.low[0], -1.0);
            assert_eq!(c.high[0], 1.0);
        } else {
            panic!("Expected continuous action space");
        }
    }

    #[test]
    fn test_reset() {
        let config = AcrobotConfig::default();
        let mut acrobot = Acrobot::new(config).unwrap();

        let (state, _) = acrobot.reset(Some(42)).unwrap();

        // State size should be 6 (cos/sin of 2 angles + 2 angular vels)
        assert_eq!(state.len(), 6);

        // Initial state should be small random values around 0
        // cos(small) ~ 1.0, sin(small) ~ 0.0
        assert!((state[0] - 1.0).abs() < 0.1); // cos(theta1)
        assert!(state[1].abs() < 0.2); // sin(theta1)
        assert!((state[2] - 1.0).abs() < 0.1); // cos(theta2)
        assert!(state[3].abs() < 0.2); // sin(theta2)
        assert!(state[4].abs() < 0.2); // vel1
        assert!(state[5].abs() < 0.2); // vel2

        // Check internal raw state
        let raw = acrobot.raw().unwrap();
        assert!(raw[0].abs() <= 0.1);
        assert!(raw[1].abs() <= 0.1);
        assert!(raw[2].abs() <= 0.1);
        assert!(raw[3].abs() <= 0.1);
    }

    #[test]
    fn test_step_discrete_actions() {
        let config = AcrobotConfig::default().with_discrete_action();
        let mut acrobot = Acrobot::new(config).unwrap();
        acrobot.reset(None).unwrap();

        // Action 0: Torque -1
        let action_0 = MixedItem::Discrete(0);
        let exp_0 = acrobot.step(action_0).unwrap();
        assert_eq!(exp_0.reward, -1.0);
        assert!(!exp_0.terminal.is_done());

        // Action 2: Torque +1
        let action_2 = MixedItem::Discrete(2);
        let _exp_2 = acrobot.step(action_2).unwrap();
    }

    #[test]
    fn test_step_continuous_actions() {
        let config = AcrobotConfig::default().with_continuous_action();
        let mut acrobot = Acrobot::new(config).unwrap();
        acrobot.reset(None).unwrap();

        // Torque 0.5
        let action = MixedItem::Continuous(SVector::from_element(0.5));
        let exp = acrobot.step(action).unwrap();

        assert_eq!(exp.reward, -1.0);
    }

    #[test]
    fn test_step_invalid_action_mismatch() {
        let config_disc = AcrobotConfig::default().with_discrete_action();
        let mut env_disc = Acrobot::new(config_disc).unwrap();
        env_disc.reset(None).unwrap();

        // Sending continuous action to discrete env
        let bad_action = MixedItem::Continuous(SVector::from_element(0.5));
        let res = env_disc.step(bad_action);
        assert!(res.is_err());
    }

    #[test]
    fn test_step_invalid_action_bounds() {
        let config = AcrobotConfig::default().with_discrete_action();
        let mut env = Acrobot::new(config).unwrap();
        env.reset(None).unwrap();

        // Action index 3 is out of bounds (0, 1, 2 allowed)
        let bad_action = MixedItem::Discrete(3);
        let res = env.step(bad_action);
        assert!(res.is_err());
    }

    #[test]
    fn test_terminal_condition() {
        let config = AcrobotConfig::default();
        let mut acrobot = Acrobot::new(config).unwrap();
        acrobot.reset(None).unwrap();

        // Manually set raw state to a terminal configuration
        // Condition: -cos(s[0]) - cos(s[1] + s[0]) > 1.0 (based on standard gym, but here implementation is:)
        // s[0].cos() + (s[0] + s[1]).cos() < -1.0
        // Let s[0] = PI (cos = -1), s[1] = 0 (cos(pi+0) = -1). Sum = -2.
        acrobot.raw_state = Some(SVector::from_vec(vec![PI, 0.0, 0.0, 0.0]));

        assert!(acrobot.is_terminal().unwrap());

        // Take a step in terminal state
        let action = MixedItem::Discrete(1);
        let exp = acrobot.step(action).unwrap();

        assert_eq!(exp.reward, 0.0);
        assert!(exp.terminal.is_terminated());
        assert!(exp.terminal.is_done());
    }

    #[test]
    fn test_truncation() {
        let max_t = 5;
        let config = AcrobotConfig::default().with_max_steps(max_t);
        let mut acrobot = Acrobot::new(config).unwrap();
        acrobot.reset(None).unwrap();

        for i in 0..max_t {
            assert!(!acrobot.is_truncated());
            let _ = acrobot.step(MixedItem::Discrete(1)).unwrap();
            assert_eq!(acrobot.t, i + 1);
        }

        assert!(acrobot.is_truncated());

        // Next step should fail due to done
        let res = acrobot.step(MixedItem::Discrete(1));
        assert!(matches!(res, Err(Error::EpisodeDone)));
    }

    #[test]
    fn test_constraint_wrapping() {
        let config = AcrobotConfig::default();
        let mut acrobot = Acrobot::new(config).unwrap();
        acrobot.reset(None).unwrap();

        // Set angle > PI, should wrap
        let large_angle = PI + 0.5;
        acrobot.raw_state = Some(SVector::from_vec(vec![large_angle, large_angle, 0.0, 0.0]));

        // Trigger constraint logic by stepping (which calls rk4 -> constraint)
        let _ = acrobot.step(MixedItem::Discrete(1)).unwrap();

        let raw_after = acrobot.raw().unwrap();
        // Just checking that it moved away from the raw large value into the canonical range [-PI, PI]
        // Note: Logic is (x + PI) % TAU - PI.
        // (PI + 0.5 + PI) % 2PI - PI = (2PI + 0.5) % 2PI - PI = 0.5 - PI = -2.64 approx
        assert!(raw_after[0] >= -PI && raw_after[0] <= PI);
        assert!(raw_after[1] >= -PI && raw_after[1] <= PI);
    }

    #[test]
    fn test_constraint_velocity_clamping() {
        let config = AcrobotConfig::default().with_max_velocities(1.0, 1.0); // Low limits
        let mut acrobot = Acrobot::new(config).unwrap();
        acrobot.reset(None).unwrap();

        // Set velocity very high
        acrobot.raw_state = Some(SVector::from_vec(vec![0.0, 0.0, 10.0, 10.0]));

        // Step to trigger constraint
        let _ = acrobot.step(MixedItem::Discrete(1)).unwrap();

        let raw_after = acrobot.raw().unwrap();
        // Should be clamped close to max_vel (1.0) plus small integration change
        assert!(raw_after[2] <= 1.5); // allowing some buffer for physics update
        assert!(raw_after[3] <= 1.5);
    }

    #[test]
    fn test_torque_noise() {
        let config = AcrobotConfig::default().with_torque_noise(5.0); // High noise
        let mut acrobot = Acrobot::new(config).unwrap();
        acrobot.reset(Some(123)).unwrap();

        // We can't easily inspect the noise added inside `tau`,
        // but we can ensure the step doesn't panic and produces valid state.
        let _ = acrobot.step(MixedItem::Discrete(1)).unwrap();

        let state = acrobot.state().unwrap();
        assert!(state.iter().all(|x| x.is_finite()));
    }
}
