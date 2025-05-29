use nalgebra::SVector;

use super::{boxed::Boxed, discrete::Discrete, error::Error, space::Space};

#[derive(Debug)]
pub enum MixedItem<const D: usize> {
    Discrete(u32),
    Continuous(SVector<f64, D>),
}

#[derive(Debug)]
pub enum Mixed<const D: usize> {
    Discrete(Discrete),
    Continuous(Boxed<D>),
}

impl<const D: usize> Mixed<D> {
    pub fn discrete(n: u32) -> Result<Self, Error> {
        Ok(Self::Discrete(Discrete::new(n)?))
    }

    pub fn continuous(low: SVector<f64, D>, high: SVector<f64, D>) -> Result<Self, Error> {
        Ok(Self::Continuous(Boxed::new(low, high)?))
    }
}

impl<const D: usize> Space for Mixed<D> {
    type Item = MixedItem<D>;

    fn sample(&self) -> Result<Self::Item, Error> {
        match self {
            Self::Discrete(space) => space.sample().map(MixedItem::Discrete),
            Self::Continuous(space) => space.sample().map(MixedItem::Continuous),
        }
    }

    fn contains(&self, value: &Self::Item) -> Result<(), Error> {
        match (self, value) {
            (Self::Discrete(space), MixedItem::Discrete(val)) => space.contains(val),
            (Self::Continuous(space), MixedItem::Continuous(val)) => space.contains(val),
            _ => Err(Error::TypeMismatch),
        }
    }

    fn shape(&self) -> Vec<usize> {
        match self {
            Self::Discrete(space) => space.shape(),
            Self::Continuous(space) => space.shape(),
        }
    }

    fn bounds(&self) -> (Self::Item, Self::Item) {
        match self {
            Self::Discrete(space) => {
                let (low, high) = space.bounds();
                (MixedItem::Discrete(low), MixedItem::Discrete(high))
            }
            Self::Continuous(space) => {
                let (low, high) = space.bounds();
                (MixedItem::Continuous(low), MixedItem::Continuous(high))
            }
        }
    }
}
