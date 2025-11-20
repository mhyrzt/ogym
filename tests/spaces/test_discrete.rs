use ogym::spaces::{Discrete, Space};

#[test]
fn test_discrete_new_valid() {
    let discrete = Discrete::new(5);
    assert!(discrete.is_ok());
    
    let discrete = Discrete::new(2); // Minimum valid size
    assert!(discrete.is_ok());
}

#[test]
fn test_discrete_new_invalid_size() {
    let discrete = Discrete::new(1); // Size must be greater than 1
    assert!(discrete.is_err());
    
    use ogym::spaces::Error;
    match discrete {
        Err(Error::DiscreteInvalidSize) => assert!(true),
        _ => panic!("Expected DiscreteInvalidSize error"),
    }
}

#[test]
fn test_discrete_new_invalid_size_zero() {
    let discrete = Discrete::new(0); // Size must be greater than 1
    assert!(discrete.is_err());
    
    use ogym::spaces::Error;
    match discrete {
        Err(Error::DiscreteInvalidSize) => assert!(true),
        _ => panic!("Expected DiscreteInvalidSize error"),
    }
}

#[test]
fn test_discrete_sample() {
    let discrete = Discrete::new(5).unwrap();
    
    for _ in 0..10 { // Test multiple samples
        let sample = discrete.sample();
        assert!(sample.is_ok());
        let value = sample.unwrap();
        assert!(value < 5);
    }
}

#[test]
fn test_discrete_contains_valid() {
    let discrete = Discrete::new(5).unwrap();
    
    for i in 0..5 {
        let result = discrete.contains(&i);
        assert!(result.is_ok());
    }
}

#[test]
fn test_discrete_contains_invalid() {
    let discrete = Discrete::new(5).unwrap();
    
    // Test values outside the valid range
    assert!(discrete.contains(&5).is_err()); // 5 is not less than n=5
    assert!(discrete.contains(&10).is_err());
    
    use ogym::spaces::Error;
    match discrete.contains(&5) {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_discrete_shape() {
    let discrete = Discrete::new(5).unwrap();
    let shape = discrete.shape();
    assert_eq!(shape, vec![5]);
}

#[test]
fn test_discrete_shape_large() {
    let discrete = Discrete::new(100).unwrap();
    let shape = discrete.shape();
    assert_eq!(shape, vec![100]);
}

#[test]
fn test_discrete_bounds() {
    let discrete = Discrete::new(5).unwrap();
    let (low, high) = discrete.bounds();
    assert_eq!(low, 0);
    assert_eq!(high, 4); // n - 1 = 5 - 1 = 4
}

#[test]
fn test_discrete_bounds_edge_case() {
    let discrete = Discrete::new(2).unwrap(); // Minimum valid size
    let (low, high) = discrete.bounds();
    assert_eq!(low, 0);
    assert_eq!(high, 1); // n - 1 = 2 - 1 = 1
}