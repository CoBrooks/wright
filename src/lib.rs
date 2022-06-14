use core::sync::atomic::{AtomicUsize, Ordering};

use colored::Colorize;

pub mod assertions;

pub trait TestFn: Sized + Send {
    fn run(self, name: String);
}

static SUCCEEDED: AtomicUsize = AtomicUsize::new(0);
static FAILED: AtomicUsize = AtomicUsize::new(0);

impl<F> TestFn for F
where F: Fn() -> bool + Sized + Send {
    fn run(self, name: String)  {
        if self() {
            println!("{} {}", "✔".green().bold(), name.bright_black());

            SUCCEEDED.fetch_add(1, Ordering::Relaxed);
        } else {
            println!("{} {}", "✘".red().bold(), name.red().dimmed());
            
            FAILED.fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub struct Expectation<'a, T> {
    val: &'a T,
}

impl<'a, T> Expectation<'a, T> {
    pub fn new(t: &'a T) -> Self {
        Expectation {
            val: t,
        }
    }

    pub fn to(self) -> Self {
        self
    }

    pub fn equal<U: PartialEq<T>>(self, other: U) -> bool {
        other == *self.val
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

impl<'a, T> ExpectationClause<'a, Option<T>> {
    pub fn unwrapped(self) -> Expectation<'a, T> {
        Expectation::new(self.val.as_ref().unwrap())
    }
}

impl<'a, T, E: std::fmt::Debug> ExpectationClause<'a, Result<T, E>> {
    pub fn unwrapped(self) -> Expectation<'a, T> {
        Expectation::new(self.val.as_ref().unwrap())
    }
}

pub fn expect<'a, T>(t: &'a T) -> Expectation<'a, T> {
    Expectation::new(t)
}

static DEPTH: AtomicUsize = AtomicUsize::new(0);
const TAB_WIDTH: usize = 2;

pub fn describe(description: impl Into<String>, body: impl Fn() + Send) {
    DEPTH.fetch_add(TAB_WIDTH, Ordering::Relaxed);

    let description = description.into();
    println!("{:>depth$}{description}", " ", depth = DEPTH.load(Ordering::Relaxed));

    body();
    
    DEPTH.fetch_sub(TAB_WIDTH, Ordering::Relaxed);

    if DEPTH.load(Ordering::Relaxed) == 0 {
        let s = SUCCEEDED.load(Ordering::Relaxed);
        let f = FAILED.load(Ordering::Relaxed);

        println!();
        print!("{}: {s:<5}", "SUCCEEDED".green().bold());
        print!("{}: {f:<5}", "FAILED".red().bold());
        print!("{}: {}\n", "TOTAL".bright_black(), s + f);
        println!();
    }
}

pub fn it(description: impl Into<String>, test: impl TestFn) {
    DEPTH.fetch_add(TAB_WIDTH, Ordering::Relaxed);

    print!("{:>depth$}", " ", depth = DEPTH.load(Ordering::Relaxed));
    test.run(description.into());

    DEPTH.fetch_sub(TAB_WIDTH, Ordering::Relaxed);
}
