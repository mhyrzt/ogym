use super::{error::Error, space::Space};
use nalgebra::SVector;
use rand::{
    distr::{Distribution, Uniform},
    rngs::StdRng,
    SeedableRng,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Boxed<const D: usize> {
    pub low: SVector<f64, D>,
    pub high: SVector<f64, D>,
}

impl<const D: usize> Boxed<D> {
    pub fn new(low: SVector<f64, D>, high: SVector<f64, D>) -> Result<Self, Error> {
        if !low.iter().zip(high.iter()).all(|(&l, &h)| l <= h) {
            return Err(Error::InvalidBounds);
        }
        Ok(Self { low, high })
    }

    pub fn uniform(
        &self,
        seed: Option<u64>,
        low: f64,
        high: f64,
    ) -> Result<SVector<f64, D>, Error> {
        let mut rng = match seed {
            Some(state) => StdRng::seed_from_u64(state),
            None => StdRng::from_rng(&mut rand::rng()),
        };
        let dist = Uniform::new(low, high)?;
        Ok(SVector::from_fn(|_, _| dist.sample(&mut rng)))
    }
}

impl<const D: usize> Space for Boxed<D> {
    type Item = SVector<f64, D>;

    fn sample(&self) -> Result<Self::Item, Error> {
        let mut rng = rand::rng();
        let sampled: Result<Vec<f64>, Error> = self
            .low
            .iter()
            .zip(self.high.iter())
            .map(|(&l, &h)| Ok(Uniform::new_inclusive(l, h)?.sample(&mut rng)))
            .collect();

        Ok(SVector::from_vec(sampled?))
    }

    fn contains(&self, value: &Self::Item) -> Result<(), Error> {
        if value.len() != self.low.len() {
            return Err(Error::ShapeMismatch);
        }

        for ((&v, &l), &h) in value.iter().zip(&self.low).zip(&self.high) {
            if v < l || v > h {
                return Err(Error::InvalidBounds);
            }
        }
        Ok(())
    }

    fn shape(&self) -> Vec<usize> {
        vec![self.low.len()]
    }

    fn bounds(&self) -> (Self::Item, Self::Item) {
        (self.low, self.high)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::error::Error;
    use crate::spaces::Space;
    use nalgebra::SVector;

    fn vec2(x: f64, y: f64) -> SVector<f64, 2> {
        SVector::from([x, y])
    }

    #[test]
    fn test_new_valid_bounds() {
        let low = vec2(0.0, -1.0);
        let high = vec2(1.0, 1.0);
        let b = Boxed::new(low, high);
        assert!(b.is_ok());
        let b = b.unwrap();
        assert_eq!(b.low, low);
        assert_eq!(b.high, high);
    }

    #[test]
    fn test_new_invalid_bounds() {
        let low = vec2(0.0, 2.0);
        let high = vec2(1.0, 1.0);
        let b = Boxed::new(low, high);
        assert_eq!(b, Err(Error::InvalidBounds));
    }

    #[test]
    fn test_new_equal_bounds() {
        let low = vec2(1.0, 1.0);
        let high = vec2(1.0, 1.0);
        let b = Boxed::new(low, high);
        assert!(b.is_ok());
    }

    #[test]
    fn test_sample() {
        let low = vec2(0.0, 10.0);
        let high = vec2(1.0, 20.0);
        let b = Boxed::new(low, high).unwrap();

        for _ in 0..100 {
            let sample = b.sample().unwrap();
            assert_eq!(sample.len(), 2);

            assert!(sample[0] >= 0.0);
            assert!(sample[0] <= 1.0);

            assert!(sample[1] >= 10.0);
            assert!(sample[1] <= 20.0);
        }
    }

    #[test]
    fn test_contains() {
        let low = vec2(-1.0, -1.0);
        let high = vec2(1.0, 1.0);
        let b = Boxed::new(low, high).unwrap();

        assert!(b.contains(&vec2(0.0, 0.0)).is_ok());
        assert!(b.contains(&vec2(-1.0, 1.0)).is_ok());

        assert_eq!(b.contains(&vec2(1.1, 0.0)), Err(Error::InvalidBounds));
        assert_eq!(b.contains(&vec2(0.0, -1.1)), Err(Error::InvalidBounds));
    }

    #[test]
    fn test_shape() {
        let low = vec2(0.0, 0.0);
        let high = vec2(1.0, 1.0);
        let b = Boxed::new(low, high).unwrap();
        assert_eq!(b.shape(), vec![2]);
    }

    #[test]
    fn test_bounds_trait() {
        let low = vec2(-5.0, 0.0);
        let high = vec2(5.0, 10.0);
        let b = Boxed::new(low, high).unwrap();
        let (l, h) = b.bounds();
        assert_eq!(l, low);
        assert_eq!(h, high);
    }

    #[test]
    fn test_uniform_helper_seeded() {
        let b = Boxed::new(vec2(0.0, 0.0), vec2(1.0, 1.0)).unwrap();

        let seed = Some(42);
        let val1 = b.uniform(seed, 0.0, 10.0).unwrap();
        let val2 = b.uniform(seed, 0.0, 10.0).unwrap();
        assert_eq!(val1, val2);

        let val3 = b.uniform(Some(43), 0.0, 10.0).unwrap();
        assert_ne!(val1, val3);

        assert!(val1[0] >= 0.0 && val1[0] < 10.0);
        assert!(val1[1] >= 0.0 && val1[1] < 10.0);
    }

    #[test]
    fn test_uniform_helper_invalid_range() {
        let b = Boxed::new(vec2(0.0, 0.0), vec2(1.0, 1.0)).unwrap();
        let res = b.uniform(None, 10.0, 0.0);

        match res {
            Err(Error::Distribution(_)) => {}
            _ => panic!("Expected Distribution error for invalid uniform range"),
        }
    }

    #[test]
    fn test_debug_clone_partial_eq() {
        let low = vec2(0.0, 0.0);
        let high = vec2(1.0, 1.0);
        let b1 = Boxed::new(low, high).unwrap();
        let b2 = b1.clone();

        assert_eq!(b1, b2);

        let debug_str = format!("{:?}", b1);
        assert!(debug_str.contains("Boxed"));
        assert!(debug_str.contains("low"));
        assert!(debug_str.contains("high"));
    }
}
