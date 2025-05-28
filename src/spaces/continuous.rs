use super::space::Space;
use nalgebra::DVector;
use rand::distr::{Distribution, Uniform, uniform};

#[derive(Debug)]
pub struct ContinuousSpace {
    pub low: DVector<f64>,
    pub high: DVector<f64>,
}

#[derive(Debug, thiserror::Error)]
pub enum ContinuousSpaceError {
    #[error("Low and high tensors must have the same shape.")]
    ShapeMismatch,

    #[error("All elements of `low` must be less than or equal to `high`.")]
    InvalidBounds,

    #[error("Distribution Error: {0}")]
    DistError(#[from] uniform::Error),
}

impl ContinuousSpace {
    pub fn new(low: DVector<f64>, high: DVector<f64>) -> Result<Self, ContinuousSpaceError> {
        if low.len() != high.len() {
            return Err(ContinuousSpaceError::ShapeMismatch);
        }

        if !low.iter().zip(high.iter()).all(|(&l, &h)| l <= h) {
            return Err(ContinuousSpaceError::InvalidBounds);
        }

        Ok(Self { low, high })
    }
}

impl Space for ContinuousSpace {
    type Item = DVector<f64>;
    type Error = ContinuousSpaceError;

    fn sample(&self) -> Result<Self::Item, Self::Error> {
        // Assuming low and high are scalar tensors (single value)
        let mut rng = rand::rng();
        let sampled: Result<Vec<f64>, ContinuousSpaceError> = self
            .low
            .iter()
            .zip(self.high.iter())
            .map(|(&l, &h)| Ok(Uniform::new_inclusive(l, h)?.sample(&mut rng)))
            .collect();

        Ok(DVector::from_vec(sampled?))
    }

    fn contains(&self, value: &Self::Item) -> Result<(), Self::Error> {
        if value.len() != self.low.len() {
            return Err(ContinuousSpaceError::ShapeMismatch);
        }

        for ((&v, &l), &h) in value.iter().zip(&self.low).zip(&self.high) {
            if v < l || v > h {
                return Err(ContinuousSpaceError::InvalidBounds);
            }
        }
        Ok(())
    }

    fn shape(&self) -> Vec<usize> {
        vec![self.low.len()]
    }

    fn bounds(&self) -> (Self::Item, Self::Item) {
        (self.low.clone(), self.high.clone())
    }
}
