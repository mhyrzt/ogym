use nalgebra::SVector;

use super::{boxed::Boxed, discrete::Discrete, error::Error, space::Space};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MixedItem<const D: usize> {
    Discrete(u32),
    Continuous(SVector<f64, D>),
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn is_discrete(&self) -> bool {
        matches!(self, Self::Discrete(_))
    }

    pub fn is_continuous(&self) -> bool {
        matches!(self, Self::Continuous(_))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::error::Error;
    use crate::spaces::space::Space;
    use nalgebra::SVector;

    #[test]
    fn test_discrete_construction_and_sample() {
        let space: Mixed<2> = Mixed::discrete(5).expect("Failed to create discrete mixed");

        if let Mixed::Discrete(d) = &space {
            assert_eq!(d.shape(), vec![5]);
        } else {
            panic!("Expected Mixed::Discrete variant");
        }

        let sample = space.sample().unwrap();
        match sample {
            MixedItem::Discrete(val) => assert!(val < 5),
            _ => panic!("Expected MixedItem::Discrete sample"),
        }
    }

    #[test]
    fn test_continuous_construction_and_sample() {
        let low = SVector::<f64, 2>::from([0.0, 0.0]);
        let high = SVector::<f64, 2>::from([1.0, 1.0]);
        let space = Mixed::continuous(low, high).expect("Failed to create continuous mixed");

        if let Mixed::Continuous(b) = &space {
            assert_eq!(b.shape(), vec![2]);
        } else {
            panic!("Expected Mixed::Continuous variant");
        }

        let sample = space.sample().unwrap();
        match sample {
            MixedItem::Continuous(val) => {
                assert!(val[0] >= 0.0 && val[0] <= 1.0);
                assert!(val[1] >= 0.0 && val[1] <= 1.0);
            }
            _ => panic!("Expected MixedItem::Continuous sample"),
        }
    }

    #[test]
    fn test_contains_type_mismatch() {
        let discrete_space: Mixed<2> = Mixed::discrete(5).unwrap();
        let continuous_item = MixedItem::Continuous(SVector::<f64, 2>::from([0.5, 0.5]));

        assert_eq!(
            discrete_space.contains(&continuous_item),
            Err(Error::TypeMismatch)
        );

        let low = SVector::<f64, 2>::from([0.0, 0.0]);
        let high = SVector::<f64, 2>::from([1.0, 1.0]);
        let continuous_space = Mixed::continuous(low, high).unwrap();
        let discrete_item = MixedItem::Discrete(2);

        assert_eq!(
            continuous_space.contains(&discrete_item),
            Err(Error::TypeMismatch)
        );
    }

    #[test]
    fn test_contains_valid_types_check_bounds() {
        let d_space: Mixed<1> = Mixed::discrete(3).unwrap();
        assert!(d_space.contains(&MixedItem::Discrete(2)).is_ok());
        assert_eq!(
            d_space.contains(&MixedItem::Discrete(5)),
            Err(Error::InvalidBounds)
        );

        let low = SVector::<f64, 1>::from([0.0]);
        let high = SVector::<f64, 1>::from([10.0]);
        let c_space = Mixed::continuous(low, high).unwrap();

        assert!(c_space
            .contains(&MixedItem::Continuous(SVector::from([5.0])))
            .is_ok());
        assert_eq!(
            c_space.contains(&MixedItem::Continuous(SVector::from([15.0]))),
            Err(Error::InvalidBounds)
        );
    }

    #[test]
    fn test_mixed_bounds() {
        let d_space: Mixed<2> = Mixed::discrete(5).unwrap();
        let (d_low, d_high) = d_space.bounds();
        assert_eq!(d_low, MixedItem::Discrete(0));
        assert_eq!(d_high, MixedItem::Discrete(4));

        let low = SVector::<f64, 2>::from([-1.0, -1.0]);
        let high = SVector::<f64, 2>::from([1.0, 1.0]);
        let c_space = Mixed::continuous(low, high).unwrap();
        let (c_low, c_high) = c_space.bounds();

        assert_eq!(c_low, MixedItem::Continuous(low));
        assert_eq!(c_high, MixedItem::Continuous(high));
    }

    #[test]
    fn test_shape_delegation() {
        let d_space: Mixed<10> = Mixed::discrete(5).unwrap();
        assert_eq!(d_space.shape(), vec![5]);

        let c_space =
            Mixed::continuous(SVector::<f64, 3>::zeros(), SVector::<f64, 3>::zeros()).unwrap();
        assert_eq!(c_space.shape(), vec![3]);
    }
}
