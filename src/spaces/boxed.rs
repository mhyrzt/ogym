use super::{error::Error, space::Space};
use nalgebra::SVector;
use rand::{
    SeedableRng,
    distr::{Distribution, Uniform},
    rngs::StdRng,
};

#[derive(Debug, Clone)]
pub struct Boxed<const D: usize> {
    pub low: SVector<f64, D>,
    pub high: SVector<f64, D>,
}

impl<const D: usize> Boxed<D> {
    pub fn new(low: SVector<f64, D>, high: SVector<f64, D>) -> Result<Self, Error> {
        if low.len() != high.len() {
            return Err(Error::ShapeMismatch);
        }

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
        // Assuming low and high are scalar tensors (single value)
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
