use super::{error::Error, space::Space};
use rand::Rng;

#[derive(Debug)]
pub struct Discrete {
    n: u32,
}

impl Discrete {
    pub fn new(n: u32) -> Result<Self, Error> {
        match n > 1 {
            true => Ok(Discrete { n }),
            false => Err(Error::DiscreteInvalidSize),
        }
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
