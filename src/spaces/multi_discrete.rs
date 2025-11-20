use super::{error::Error, space::Space};
use rand::Rng;

#[derive(Debug)]
pub struct MultiDiscrete {
    nvec: Vec<u32>,
}

impl MultiDiscrete {
    pub fn new(nvec: Vec<u32>) -> Result<Self, Error> {
        if nvec.is_empty() {
            return Err(Error::EmptyVec);
        }

        for &n in &nvec {
            if n <= 1 {
                return Err(Error::DiscreteInvalidSize);
            }
        }

        Ok(MultiDiscrete { nvec })
    }

    pub fn to_u32(&self, values: &[u32]) -> Result<u32, Error> {
        if values.len() != self.nvec.len() {
            return Err(Error::ShapeMismatch);
        }

        for (i, &value) in values.iter().enumerate() {
            if value >= self.nvec[i] {
                return Err(Error::InvalidBounds);
            }
        }

        let mut result = 0u32;
        let mut multiplier = 1u32;

        for i in (0..values.len()).rev() {
            result = result
                .checked_add(
                    values[i]
                        .checked_mul(multiplier)
                        .ok_or(Error::InvalidBounds)?,
                )
                .ok_or(Error::InvalidBounds)?;

            multiplier = multiplier
                .checked_mul(self.nvec[i])
                .ok_or(Error::InvalidBounds)?;
        }

        Ok(result)
    }

    pub fn u32_to_multi_discrete(&self, mut value: u32) -> Result<Vec<u32>, Error> {
        let mut result = vec![0u32; self.nvec.len()];

        for i in (0..self.nvec.len()).rev() {
            result[i] = value % self.nvec[i];
            value /= self.nvec[i];
        }

        if value > 0 {
            return Err(Error::InvalidBounds);
        }

        Ok(result)
    }

    pub fn total_combinations(&self) -> Result<u32, Error> {
        let mut total = 1u32;
        for &n in &self.nvec {
            total = total.checked_mul(n).ok_or(Error::InvalidBounds)?;
        }
        Ok(total)
    }
}

impl Space for MultiDiscrete {
    type Item = Vec<u32>;

    fn sample(&self) -> Result<Self::Item, Error> {
        let mut rng = rand::rng();
        let mut result = Vec::with_capacity(self.nvec.len());

        for &n in &self.nvec {
            result.push(rng.random_range(0..n));
        }

        Ok(result)
    }

    fn contains(&self, value: &Self::Item) -> Result<(), Error> {
        if value.len() != self.nvec.len() {
            return Err(Error::ShapeMismatch);
        }

        for (i, &val) in value.iter().enumerate() {
            if val >= self.nvec[i] {
                return Err(Error::InvalidBounds);
            }
        }

        Ok(())
    }

    fn shape(&self) -> Vec<usize> {
        self.nvec.iter().map(|&n| n as usize).collect()
    }

