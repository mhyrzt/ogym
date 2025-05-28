use super::space::Space;
use rand::Rng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiscreteSpaceError {
    #[error("size must be greater than one")]
    InvalidSize,
    #[error("the given value is not in bound")]
    NotInBound,
}

#[derive(Debug)]
pub struct DiscreteSpace {
    n: u32,
}

impl DiscreteSpace {
    pub fn new(n: u32) -> Result<Self, DiscreteSpaceError> {
        match n > 1 {
            true => Ok(DiscreteSpace { n }),
            false => Err(DiscreteSpaceError::InvalidSize),
        }
    }
}

impl Space for DiscreteSpace {
    type Item = u32;
    type Error = DiscreteSpaceError;

    fn sample(&self) -> Result<Self::Item, Self::Error> {
        Ok(rand::rng().random_range(0..self.n))
    }

    fn contains(&self, value: &Self::Item) -> Result<(), Self::Error> {
        match *value < self.n {
            true => Ok(()),
            false => Err(DiscreteSpaceError::NotInBound),
        }
    }

    fn shape(&self) -> Vec<usize> {
        vec![self.n as usize]
    }

    fn bounds(&self) -> (Self::Item, Self::Item) {
        (0, self.n - 1)
    }
}
