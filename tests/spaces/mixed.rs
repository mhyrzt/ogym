use nalgebra::SVector;
use ogym::spaces::{Mixed, MixedItem, Space};

#[test]
fn test_mixed_discrete_creation() {
    let mixed = Mixed::<2>::discrete(5);
    assert!(mixed.is_ok());
}

#[test]
fn test_mixed_discrete_creation_invalid() {
    let mixed = Mixed::<2>::discrete(1); // Should fail due to invalid size
    assert!(mixed.is_err());

    use ogym::spaces::Error;
    match mixed {
        Err(Error::DiscreteInvalidSize) => assert!(true),
        _ => panic!("Expected DiscreteInvalidSize error"),
    }
}

#[test]
fn test_mixed_continuous_creation() {
    let low = SVector::from_vec(vec![0.0, -1.0]);
    let high = SVector::from_vec(vec![1.0, 2.0]);
    let mixed = Mixed::<2>::continuous(low, high);
    assert!(mixed.is_ok());
}

#[test]
fn test_mixed_continuous_creation_invalid_bounds() {
    let low = SVector::from_vec(vec![2.0, -1.0]);
    let high = SVector::from_vec(vec![1.0, 2.0]); // low > high for first element
    let mixed = Mixed::<2>::continuous(low, high);
    assert!(mixed.is_err());

    use ogym::spaces::Error;
    match mixed {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_mixed_sample_discrete() {
    let mixed = Mixed::<2>::discrete(5).unwrap();

    for _ in 0..10 {
        let sample = mixed.sample();
        assert!(sample.is_ok());
        match sample.unwrap() {
            MixedItem::Discrete(_) => assert!(true), // Success
            _ => panic!("Expected Discrete item"),
        }
    }
}

#[test]
fn test_mixed_sample_continuous() {
    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let mixed = Mixed::<2>::continuous(low, high).unwrap();

    for _ in 0..10 {
        let sample = mixed.sample();
        assert!(sample.is_ok());
        match sample.unwrap() {
            MixedItem::Continuous(value) => {
                assert_eq!(value.len(), 2);
                assert!(value[0] >= 0.0 && value[0] <= 1.0);
                assert!(value[1] >= 0.0 && value[1] <= 1.0);
            }
            _ => panic!("Expected Continuous item"),
        }
    }
}

#[test]
fn test_mixed_contains_discrete_valid() {
    let mixed = Mixed::<2>::discrete(5).unwrap();
    let value = MixedItem::Discrete(3);
    assert!(mixed.contains(&value).is_ok());
}

#[test]
fn test_mixed_contains_discrete_invalid() {
    let mixed = Mixed::<2>::discrete(5).unwrap();
    let value = MixedItem::Discrete(7); // Outside valid range [0, 5)
    assert!(mixed.contains(&value).is_err());

    use ogym::spaces::Error;
    match mixed.contains(&value) {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_mixed_contains_continuous_valid() {
    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let mixed = Mixed::<2>::continuous(low, high).unwrap();
    let value = MixedItem::Continuous(SVector::from_vec(vec![0.5, 0.8]));
    assert!(mixed.contains(&value).is_ok());
}

#[test]
fn test_mixed_contains_continuous_invalid() {
    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let mixed = Mixed::<2>::continuous(low, high).unwrap();
    let value = MixedItem::Continuous(SVector::from_vec(vec![1.5, 0.8])); // 1.5 is outside [0, 1]
    assert!(mixed.contains(&value).is_err());

    use ogym::spaces::Error;
    match mixed.contains(&value) {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_mixed_contains_type_mismatch() {
    let discrete_mixed = Mixed::<2>::discrete(5).unwrap();
    let continuous_value = MixedItem::Continuous(SVector::from_vec(vec![0.5, 0.8]));
    assert!(discrete_mixed.contains(&continuous_value).is_err());

    use ogym::spaces::Error;
    match discrete_mixed.contains(&continuous_value) {
        Err(Error::TypeMismatch) => assert!(true),
        _ => panic!("Expected TypeMismatch error"),
    }

    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let continuous_mixed = Mixed::<2>::continuous(low, high).unwrap();
    let discrete_value = MixedItem::Discrete(2);
    assert!(continuous_mixed.contains(&discrete_value).is_err());

    match continuous_mixed.contains(&discrete_value) {
        Err(Error::TypeMismatch) => assert!(true),
        _ => panic!("Expected TypeMismatch error"),
    }
}

#[test]
fn test_mixed_shape_discrete() {
    let mixed = Mixed::<2>::discrete(5).unwrap();
    let shape = mixed.shape();
    assert_eq!(shape, vec![5]);
}

#[test]
fn test_mixed_shape_continuous() {
    let low = SVector::from_vec(vec![0.0, -1.0, 2.0]);
    let high = SVector::from_vec(vec![1.0, 2.0, 3.0]);
    let mixed = Mixed::<3>::continuous(low, high).unwrap();
    let shape = mixed.shape();
    assert_eq!(shape, vec![3]);
}

#[test]
fn test_mixed_bounds_discrete() {
    let mixed = Mixed::<2>::discrete(5).unwrap();
    let (low, high) = mixed.bounds();

    match (low, high) {
        (MixedItem::Discrete(l), MixedItem::Discrete(h)) => {
            assert_eq!(l, 0);
            assert_eq!(h, 4); // n - 1 = 5 - 1 = 4
        }
        _ => panic!("Expected both bounds to be discrete"),
    }
}

#[test]
fn test_mixed_bounds_continuous() {
    let low = SVector::<f64, 2>::from([0.0, -1.0]);
    let high = SVector::<f64, 2>::from([1.0, 2.0]);
    let mixed = Mixed::<2>::continuous(low, high).unwrap();
    let (actual_low, actual_high) = mixed.bounds();

    match (actual_low, actual_high) {
        (MixedItem::Continuous(l), MixedItem::Continuous(h)) => {
            assert_eq!(l, low);
            assert_eq!(h, high);
        }
        _ => panic!("Expected both bounds to be continuous"),
    }
}
