use std::{sync::{Arc, Mutex}, fmt::Debug};

use colored::Colorize;
#[macro_use] extern crate lazy_static;

pub mod assertions;

const TAB_WIDTH: usize = 2;
lazy_static! {
    static ref STATE: TestState = TestState::new();
}

struct TestState {
    succeeded: Arc<Mutex<usize>>,
    failed: Arc<Mutex<usize>>,
    depth: Arc<Mutex<usize>>,
}

impl TestState {
    pub fn new() -> Self {
        TestState {
            succeeded: Arc::new(Mutex::new(0)),
            failed: Arc::new(Mutex::new(0)),
            depth: Arc::new(Mutex::new(0))
        }
    }

    pub fn at_root(&self) -> bool {
        *self.depth.lock().unwrap() == 0
    }

    pub fn inc_succeeded(&self) {
        let mut succeeded = self.succeeded.lock().unwrap();
        *succeeded += 1;
    }

    pub fn inc_failed(&self) {
        let mut failed = self.failed.lock().unwrap();
        *failed += 1;
    }
    
    pub fn add_tab(&self) {
        let mut depth = self.depth.lock().unwrap();
        *depth += TAB_WIDTH;
    }
    
    pub fn sub_tab(&self) {
        let mut depth = self.depth.lock().unwrap();
        *depth -= TAB_WIDTH;
    }

    pub fn print_indent(&self) {
        let depth = *self.depth.lock().unwrap();
        print!("{:>depth$}", " ");
    }

    pub fn print(&self) {
        let TestState { succeeded, failed, .. } = self;
        let s = *succeeded.lock().unwrap();
        let f = *failed.lock().unwrap();
        let t = s + f;

        println!();
        print!("{}: {s:<5}", "SUCCEEDED".green().bold());
        print!("{}: {f:<5}", "FAILED".red().bold());
        print!("{}: {t:<5}", "TOTAL".bright_black().bold());
        println!();
        println!();
    }
}

pub enum TestResult {
    Success,
    Failure(String)
}

impl TestResult {
    pub fn and(self, other: TestResult) -> TestResult {
        match self {
            TestResult::Success => other,
            TestResult::Failure(_) => self
        }
    }
}

pub trait TestFn: Sized + Send {
    fn run(self, name: String);
}

impl<F> TestFn for F
where F: Fn() -> TestResult + Sized + Send {
    fn run(self, name: String)  {
        match self() {
            TestResult::Success => {
                println!("{} {}", "✔".green().bold(), name.bright_black());

                STATE.inc_succeeded();
            },
            TestResult::Failure(reason) => {
                println!("{} {}", "✘".red().bold(), name.red().dimmed());
                println!("{reason}");
                
                STATE.inc_failed();
            }
        }
    }
}

pub struct Expectation<'a, T> {
    val: &'a T,
}

impl<'a, T> Expectation<'a, T> 
where T: Debug {
    pub fn new(t: &'a T) -> Self {
        Expectation {
            val: t,
        }
    }

    pub fn to(self) -> Self {
        self
    }

    pub fn equal<U>(self, other: U) -> TestResult 
    where U: PartialEq<T> + Debug {
        if other == *self.val {
            TestResult::Success
        } else {
            TestResult::Failure(
                format!("Expected {other:?}, found {:?}", *self.val)
            )
        }
    }

    pub fn be(self) -> assertions::Equality<'a, T> {
        self.into()
    }
    
    pub fn have(self) -> assertions::Property<'a, T> {
        self.into()
    }

    pub fn when(self) -> ExpectationClause<'a, T> {
        ExpectationClause { val: self.val }
    }
}

pub struct ExpectationClause<'a, T> {
    val: &'a T
}

impl<'a, T> ExpectationClause<'a, Option<T>> 
where T: Debug {
    pub fn unwrapped(self) -> Expectation<'a, T> {
        Expectation::new(self.val.as_ref().unwrap())
    }
}

impl<'a, T, E> ExpectationClause<'a, Result<T, E>> 
where T: Debug,
      E: Debug {
    pub fn unwrapped(self) -> Expectation<'a, T> {
        Expectation::new(self.val.as_ref().unwrap())
    }
    
    pub fn err_unwrapped(self) -> Expectation<'a, E> {
        Expectation::new(self.val.as_ref().unwrap_err())
    }
}

pub fn expect<T>(t: &T) -> Expectation<T> 
where T: Debug {
    Expectation::new(t)
}


pub fn describe(description: impl Into<String>, body: impl Fn() + Send) {
    STATE.add_tab();
    STATE.print_indent();

    let description = description.into();
    println!("{description}");

    body();
    
    STATE.sub_tab();

    if STATE.at_root() {
        STATE.print();
    }
}

pub fn it(description: impl Into<String>, test: impl TestFn) {
    STATE.add_tab();
    STATE.print_indent();

    test.run(description.into());

    STATE.sub_tab();
}
