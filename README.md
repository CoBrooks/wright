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

        describe("::new", || {
            let s = String::new();

            it("should be empty", || {
                expect(&s.len()).to().equal(0)
            });
        });

        describe(".pop", || {
            let mut s = String::from("A");

            let c = s.pop();
            it("should return the last char", || {
                expect(&c).to().be().some()
                    && expect(&c.unwrap()).to().equal('A')
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
    ::new
      ✔ should be empty
    .pop
      ✔ should return the last char
      ✔ should return None if the String is empty

SUCCEEDED: 4   FAILED: 0   TOTAL: 4
```
