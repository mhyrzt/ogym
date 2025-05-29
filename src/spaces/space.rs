use super::error::Error;

pub trait Space {
    type Item;

    fn sample(&self) -> Result<Self::Item, Error>;
    fn contains(&self, value: &Self::Item) -> Result<(), Error>;
    fn shape(&self) -> Vec<usize>;
    fn bounds(&self) -> (Self::Item, Self::Item);
}

#[derive(Debug, Clone)]
pub struct EnvSpace<S: Space, A: Space> {
    pub state: S,
    pub action: A,
}
