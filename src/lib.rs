use core::any::Any;
use core::fmt::Debug;
use core::sync::atomic::{AtomicUsize, Ordering};

use colored::Colorize;

pub trait TestFn: 'static + Sized + Send {
    fn run(self, name: String);
}

static SUCCEEDED: AtomicUsize = AtomicUsize::new(0);
static FAILED: AtomicUsize = AtomicUsize::new(0);

impl<F: 'static + FnOnce() -> bool + Sized + Send> TestFn for F {
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

pub struct Expectation<T> {
    val: Box<T>,
    not: bool,
}

impl<T: 'static> Expectation<T> {
    pub fn new(t: T) -> Self {
        Expectation {
            val: Box::new(t),
            not: false
        }
    }

    pub fn to(self) -> Self {
        self
    }

    pub fn equal<U: PartialEq<T>>(self, other: U) -> bool {
        other == *self.val && !self.not
    }

    pub fn be(self) -> Assertion<T> {
        Assertion::new(self)
    }

    pub fn not(self) -> Self {
        Self {
            not: true,
            ..self
        }
    }

    pub fn into_inner(self) -> T {
        *self.val
    }
}

pub struct Assertion<T>{
    val: Box<T>,
    not: bool
}

impl<T: 'static> Assertion<T> {
    pub fn new(ex: Expectation<T>) -> Self {
        let Expectation { val, not } = ex;

        Self {
            val,
            not,
        }
    }

    pub fn empty(self) -> bool 
    where T: IntoIterator {
        let i = self.val.into_iter();

        i.count() == 0
    }

    pub fn some(self) -> bool
    where T: Into<Option<T>> {
        let o: Option<T> = (*self.val).into();

        o.is_some()
    }
    
    pub fn none(self) -> bool
    where T: Into<Option<T>> {
        let o: Option<T> = (*self.val).into();

        o.is_none()
    }

    pub fn a<U: 'static>(self) -> bool {
        let t: Box<dyn Any> = self.val;

        let b = t.is::<U>() && !self.not;

        b
    }

    pub fn an<U: 'static>(self) -> bool {
        let t: Box<dyn Any> = self.val;
        
        t.is::<U>() && !self.not
    }
}

pub fn expect<T: 'static + Debug + PartialEq<T>>(t: T) -> Expectation<T> {
    Expectation::new(t)
}

static DEPTH: AtomicUsize = AtomicUsize::new(0);
const TAB_WIDTH: usize = 2;

pub fn describe(description: impl Into<String>, body: impl FnOnce() + Send + 'static) {
    DEPTH.fetch_add(TAB_WIDTH, Ordering::Relaxed);

    let description = description.into();
    println!("{:>depth$}{description}", " ", depth = DEPTH.load(Ordering::Relaxed));

    let handle = std::thread::spawn(body);

    handle.join().unwrap();
    
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
