use ogym::spaces::{MultiDiscrete, Space};

#[test]
fn test_multidiscrete_new_valid() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec);
    assert!(multidiscrete.is_ok());
}

#[test]
fn test_multidiscrete_new_empty() {
    let nvec = vec![]; // Empty vector should fail
    let multidiscrete = MultiDiscrete::new(nvec);
    assert!(multidiscrete.is_err());
    
    use ogym::spaces::Error;
    match multidiscrete {
        Err(Error::EmptyVec) => assert!(true),
        _ => panic!("Expected EmptyVec error"),
    }
}

#[test]
fn test_multidiscrete_new_invalid_size() {
    let nvec = vec![2, 1, 4]; // Second element is <= 1, should fail
    let multidiscrete = MultiDiscrete::new(nvec);
    assert!(multidiscrete.is_err());
    
    use ogym::spaces::Error;
    match multidiscrete {
        Err(Error::DiscreteInvalidSize) => assert!(true),
        _ => panic!("Expected DiscreteInvalidSize error"),
    }
}

#[test]
fn test_multidiscrete_new_invalid_size_zero() {
    let nvec = vec![2, 0, 4]; // Second element is 0, should fail
    let multidiscrete = MultiDiscrete::new(nvec);
    assert!(multidiscrete.is_err());
    
    use ogym::spaces::Error;
    match multidiscrete {
        Err(Error::DiscreteInvalidSize) => assert!(true),
        _ => panic!("Expected DiscreteInvalidSize error"),
    }
}

#[test]
fn test_multidiscrete_sample() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    for _ in 0..10 {
        let sample = multidiscrete.sample();
        assert!(sample.is_ok());
        let values = sample.unwrap();
        assert_eq!(values.len(), 3);
        assert!(values[0] < 2);
        assert!(values[1] < 3);
        assert!(values[2] < 4);
    }
}

#[test]
fn test_multidiscrete_contains_valid() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    let value = vec![1, 2, 3]; // All values within bounds
    assert!(multidiscrete.contains(&value).is_ok());
}

#[test]
fn test_multidiscrete_contains_invalid_bounds() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    let value = vec![1, 3, 3]; // Second value is 3, but must be < 3
    assert!(multidiscrete.contains(&value).is_err());
    
    use ogym::spaces::Error;
    match multidiscrete.contains(&value) {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_multidiscrete_contains_invalid_shape() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    let value = vec![1, 2]; // Different length than nvec
    assert!(multidiscrete.contains(&value).is_err());
    
    use ogym::spaces::Error;
    match multidiscrete.contains(&value) {
        Err(Error::ShapeMismatch) => assert!(true),
        _ => panic!("Expected ShapeMismatch error"),
    }
}

#[test]
fn test_multidiscrete_shape() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    let shape = multidiscrete.shape();
    assert_eq!(shape, vec![2, 3, 4]);
}

#[test]
fn test_multidiscrete_bounds() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    let (low, high) = multidiscrete.bounds();
    assert_eq!(low, vec![0, 0, 0]);
    assert_eq!(high, vec![1, 2, 3]); // nvec[i] - 1 for each element
}

#[test]
fn test_multidiscrete_to_u32_valid() {
    let nvec = vec![2, 3, 4]; // Total combinations = 2 * 3 * 4 = 24
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    // Test various valid conversions
    assert_eq!(multidiscrete.to_u32(&[0, 0, 0]).unwrap(), 0); // First combination
    assert_eq!(multidiscrete.to_u32(&[0, 0, 1]).unwrap(), 1);
    assert_eq!(multidiscrete.to_u32(&[0, 0, 2]).unwrap(), 2);
    assert_eq!(multidiscrete.to_u32(&[0, 0, 3]).unwrap(), 3);
    assert_eq!(multidiscrete.to_u32(&[0, 1, 0]).unwrap(), 4);
    assert_eq!(multidiscrete.to_u32(&[1, 2, 3]).unwrap(), 23); // Last combination
}

#[test]
fn test_multidiscrete_to_u32_invalid_bounds() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    // Test value out of bounds
    let result = multidiscrete.to_u32(&[2, 1, 2]); // First value should be < 2
    assert!(result.is_err());
    
    use ogym::spaces::Error;
    match result {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_multidiscrete_to_u32_invalid_shape() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    // Test wrong shape
    let result = multidiscrete.to_u32(&[1, 2]); // Should be length 3, not 2
    assert!(result.is_err());
    
    use ogym::spaces::Error;
    match result {
        Err(Error::ShapeMismatch) => assert!(true),
        _ => panic!("Expected ShapeMismatch error"),
    }
}

#[test]
fn test_multidiscrete_u32_to_multi_discrete_valid() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    // Test reverse conversions
    assert_eq!(multidiscrete.u32_to_multi_discrete(0).unwrap(), vec![0, 0, 0]);
    assert_eq!(multidiscrete.u32_to_multi_discrete(1).unwrap(), vec![0, 0, 1]);
    assert_eq!(multidiscrete.u32_to_multi_discrete(4).unwrap(), vec![0, 1, 0]);
    assert_eq!(multidiscrete.u32_to_multi_discrete(23).unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_multidiscrete_u32_to_multi_discrete_invalid() {
    let nvec = vec![2, 3, 4]; // Total combinations is 24
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    
    // Test value out of bounds
    let result = multidiscrete.u32_to_multi_discrete(24); // 24 is too large
    assert!(result.is_err());
    
    use ogym::spaces::Error;
    match result {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_multidiscrete_total_combinations() {
    let nvec = vec![2, 3, 4];
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    let total = multidiscrete.total_combinations().unwrap();
    assert_eq!(total, 24); // 2 * 3 * 4 = 24
}

#[test]
fn test_multidiscrete_total_combinations_overflow() {
    let nvec = vec![u32::MAX, 2]; // This should cause overflow
    let multidiscrete = MultiDiscrete::new(nvec).unwrap();
    let result = multidiscrete.total_combinations();
    assert!(result.is_err());
    
    use ogym::spaces::Error;
    match result {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}