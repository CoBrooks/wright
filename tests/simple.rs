use wright::expect;

#[test]
fn equality() {
    expect! { (2 + 2) to equal 4 }
    expect! { (2 + 2) to not equal 5 }
}

#[test]
fn result() {
    let foobar: Result<i32, i32> = Ok(2);

    expect! { (foobar) to be ok }
    expect! { (foobar) to not be err }
    
    expect! { (foobar.unwrap()) to equal 2 }
}
