use core::any::Any;

use super::*;

pub struct Equality<'a, T> {
    val: &'a T,
}

impl<'a, T> From<Expectation<'a, T>> for Equality<'a, T> {
    fn from(ex: Expectation<'a, T>) -> Self {
        let Expectation { val } = ex;

        Self { val }
    }
}

impl<'a, T> Equality<'a, Option<T>> {
    pub fn some(self) -> bool {
        self.val.is_some()
    }
    
    pub fn none(self) -> bool {
        self.val.is_none()
    }
}

impl<'a, T, E> Equality<'a, Result<T, E>> {
    pub fn ok(self) -> bool {
        self.val.is_ok()
    }
    
    pub fn err(self) -> bool {
        self.val.is_err()
    }
}

impl<'a, T> Equality<'a, T> {
    pub fn empty<U>(self) -> bool 
    where T: AsRef<[U]> {
        self.val.as_ref().is_empty()
    }

    pub fn a<U: 'static>(self) -> bool
    where T: Any {
        let t: &'a dyn Any = self.val;

        let b = t.is::<U>();

        b
    }

    pub fn an<U: 'static>(self) -> bool 
    where T: Any {
        let t: &dyn Any = self.val;
        
        t.is::<U>()
    }
}
