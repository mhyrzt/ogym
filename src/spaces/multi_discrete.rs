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

        // Check bounds for each value
        for (i, &value) in values.iter().enumerate() {
            if value >= self.nvec[i] {
                return Err(Error::InvalidBounds);
            }
        }

        let mut result = 0u32;
        let mut multiplier = 1u32;

        // Convert from least significant to most significant dimension
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

        // Convert from least significant to most significant dimension
        for i in (0..self.nvec.len()).rev() {
            result[i] = value % self.nvec[i];
            value /= self.nvec[i];
        }

        // If there's still a remainder, the input value was too large
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