    fn bounds(&self) -> (Self::Item, Self::Item) {
        let lower = vec![0; self.nvec.len()];
        let upper = self.nvec.iter().map(|&n| n - 1).collect();
        (lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spaces::error::Error;
    use crate::spaces::space::Space;

    #[test]
    fn test_new_valid() {
        let md = MultiDiscrete::new(vec![2, 3, 4]);
        assert!(md.is_ok());
    }

    #[test]
    fn test_new_invalid_empty() {
        let md = MultiDiscrete::new(vec![]);
        assert_eq!(md.err(), Some(Error::EmptyVec));
    }

    #[test]
    fn test_new_invalid_dimension_size() {
        let md = MultiDiscrete::new(vec![2, 1, 3]);
        assert_eq!(md.err(), Some(Error::DiscreteInvalidSize));

        let md = MultiDiscrete::new(vec![0, 3]);
        assert_eq!(md.err(), Some(Error::DiscreteInvalidSize));
    }

    #[test]
    fn test_shape_and_bounds() {
        let md = MultiDiscrete::new(vec![2, 3]).unwrap();

        assert_eq!(md.shape(), vec![2, 3]);

        let (low, high) = md.bounds();
        assert_eq!(low, vec![0, 0]);
        assert_eq!(high, vec![1, 2]);
    }

    #[test]
    fn test_contains() {
        let md = MultiDiscrete::new(vec![3, 3]).unwrap();

        assert!(md.contains(&vec![0, 0]).is_ok());
        assert!(md.contains(&vec![2, 2]).is_ok());

        assert_eq!(md.contains(&vec![3, 0]), Err(Error::InvalidBounds));
        assert_eq!(md.contains(&vec![0, 4]), Err(Error::InvalidBounds));

        assert_eq!(md.contains(&vec![0]), Err(Error::ShapeMismatch));
        assert_eq!(md.contains(&vec![0, 0, 0]), Err(Error::ShapeMismatch));
    }

    #[test]
    fn test_sample() {
        let dims = vec![5, 10];
        let md = MultiDiscrete::new(dims.clone()).unwrap();

        for _ in 0..50 {
            let sample = md.sample().unwrap();
            assert_eq!(sample.len(), 2);
            assert!(sample[0] < 5);
            assert!(sample[1] < 10);
            assert!(md.contains(&sample).is_ok());
        }
    }

    #[test]
    fn test_conversion_logic_simple() {
        // Dimensions: [2, 2]
        // Combinations:
        // [0, 0] -> 0
        // [0, 1] -> 1
        // [1, 0] -> 2
        // [1, 1] -> 3
        let md = MultiDiscrete::new(vec![2, 2]).unwrap();

        assert_eq!(md.to_u32(&[0, 0]).unwrap(), 0);
        assert_eq!(md.to_u32(&[0, 1]).unwrap(), 1);
        assert_eq!(md.to_u32(&[1, 0]).unwrap(), 2);
        assert_eq!(md.to_u32(&[1, 1]).unwrap(), 3);

        assert_eq!(md.u32_to_multi_discrete(0).unwrap(), vec![0, 0]);
        assert_eq!(md.u32_to_multi_discrete(3).unwrap(), vec![1, 1]);
    }

    #[test]
    fn test_conversion_round_trip() {
        let md = MultiDiscrete::new(vec![3, 5, 2]).unwrap();
        let total = md.total_combinations().unwrap();

        for i in 0..total {
            let vec_rep = md.u32_to_multi_discrete(i).unwrap();
            let scalar_rep = md.to_u32(&vec_rep).unwrap();
            assert_eq!(i, scalar_rep);
            assert!(md.contains(&vec_rep).is_ok());
        }
    }

    #[test]
    fn test_to_u32_errors() {
        let md = MultiDiscrete::new(vec![3, 3]).unwrap();
        assert_eq!(md.to_u32(&[1]), Err(Error::ShapeMismatch));
        assert_eq!(md.to_u32(&[3, 0]), Err(Error::InvalidBounds));
    }

    #[test]
    fn test_u32_to_multi_discrete_errors() {
        let md = MultiDiscrete::new(vec![2, 2]).unwrap();
        assert_eq!(md.u32_to_multi_discrete(4), Err(Error::InvalidBounds));
    }

    #[test]
    fn test_total_combinations() {
        let md = MultiDiscrete::new(vec![2, 4, 5]).unwrap();
        assert_eq!(md.total_combinations().unwrap(), 40);
    }

    #[test]
    fn test_overflow_protection() {
        let safe_md = MultiDiscrete::new(vec![u32::MAX / 2, 2]).unwrap();
        assert!(safe_md.total_combinations().is_ok());

        let overflow_md = MultiDiscrete::new(vec![u32::MAX, 2]).unwrap();
        assert_eq!(overflow_md.total_combinations(), Err(Error::InvalidBounds));

        assert_eq!(
            overflow_md.to_u32(&[u32::MAX, 1]),
            Err(Error::InvalidBounds)
        );
    }
}
