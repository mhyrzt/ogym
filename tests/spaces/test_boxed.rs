use nalgebra::SVector;
use ogym::spaces::{Boxed, Space};

#[test]
fn test_boxed_new_valid() {
    let low = SVector::from_vec(vec![0.0, -1.0]);
    let high = SVector::from_vec(vec![1.0, 2.0]);
    let boxed = Boxed::<2>::new(low, high);
    assert!(boxed.is_ok());
}

#[test]
fn test_boxed_new_invalid_bounds() {
    let low = SVector::from_vec(vec![2.0, -1.0]);
    let high = SVector::from_vec(vec![1.0, 2.0]); // low > high for first element
    let boxed = Boxed::<2>::new(low, high);
    assert!(boxed.is_err());

    // Also test that error is InvalidBounds
    use ogym::spaces::Error;
    match boxed {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_boxed_sample() {
    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let boxed = Boxed::<2>::new(low, high).unwrap();

    let sample = boxed.sample();
    assert!(sample.is_ok());
    let value = sample.unwrap();
    assert_eq!(value.len(), 2);
    assert!(value[0] >= 0.0 && value[0] <= 1.0);
    assert!(value[1] >= 0.0 && value[1] <= 1.0);
}

#[test]
fn test_boxed_contains_valid() {
    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let boxed = Boxed::<2>::new(low, high).unwrap();

    let value = SVector::from_vec(vec![0.5, 0.5]);
    assert!(boxed.contains(&value).is_ok());
}

#[test]
fn test_boxed_contains_invalid_bounds() {
    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let boxed = Boxed::<2>::new(low, high).unwrap();

    let value = SVector::from_vec(vec![1.5, 0.5]); // 1.5 is outside [0, 1]
    assert!(boxed.contains(&value).is_err());

    use ogym::spaces::Error;
    match boxed.contains(&value) {
        Err(Error::InvalidBounds) => assert!(true),
        _ => panic!("Expected InvalidBounds error"),
    }
}

#[test]
fn test_boxed_shape() {
    let low = SVector::from_vec(vec![0.0, -1.0, 2.0]);
    let high = SVector::from_vec(vec![1.0, 2.0, 3.0]);
    let boxed = Boxed::<3>::new(low, high).unwrap();

    let shape = boxed.shape();
    assert_eq!(shape, vec![3]);
}

#[test]
fn test_boxed_bounds() {
    let low = SVector::from_vec(vec![0.0, -1.0]);
    let high = SVector::from_vec(vec![1.0, 2.0]);
    let boxed = Boxed::<2>::new(low, high).unwrap();

    let (actual_low, actual_high) = boxed.bounds();
    assert_eq!(actual_low, low);
    assert_eq!(actual_high, high);
}

#[test]
fn test_boxed_uniform() {
    let low = SVector::from_vec(vec![0.0, 0.0]);
    let high = SVector::from_vec(vec![1.0, 1.0]);
    let boxed = Boxed::<2>::new(low, high).unwrap();

    // Test with seed
    let result = boxed.uniform(Some(42), 0.0, 1.0);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.len(), 2);

    // Test without seed
    let result = boxed.uniform(None, 0.0, 1.0);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.len(), 2);
}
