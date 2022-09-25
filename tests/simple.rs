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

