use std::f64::consts::{PI, TAU};

use super::PendulumConfig;
use crate::{
    env::environment::{Environment, Error, Experience},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};
use nalgebra::SVector;
use rand::{rngs::StdRng, Rng, SeedableRng};

const STATE_SIZE: usize = 3;
const ACTION_SIZE: usize = 1;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

fn normalize_angle(x: f64) -> f64 {
    (x + PI).rem_euclid(TAU) - PI
}

#[derive(Debug)]
pub struct Pendulum {
    t: u32,
    config: PendulumConfig,
    state: Option<State>,
    theta: f64,
    omega: f64,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl Pendulum {
    pub fn new(config: PendulumConfig) -> Result<Self, Error> {
        let hs = SVector::from_vec(vec![1.0, 1.0, config.max_v]);
        let ha = SVector::from_element(config.max_tau);

        Ok(Self {
            space: EnvSpace {
                state: Boxed::new(-hs, hs)?,
                action: match config.continuous {
                    true => Mixed::continuous(-ha, ha)?,
                    false => Mixed::discrete(config.n)?,
                },
            },
            config,
            t: 0,
            state: None,
            theta: 0.,
            omega: 0.,
        })
    }

    fn rand_theta_omega(&self, seed: Option<u64>) -> (f64, f64) {
        let mut rng = match seed {
            Some(s) => StdRng::seed_from_u64(s),
            None => StdRng::from_os_rng(),
        };
        let theta = if self.config.x0 > 0.0 {
            rng.random_range(-self.config.x0..self.config.x0)
        } else {
            0.0
        };
        let omega = if self.config.y0 > 0.0 {
            rng.random_range(-self.config.y0..self.config.y0)
        } else {
            0.0
        };

        (theta, omega)
    }

    fn to_state(&self) -> SVector<f64, STATE_SIZE> {
        let (ts, tc) = self.theta.sin_cos();
        SVector::from_vec(vec![tc, ts, self.omega])
    }
}

impl Environment for Pendulum {
    type Action = Action;
    type State = State;
    type Info = ();

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        self.t = 0;
        (self.theta, self.omega) = self.rand_theta_omega(seed);
        let s = self.to_state();
        self.state = Some(s);
        Ok((s, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, Self::Action>, Error> {
        self.state()?;
        match (&self.space.action, &action) {
            (Mixed::Discrete(_), MixedItem::Discrete(_))
            | (Mixed::Continuous(_), MixedItem::Continuous(_)) => {}
            _ => return Err(crate::spaces::Error::TypeMismatch.into()),
        }
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }

        let u = match action {
            MixedItem::Discrete(a) => {
                self.space.action.contains(&action)?;
                (a as f64) * (2.0 * self.config.max_tau) / (self.config.n as f64 - 1.)
                    - self.config.max_tau
            }
            // Gymnasium clips out-of-range torque (np.clip) rather than
            // rejecting the step outright.
            MixedItem::Continuous(a) => a[0].clamp(-self.config.max_tau, self.config.max_tau),
        };
        let g = self.config.g;
        let l = self.config.l;
        let m = self.config.m;
        let dt = self.config.dt;

        let cost =
            normalize_angle(self.theta).powi(2) + 0.1 * self.omega.powi(2) + 1e-3 * u.powi(2);

        self.omega = (self.omega
            + dt * (3. * g * self.theta.sin() / (2. * l) + 3. * u / (m * l.powi(2))))
        .clamp(-self.config.max_v, self.config.max_v);

        self.theta += self.omega * dt;

        let curr_state = self.state()?;
        let next_state = self.to_state();
        self.state = Some(next_state);
        self.t += 1;

        Ok(Experience::new(
            curr_state,
            -cost,
            action,
            next_state,
            (),
            self.to_terminal()?,
            self.t,
        ))
    }

    fn state(&self) -> Result<Self::State, Error> {
        match self.state {
            Some(s) => Ok(s),
            None => Err(Error::NotInitialized),
        }
    }

    fn is_terminal(&self) -> Result<bool, Error> {
        Ok(false)
    }

    fn is_truncated(&self) -> bool {
        self.t >= self.config.max_t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::MixedItem;
    use nalgebra::SVector;
    use std::f64::consts::PI;

    #[test]
    fn test_normalize_angle() {
        assert!((normalize_angle(PI) - -PI).abs() < 1e-6);
        assert!((normalize_angle(3.0 * PI) - -PI).abs() < 1e-6);
        assert!((normalize_angle(0.0) - 0.0).abs() < 1e-6);
        assert!((normalize_angle(-PI) - -PI).abs() < 1e-6);
    }

    #[test]
    fn test_new_initialization() {
        let config = PendulumConfig::default();
        let pendulum = Pendulum::new(config).unwrap();

        assert_eq!(pendulum.t, 0);
        assert!(pendulum.state.is_none());
        // FIX: Default pendulum is Continuous
        assert!(pendulum.space.action.is_continuous());
    }

    #[test]
    fn test_reset() {
        let config = PendulumConfig::default();
        let mut pendulum = Pendulum::new(config).unwrap();

        let (state, _) = pendulum.reset(Some(42)).unwrap();

        // Check dimensions
        assert_eq!(state.len(), 3);

        // Check internal state update
        assert!(pendulum.state.is_some());
        assert_eq!(pendulum.state.unwrap(), state);

        // Check determinism
        let (state_2, _) = pendulum.reset(Some(42)).unwrap();
        assert_eq!(state, state_2);

        let (state_3, _) = pendulum.reset(Some(43)).unwrap();
        assert_ne!(state, state_3);
    }

    #[test]
    fn test_step_uninitialized() {
        let config = PendulumConfig::default().with_discrete_action(3);
        let mut pendulum = Pendulum::new(config).unwrap();

        let action = MixedItem::Discrete(0);
        let result = pendulum.step(action);

        assert!(matches!(result, Err(Error::NotInitialized)));
    }

    #[test]
    fn test_step_discrete_logic() {
        // FIX: Must explicitly configure discrete action for this test
        let config = PendulumConfig::default().with_discrete_action(3);
        let mut pendulum = Pendulum::new(config).unwrap();

        pendulum.reset(Some(0)).unwrap();

        // Action 0
        let action = MixedItem::Discrete(0);
        let exp = pendulum.step(action).unwrap();

        assert_eq!(exp.step, 1);
        assert!(!exp.terminal.is_terminated());
        assert!(!exp.terminal.is_truncated());

        assert_ne!(exp.next_state, exp.curr_state);
    }

    #[test]
    fn test_step_continuous_logic() {
        // FIX: Default config is continuous, so we can test continuous logic directly
        let config = PendulumConfig::default();
        let mut pendulum = Pendulum::new(config).unwrap();

        pendulum.reset(Some(0)).unwrap();

        let action_val = SVector::<f64, 1>::from_element(0.5);
        let action = MixedItem::Continuous(action_val);

        let exp = pendulum.step(action).unwrap();

        assert_eq!(exp.step, 1);
    }

    #[test]
    fn test_truncation() {
        // FIX: Use default continuous action for truncation test
        let config = PendulumConfig::default().with_max_steps(2);
        let mut pendulum = Pendulum::new(config).unwrap();

        pendulum.reset(Some(0)).unwrap();
        let action = MixedItem::Continuous(SVector::from_element(0.0));

        // Step 1
        let exp1 = pendulum.step(action).unwrap();
        assert!(!exp1.terminal.is_truncated());
        assert!(!pendulum.is_truncated());
        // Step 2 (Max steps reached)
        let exp2 = pendulum.step(action).unwrap();
        assert!(exp2.terminal.is_truncated());
        assert!(pendulum.is_truncated());

        // Step 3 (Should fail)
        let res = pendulum.step(action);
        assert!(matches!(res, Err(Error::EpisodeDone)));
    }

    #[test]
    fn test_state_getter() {
        let config = PendulumConfig::default();
        let mut pendulum = Pendulum::new(config).unwrap();

        assert!(matches!(pendulum.state(), Err(Error::NotInitialized)));

        let (s_reset, _) = pendulum.reset(None).unwrap();
        let s_get = pendulum.state().unwrap();

        assert_eq!(s_reset, s_get);
    }

    #[test]
    fn test_cost_calculation() {
        // FIX: Default is continuous, use continuous action
        let config = PendulumConfig::default()
            .with_initial_angle(0.0)
            .with_initial_velocity(0.0);

        let mut pendulum = Pendulum::new(config).unwrap();

        pendulum.reset(None).unwrap();
        let action = MixedItem::Continuous(SVector::from_element(0.0));
        let exp = pendulum.step(action).unwrap();

        assert!(exp.reward <= 0.0);
    }

    #[test]
    fn test_invalid_action_space() {
        // Default config is Continuous
        let config = PendulumConfig::default();
        let mut pendulum = Pendulum::new(config).unwrap();
        pendulum.reset(None).unwrap();

        // FIX: Pass Discrete action to Continuous space to trigger error
        let invalid_action = MixedItem::Discrete(0);
        let res = pendulum.step(invalid_action);

        assert!(res.is_err());
    }
}
