use super::{error::Error, space::Space};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Discrete {
    n: u32,
}

impl Discrete {
    pub fn new(n: u32) -> Result<Self, Error> {
        match n > 1 {
            true => Ok(Self { n }),
            false => Err(Error::DiscreteInvalidSize),
        }
    }

    pub fn size(&self) -> u32 {
        self.n
    }
}

impl Space for Discrete {
    type Item = u32;

    fn sample(&self) -> Result<Self::Item, Error> {
        Ok(rand::rng().random_range(0..self.n))
    }

    fn contains(&self, value: &Self::Item) -> Result<(), Error> {
        match *value < self.n {
            true => Ok(()),
            false => Err(Error::InvalidBounds),
        }
    }

    fn shape(&self) -> Vec<usize> {
        vec![self.n as usize]
    }

    fn bounds(&self) -> (Self::Item, Self::Item) {
        (0, self.n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::error::Error;
    use crate::spaces::Space;

    #[test]
    fn test_new_valid_size() {
        let d = Discrete::new(2);
        assert!(d.is_ok());
        let d = d.unwrap();
        assert_eq!(d.shape(), vec![2]);

        let d = Discrete::new(100);
        assert!(d.is_ok());
    }

    #[test]
    fn test_new_invalid_size() {
        let d = Discrete::new(1);
        assert_eq!(d, Err(Error::DiscreteInvalidSize));

        let d = Discrete::new(0);
        assert_eq!(d, Err(Error::DiscreteInvalidSize));
    }

    #[test]
    fn test_sample_range() {
        let size = 10;
        let d = Discrete::new(size).unwrap();

        for _ in 0..100 {
            let sample = d.sample().unwrap();
            assert!(sample < size);
            assert!(d.contains(&sample).is_ok());
        }
    }

    #[test]
    fn test_contains() {
        let size = 5;
        let d = Discrete::new(size).unwrap();

        assert!(d.contains(&0).is_ok());
        assert!(d.contains(&4).is_ok());

        assert_eq!(d.contains(&5), Err(Error::InvalidBounds));
        assert_eq!(d.contains(&100), Err(Error::InvalidBounds));
    }

    #[test]
    fn test_shape() {
        let d = Discrete::new(10).unwrap();
        assert_eq!(d.shape(), vec![10]);
    }

    #[test]
    fn test_bounds() {
        let d = Discrete::new(5).unwrap();
        assert_eq!(d.bounds(), (0, 4));

        let d = Discrete::new(2).unwrap();
        assert_eq!(d.bounds(), (0, 1));
    }

    #[test]
    fn test_debug_clone_partial_eq() {
        let d1 = Discrete::new(5).unwrap();
        let d2 = Discrete::new(5).unwrap();
        let d3 = Discrete::new(6).unwrap();

        assert_eq!(d1, d2);
        assert_ne!(d1, d3);

        let d1_clone = d1.clone();
        assert_eq!(d1, d1_clone);

        let debug_str = format!("{:?}", d1);
        assert!(debug_str.contains("Discrete"));
        assert!(debug_str.contains("n: 5"));
    }
}
