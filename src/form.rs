pub trait Form {
    type Output;

    fn and<F>(self, then: F) -> Both<Self, F>
    where
        Self: Sized,
    {
        Both { first: self, then }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Select<T> {
    options: Vec<T>,
}

impl<T> Select<T> {
    pub fn from(options: Vec<T>) -> Self {
        Select { options }
    }
}

impl<T> Form for Select<T> {
    type Output = T;
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Both<F, G> {
    first: F,
    then: G,
}

impl<F: Form, G: Form> Form for Both<F, G> {
    type Output = (F::Output, G::Output);
}
