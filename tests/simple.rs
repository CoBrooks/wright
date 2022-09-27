use wright::expect;

#[test]
fn equality() {
    expect! { (2 + 2) to equal 4 }

    #[derive(Debug, PartialEq)]
    struct Test { x: u8, y: u8 }

    let a = Test { x: 1, y: 2 };
    let b = Test { x: 3, y: 4 };

    expect! { (a) to not equal b }
}

#[test]
fn result() {
    let x: Result<i32, i32> = Ok(0);
    expect! { (x) to be Ok }
    expect! { (x) to not be Err }

    let y: Result<i32, i32> = Err(1);
    expect! { (y) to be Err }
    expect! { (y) to not be Ok }
}

#[test]
fn option() {
    let x: Option<i32> = Some(0);
    expect! { (x) to be Some }
    expect! { (x) to not be None }

    let y: Option<i32> = None;
    expect! { (y) to be None }
    expect! { (y) to not be Some }
}

#[test]
fn unwrap() {
    let x: Result<i32, &str> = Ok(3);
    expect! { Ok(x) to equal 3 }

    let y: Result<i32, &str> = Err("error message");
    expect! { Err(y) to equal "error message" }

    let z: Option<i32> = Some(3);
    expect! { Some(z) to equal 3 }
}

#[test]
fn boolean() {
    expect! { (true) to be true }
    expect! { (true) to not be false }
}

#[test]
fn empty() {
    let heap_string = String::new();
    expect! { (heap_string) to be empty };

    let stack_string = "";
    expect! { (stack_string) to be empty };

    let vec: Vec<()> = Vec::new();
    expect! { (vec) to be empty };

    use std::collections::HashSet;
    let hs: HashSet<()> = HashSet::new();
    expect! { (hs) to be empty };

    let non_empty_str = "Hello, World!";
    expect! { (non_empty_str) to not be empty };

    let non_empty_vec = vec! [ 3 ];
    expect! { (non_empty_vec) to not be empty };
}

#[test]
fn function() {
    fn add(a: u8, b: u8) -> u8 { a + b }

    expect! { (add(1, 2)) to succeed };
    expect! { (add(1, 2)) to equal 3 };
    
    expect! { (add(255, 255)) to panic };

    let char_a: u8 = b'A';
    expect! { (char_a.is_ascii_digit()) to succeed };
}
