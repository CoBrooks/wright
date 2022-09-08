use core::any::Any;
use core::fmt::Debug;
use std::any::type_name;

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

impl<'a, T> Equality<'a, Option<T>>
where T: Debug {
    pub fn some(self) -> TestResult {
        if self.val.is_some() {
            TestResult::Success
        } else {
            TestResult::Failure("Expected Some(...), but value was None".to_string())
        }
    }
    
    pub fn none(self) -> TestResult {
        if self.val.is_none() {
            TestResult::Success
        } else {
            TestResult::Failure("Expected None, but value was Some(...)".to_string())
        }
    }
}

impl<'a, T, E> Equality<'a, Result<T, E>> 
where T: Debug,
      E: Debug {
    pub fn ok(self) -> TestResult {
        match self.val {
            Ok(_) => TestResult::Success,
            Err(_) => TestResult::Failure(
                format!("Expected value to be Ok(...), but it returned {:?}", self.val)
            )
        }
    }
    
    pub fn err(self) -> TestResult {
        match self.val {
            Ok(val) => TestResult::Failure(
                format!("Expected value to be Err(...), but it returned Ok({val:?})")
            ),
            Err(_) => TestResult::Success,
        }
    }
}

impl<'a, T> Equality<'a, T> 
where T: Debug {
    pub fn empty<U>(self) -> TestResult
    where T: AsRef<[U]> {
        if self.val.as_ref().is_empty() {
            TestResult::Success
        } else {
            TestResult::Failure(
                format!("Expected value to be empty, found {:?}", self.val)
            )
        }
    }

    pub fn a<U: 'static>(self) -> TestResult
    where T: Any {
        let t: &'a dyn Any = self.val;

        if t.is::<U>() {
            TestResult::Success
        } else {
            TestResult::Failure(
                format!("Expected value to be {}, was {}", type_name::<T>(), type_name::<U>())
            )
        }
    }

    pub fn an<U: 'static>(self) -> TestResult
    where T: Any {
        self.a::<U>()
    }
}
