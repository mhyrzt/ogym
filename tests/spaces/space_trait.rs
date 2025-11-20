use nalgebra::SVector;
use ogym::spaces::{Discrete, Boxed, EnvSpace, Space};

#[test]
fn test_env_space_creation() {
    let state_space = Boxed::<2>::new(
        SVector::from_vec(vec![0.0, -1.0]),
        SVector::from_vec(vec![1.0, 1.0])
    ).unwrap();
    
    let action_space = Discrete::new(5).unwrap();
    
    let env_space = EnvSpace {
        state: state_space,
        action: action_space,
    };
    
    // Verify that the spaces are properly contained
    assert_eq!(env_space.state.shape(), vec![2]);
    assert_eq!(env_space.action.shape(), vec![5]);
}

#[test]
fn test_env_space_clone() {
    let state_space = Boxed::<2>::new(
        SVector::from_vec(vec![0.0, -1.0]),
        SVector::from_vec(vec![1.0, 1.0])
    ).unwrap();
    
    let action_space = Discrete::new(5).unwrap();
    
    let env_space = EnvSpace {
        state: state_space,
        action: action_space,
    };
    
    // Clone the EnvSpace
    let cloned_env_space = env_space.clone();
    
    // Verify that both have the same shape
    assert_eq!(env_space.state.shape(), cloned_env_space.state.shape());
    assert_eq!(env_space.action.shape(), cloned_env_space.action.shape());
}

#[test]
fn test_trait_consistency() {
    // Test that all space implementations have the required methods
    let boxed = Boxed::<2>::new(
        SVector::from_vec(vec![0.0, 0.0]),
        SVector::from_vec(vec![1.0, 1.0])
    ).unwrap();
    
    let discrete = Discrete::new(5).unwrap();
    
    // Test sample
    assert!(boxed.sample().is_ok());
    assert!(discrete.sample().is_ok());
    
    // Test shape
    assert_eq!(boxed.shape(), vec![2]);
    assert_eq!(discrete.shape(), vec![5]);
    
    // Test bounds
    let (b_low, b_high) = boxed.bounds();
    assert_eq!(b_low.len(), 2);
    assert_eq!(b_high.len(), 2);
    
    let (d_low, d_high) = discrete.bounds();
    assert_eq!(d_low, 0);
    assert_eq!(d_high, 4);
}