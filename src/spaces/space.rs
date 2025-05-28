pub trait Space {
    type Item;
    type Error;

    fn sample(&self) -> Result<Self::Item, Self::Error>;
    fn contains(&self, value: &Self::Item) -> Result<(), Self::Error>;
    fn shape(&self) -> Vec<usize>;
    fn bounds(&self) -> (Self::Item, Self::Item);
}

#[derive(Debug)]
pub struct EnvSpace<S: Space, A: Space> {
    pub state: S,
    pub action: A,
}
