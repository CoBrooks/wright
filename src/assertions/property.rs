use super::*;

pub struct Property<'a, T> {
    val: &'a T,
}

impl<'a, T> From<Expectation<'a, T>> for Property<'a, T> {
    fn from(ex: Expectation<'a, T>) -> Self {
        let Expectation { val } = ex;

        Self { val }
    }
}

impl<'a, T> Property<'a, T> {
    pub fn length<U>(self, len: usize) -> bool 
    where T: AsRef<[U]> {
        self.val.as_ref().len() == len
    }
}
