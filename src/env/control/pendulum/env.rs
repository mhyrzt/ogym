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
        let hs = SVector::from_vec(vec![1.0, 1.0, 8.0]);
        let ha = SVector::from_element(1.0);

        Ok(Self {
            space: EnvSpace {
                state: Boxed::new(-hs, hs)?,
                action: match config.continuous {
                    true => Mixed::discrete(config.n)?,
                    false => Mixed::continuous(-ha, ha)?,
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
        let theta = rng.random_range(-self.config.x0..self.config.x0);
        let omega = rng.random_range(-self.config.y0..self.config.y0);

        (theta, omega)
    }

    fn to_state(&self) -> SVector<f64, STATE_SIZE> {
        let (ts, tc) = self.theta.sin_cos();
        SVector::from_vec(vec![ts, tc, self.omega])
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
        self.space.action.contains(&action)?;
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }

        let u = match action {
            MixedItem::Discrete(a) => {
                (a as f64) * (2.0 * self.config.max_tau) / (self.config.n as f64 - 1.)
                    - self.config.max_tau
            }
            MixedItem::Continuous(a) => a[0] * self.config.max_tau,
        };
        let g = self.config.g;
        let l = self.config.l;
        let m = self.config.m;
        let dt = self.config.dt;

        let cost =
            normalize_angle(self.theta).powi(2) + 0.1 * self.omega.powi(2) + 1e-3 * u.powi(2);

        self.omega = (self.omega
            + dt * (3. * g * self.theta.sin() / (2. * l) + 3. * u / (m * l.powi(2))))
        .clamp(-self.config.max_tau, self.config.max_tau);

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
        assert!(pendulum.space.action.is_discrete());
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
        let config = PendulumConfig::default();
        let mut pendulum = Pendulum::new(config).unwrap();

        // Action irrelevant here
        let action = MixedItem::Discrete(0);
        let result = pendulum.step(action);

        assert!(matches!(result, Err(Error::NotInitialized)));
    }

    #[test]
    fn test_step_discrete_logic() {
        // Default config -> continuous=true -> Mixed::discrete logic in 'new'
        let config = PendulumConfig::default();
        let mut pendulum = Pendulum::new(config).unwrap();

        pendulum.reset(Some(0)).unwrap();

        // Action 0
        let action = MixedItem::Discrete(0);
        let exp = pendulum.step(action).unwrap();

        assert_eq!(exp.step, 1);
        assert!(!exp.terminal.is_terminated());
        assert!(!exp.terminal.is_truncated()); // max_t default is 200

        // Check state transition occurred
        assert_ne!(exp.next_state, exp.curr_state);
    }

    #[test]
    fn test_step_continuous_logic() {
        // with_discrete_action -> continuous=false -> Mixed::continuous logic in 'new'
        let config = PendulumConfig::default().with_discrete_action(2);
        let mut pendulum = Pendulum::new(config).unwrap();

        pendulum.reset(Some(0)).unwrap();

        let action_val = SVector::<f64, 1>::from_element(0.5);
        let action = MixedItem::Continuous(action_val);

        let exp = pendulum.step(action).unwrap();

        assert_eq!(exp.step, 1);
    }

    #[test]
    fn test_truncation() {
        let config = PendulumConfig::default().with_max_steps(2);
        let mut pendulum = Pendulum::new(config).unwrap();

        pendulum.reset(Some(0)).unwrap();

        // Step 1
        let exp1 = pendulum.step(MixedItem::Discrete(0)).unwrap();
        assert!(!exp1.terminal.is_truncated());
        assert!(!pendulum.is_truncated());
        // Step 2 (Max steps reached)
        let exp2 = pendulum.step(MixedItem::Discrete(0)).unwrap();
        assert!(exp2.terminal.is_truncated());
        assert!(pendulum.is_truncated());

        // Step 3 (Should fail)
        let res = pendulum.step(MixedItem::Discrete(0));
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
        let config = PendulumConfig::default()
            .with_initial_angle(0.0) // Start upright (unstable equilibrium approx)
            .with_initial_velocity(0.0);

        let mut pendulum = Pendulum::new(config).unwrap();

        // Force specific state to verify cost: theta=0, omega=0
        // Reset randomizes, so we might need to trust physics or mock RNG.
        // Instead, we run one step and check sign of reward.
        // Cost = theta^2 + 0.1*omega^2 + 0.001*u^2
        // Reward = -Cost
        // Reward should be negative or zero.

        pendulum.reset(None).unwrap();
        let exp = pendulum.step(MixedItem::Discrete(0)).unwrap();

        assert!(exp.reward <= 0.0);
    }

    #[test]
    fn test_invalid_action_space() {
        // Config is "continuous" (which means Discrete space in provided code logic)
        let config = PendulumConfig::default();
        let mut pendulum = Pendulum::new(config).unwrap();
        pendulum.reset(None).unwrap();

        // Pass Continuous action to Discrete space
        let invalid_action = MixedItem::Continuous(SVector::from_element(0.0));
        let res = pendulum.step(invalid_action);

        // The Space::contains check in step() should fail
        assert!(res.is_err());
    }
}
