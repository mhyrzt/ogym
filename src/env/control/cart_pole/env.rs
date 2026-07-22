use super::config::{CartPoleConfig, KinematicsIntegrator};
use crate::{
    env::environment::{Environment, Error, Experience},
    spaces::{Boxed, EnvSpace, Mixed, MixedItem, Space},
};
use nalgebra::SVector;

const FOUR_THIRDS: f64 = 4.0 / 3.0;
const STATE_SIZE: usize = 4;
const ACTION_SIZE: usize = 1;

type Action = MixedItem<ACTION_SIZE>;
type ActionSpace = Mixed<ACTION_SIZE>;

type State = SVector<f64, STATE_SIZE>;
type StateSpace = Boxed<STATE_SIZE>;

#[derive(Debug)]
pub struct CartPole {
    t: u32,
    config: CartPoleConfig,
    state: Option<State>,
    pub space: EnvSpace<StateSpace, ActionSpace>,
}

impl CartPole {
    pub fn new(config: CartPoleConfig) -> Result<Self, Error> {
        // Matches Gymnasium's CartPoleEnv: velocity and angular velocity are
        // reported but never bounded by the env itself, so their observation
        // bounds are effectively unbounded (Gym uses np.finfo(float32).max).
        let high: State = SVector::from_vec(vec![
            config.x_max * 2.0,
            f64::MAX,
            config.theta_max * 2.0,
            f64::MAX,
        ]);

        let space = EnvSpace {
            state: Boxed::new(-high, high)?,
            action: match config.continuous {
                false => Mixed::discrete(2)?,
                true => Mixed::continuous(SVector::from_element(-1.0), SVector::from_element(1.0))?,
            },
        };
        Ok(CartPole {
            space,
            config,
            t: 0,
            state: None,
        })
    }

    fn compute_acceleration(&self, state: &State, force: &f64) -> (f64, f64) {
        let theta: f64 = state[2];
        let omega: f64 = state[3];

        let (sin_theta, cos_theta) = theta.sin_cos();

        let m: f64 = self.config.mc + self.config.mp;
        let mpl: f64 = self.config.mp * self.config.l;

        let t: f64 = (force + mpl * omega.powi(2) * sin_theta) / m;

        let num: f64 = self.config.g * sin_theta - cos_theta * t;
        let den: f64 = self.config.l * (FOUR_THIRDS - self.config.mp * cos_theta.powi(2) / m);
        let theta_acc: f64 = num / den;
        let x_acc: f64 = t - mpl * theta_acc * cos_theta / m;

        (x_acc, theta_acc)
    }

    fn euler(&self, state: &State, x_acc: &f64, theta_acc: &f64) -> State {
        let x = state[0];
        let v = state[1];
        let theta = state[2];
        let omega = state[3];
        SVector::from_vec(vec![
            x + self.config.tau * v,
            v + self.config.tau * x_acc,
            theta + self.config.tau * omega,
            omega + self.config.tau * theta_acc,
        ])
    }

    fn semi_implicit_euler(&self, state: &State, x_acc: &f64, theta_acc: &f64) -> State {
        let x = state[0];
        let v = state[1] + self.config.tau * x_acc;
        let theta = state[2];
        let omega = state[3] + self.config.tau * theta_acc;
        SVector::from_vec(vec![
            x + self.config.tau * v,
            v,
            theta + self.config.tau * omega,
            omega,
        ])
    }

    fn integrate(&self, state: &State, force: &f64) -> State {
        let (x_acc, theta_acc) = self.compute_acceleration(state, force);
        match self.config.integrator {
            KinematicsIntegrator::Euler => self.euler(state, &x_acc, &theta_acc),
            KinematicsIntegrator::SemiImplicitEuler => {
                self.semi_implicit_euler(state, &x_acc, &theta_acc)
            }
        }
    }

    fn force(&self, action: &Action) -> Result<f64, Error> {
        match (&self.space.action, action) {
            (Mixed::Discrete(space), Action::Discrete(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                Ok((2.0 * *act as f64 - 1.0) * self.config.f)
            }
            (Mixed::Continuous(space), Action::Continuous(act)) => {
                space.contains(act).map_err(|_| Error::InvalidAction)?;
                Ok(act[0] * self.config.f)
            }
            _ => Err(Error::InvalidAction),
        }
    }
}

impl Environment for CartPole {
    type Action = Action;
    type State = State;
    type Info = ();

    fn reset(&mut self, seed: Option<u64>) -> Result<(Self::State, Self::Info), Error> {
        let state = self.space.state.uniform(seed, -5e-2, 5e-2)?;
        self.t = 0;
        self.state = Some(state);
        Ok((state, ()))
    }

