use super::error::Error;

pub trait Space {
    type Item;

    fn sample(&self) -> Result<Self::Item, Error>;
    fn contains(&self, value: &Self::Item) -> Result<(), Error>;
    fn shape(&self) -> Vec<usize>;
    fn bounds(&self) -> (Self::Item, Self::Item);
}

#[derive(Debug, Clone)]
pub struct EnvSpace<S: Space, A: Space> {
    pub state: S,
    pub action: A,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::error::Error;

    #[derive(Debug, Clone)]
    struct MockSpace {
        shape: Vec<usize>,
        bounds: (i32, i32),
        sample_val: i32,
    }

    impl Space for MockSpace {
        type Item = i32;

        fn sample(&self) -> Result<Self::Item, Error> {
            Ok(self.sample_val)
        }

        fn contains(&self, value: &Self::Item) -> Result<(), Error> {
            if *value >= self.bounds.0 && *value <= self.bounds.1 {
                Ok(())
            } else {
                Err(Error::InvalidBounds)
            }
        }

        fn shape(&self) -> Vec<usize> {
            self.shape.clone()
        }

        fn bounds(&self) -> (Self::Item, Self::Item) {
            self.bounds
        }
    }

    #[test]
    fn test_space_trait_implementation() {
        let space = MockSpace {
            shape: vec![2, 2],
            bounds: (0, 10),
            sample_val: 5,
        };

        assert_eq!(space.shape(), vec![2, 2]);
        assert_eq!(space.bounds(), (0, 10));
        assert_eq!(space.sample().unwrap(), 5);
        assert!(space.contains(&5).is_ok());
        assert_eq!(space.contains(&15), Err(Error::InvalidBounds));
    }

    #[test]
    fn test_env_space_struct() {
        let state_space = MockSpace {
            shape: vec![4],
            bounds: (0, 100),
            sample_val: 10,
        };

        let action_space = MockSpace {
            shape: vec![1],
            bounds: (-1, 1),
            sample_val: 0,
        };

        let env = EnvSpace {
            state: state_space.clone(),
            action: action_space.clone(),
        };

        assert_eq!(env.state.shape(), vec![4]);
        assert_eq!(env.action.shape(), vec![1]);

        let sampled_state = env.state.sample().unwrap();
        let sampled_action = env.action.sample().unwrap();

        assert_eq!(sampled_state, 10);
        assert_eq!(sampled_action, 0);
    }

    #[test]
    fn test_env_space_debug_and_clone() {
        let state_space = MockSpace {
            shape: vec![2],
            bounds: (0, 1),
            sample_val: 0,
        };
        let action_space = MockSpace {
            shape: vec![1],
            bounds: (0, 1),
            sample_val: 0,
        };

        let env = EnvSpace {
            state: state_space,
            action: action_space,
        };

        let env_clone = env.clone();

        let debug_str = format!("{:?}", env);
        assert!(debug_str.contains("EnvSpace"));
        assert!(debug_str.contains("state"));
        assert!(debug_str.contains("action"));

        assert_eq!(env.state.shape(), env_clone.state.shape());
    }
}
