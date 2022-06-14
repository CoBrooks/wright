# Wright

A Rust Behavior-Driven-Development testing framework built to mimic NodeJS' 
chai and mocha workflow.

***NB***: This project is a work-in-progress and is being developed alongside
my [Rust RDF Crate](https://github.com/CoBrooks/rdf-rs) as features are needed.

## Usage

### Cargo.toml

```toml
[[test]]
name = "wright"
path = "src/test.rs"
harness = false

[dev-dependencies]
wright = { "git" = "https://github.com/CoBrooks/wright" }
```

### src/test.rs

```rust
use wright::*;

fn main() {
    describe("String", || {
        describe("::from", || {
            let slice = "Hello, World!";
            let string = String::from(slice);

            it("should convert &str -> String", || {
                expect(&slice).to().be().a::<&str>()
                    && expect(&string).to().be().a::<String>()
            });
        });
        
        describe("::from_utf8", || {
            it("should return Ok with valid utf-8", || {
                let valid = vec![240, 159, 146, 150];
                let s = String::from_utf8(valid);

                expect(&s).to().be().ok() 
            });
            
            it("should return Err with invalid utf-8", || {
                let invalid = vec![0, 159, 146, 150];
                let s = String::from_utf8(invalid);

                expect(&s).to().be().err()
            });
            
            it("should be of correct length", || {
                let valid = vec![240, 159, 146, 150];
                let s = String::from_utf8(valid).unwrap();

                expect(&s).to().have().length(4)
            });
        });

        describe("::new", || {
            let s = String::new();

            it("should be empty", || {
                expect(&s).to().be().empty()
            });
        });

        describe(".pop", || {
            let mut s = String::from("A");

            let c = s.pop();
            it("should return the last char", || {
                expect(&c).to().be().some()
                    && expect(&c).when().unwrapped().to().equal('A')
            });
            
            let c = s.pop();
            it("should return None if the String is empty", || {
                expect(&c).to().be().none()
            });
        });
    });
}
```

## Running the Tests

```
$ cargo test --test wright
  String
    ::from
      ✔ should convert &str -> String
    ::from_utf8
      ✔ should return Ok with valid utf-8
      ✔ should return Err with valid utf-8
      ✔ should be of correct length
    ::new
      ✔ should be empty
    .pop
      ✔ should return the last char
      ✔ should return None if the String is empty

SUCCEEDED: 7    FAILED: 0    TOTAL: 7
```