    fn step(
        &mut self,
        action: Self::Action,
    ) -> Result<Experience<Self::State, Self::Info, self::Action>, Error> {
        if self.is_done()? {
            return Err(Error::EpisodeDone);
        }

        let curr_state = self.state()?;
        let force: f64 = self.force(&action)?;
        let next_state = self.integrate(&curr_state, &force);
        self.state = Some(next_state);
        self.t += 1;

        Ok(Experience::new(
            curr_state,
            1.,
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
        let state = self.state()?;
        Ok(state[0].abs() > self.config.x_max || state[2].abs() > self.config.theta_max)
    }

    fn is_truncated(&self) -> bool {
        self.t >= self.config.t_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::environment::Error;
    use crate::spaces::{Mixed, MixedItem};
    use nalgebra::SVector;

    fn default_config() -> CartPoleConfig {
        CartPoleConfig {
            mc: 1.0,
            mp: 0.1,
            l: 0.5,
            f: 10.0,
            g: 9.8,
            tau: 0.02,
            x_max: 2.4,
            theta_max: 0.21,
            t_max: 200,
            integrator: KinematicsIntegrator::Euler,
            continuous: false,
        }
    }

    #[test]
    fn test_new_discrete_and_continuous() {
        let cfg_discrete = default_config();
        let env_d = CartPole::new(cfg_discrete.clone()).unwrap();
        match env_d.space.action {
            Mixed::Discrete(_) => {}
            _ => panic!("Expected discrete action space"),
        }

        let mut cfg_cont = cfg_discrete.clone();
        cfg_cont.continuous = true;
        let env_c = CartPole::new(cfg_cont.clone()).unwrap();
        match env_c.space.action {
            Mixed::Continuous(_) => {}
            _ => panic!("Expected continuous action space"),
        }
    }

    #[test]
    fn test_compute_acceleration_values() {
        let cfg = default_config();
        let env = CartPole::new(cfg).unwrap();
        let state = SVector::<f64, STATE_SIZE>::from_vec(vec![0.0, 0.0, 0.1, 0.0]);
        let (xa, ta) = env.compute_acceleration(&state, &1.0);
        assert!(xa.is_finite());
        assert!(ta.is_finite());
    }

    #[test]
    fn test_euler_and_semi_implicit() {
        let cfg = default_config();
        let env = CartPole::new(cfg).unwrap();
        let state = SVector::<f64, STATE_SIZE>::from_element(0.0);
        let updated_euler = env.euler(&state, &1.0, &-1.0);
        assert_eq!(updated_euler.len(), STATE_SIZE);
        let updated_semi = env.semi_implicit_euler(&state, &1.0, &-1.0);
        assert_eq!(updated_semi.len(), STATE_SIZE);
    }

    #[test]
    fn test_integrate_switches() {
        let mut cfg = default_config();
        cfg.integrator = KinematicsIntegrator::Euler;
        let env_e = CartPole::new(cfg.clone()).unwrap();
        let state = SVector::<f64, STATE_SIZE>::from_vec(vec![0.0, 0.0, 0.1, 0.0]);
        let res_e = env_e.integrate(&state, &1.0);
        assert_eq!(res_e.len(), STATE_SIZE);

        cfg.integrator = KinematicsIntegrator::SemiImplicitEuler;
        let env_s = CartPole::new(cfg).unwrap();
        let res_s = env_s.integrate(&state, &1.0);
        assert_eq!(res_s.len(), STATE_SIZE);
    }

    #[test]
    fn test_force_discrete_and_continuous() {
        let cfg_d = default_config();
        let env_d = CartPole::new(cfg_d).unwrap();
        assert_eq!(
            env_d.force(&MixedItem::Discrete(0)).unwrap(),
            -env_d.config.f
        );
        assert_eq!(
            env_d.force(&MixedItem::Discrete(1)).unwrap(),
            env_d.config.f
        );

        let mut cfg_c = default_config();
        cfg_c.continuous = true;
        let env_c = CartPole::new(cfg_c).unwrap();
        let val = env_c
            .force(&MixedItem::Continuous(SVector::from_element(0.5)))
            .unwrap();
        assert!((val - env_c.config.f * 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_reset_initializes_state() {
        let cfg = default_config();
        let mut env = CartPole::new(cfg).unwrap();
        let (state, _) = env.reset(Some(42)).unwrap();
        assert_eq!(state.len(), STATE_SIZE);
        assert_eq!(env.t, 0);
        assert!(env.state.is_some());
    }

    #[test]
    fn test_step_advances_and_returns_experience() {
        let cfg = default_config();
        let mut env = CartPole::new(cfg).unwrap();
        env.reset(Some(123)).unwrap();
        let action = MixedItem::Discrete(1);
        let exp = env.step(action.clone()).unwrap();
        assert_eq!(exp.step, 1);
        assert_eq!(exp.action, action);
        assert_eq!(env.t, 1);
        assert!(env.state.is_some());
    }

    #[test]
    fn test_step_errors_if_done() {
        let mut cfg = default_config();
        cfg.x_max = 0.1;
        let mut env = CartPole::new(cfg).unwrap();
        env.reset(Some(1)).unwrap();
        {
            let mut s = env.state.unwrap();
            s[0] = 1.0;
            env.state = Some(s);
        }
        assert!(matches!(
            env.step(MixedItem::Discrete(0)),
            Err(Error::EpisodeDone)
        ));
    }

    #[test]
    fn test_state_and_not_initialized() {
        let cfg = default_config();
        let env = CartPole::new(cfg).unwrap();
        assert!(matches!(env.state(), Err(Error::NotInitialized)));
    }

    #[test]
    fn test_is_terminal_true_and_false() {
        let cfg = default_config();
        let mut env = CartPole::new(cfg.clone()).unwrap();
        env.reset(Some(1)).unwrap();
        {
            let mut s = env.state.unwrap();
            s[0] = cfg.x_max + 0.1;
            env.state = Some(s);
        }
        assert!(env.is_terminal().unwrap());
        env.reset(Some(2)).unwrap();
        assert!(!env.is_terminal().unwrap());
    }

    #[test]
    fn test_is_truncated() {
        let cfg = default_config();
        let mut env = CartPole::new(cfg.clone()).unwrap();
        env.t = cfg.t_max;
        assert!(env.is_truncated());
        env.t = cfg.t_max - 1;
        assert!(!env.is_truncated());
    }
}
