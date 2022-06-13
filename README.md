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
            it("should convert &str -> String", || {
                let slice = "Hello, World!";

                expect(slice).to().be().a::<&str>()
                    && expect(String::from(slice)).to().be().a::<String>()
            });
        });

        describe("::new", || {
            it("should be empty", || {
                let s = String::new();

                expect(s.len()).to().equal(0)
            });
        });

        describe(".pop", || {
            it("should return the last char", || {
                let mut s = String::from("Hello, World!");
                let c = s.pop();

                expect(c).to().be().some()
                    && expect(c.unwrap()).to().equal('!')
            });
            
            it("should return None if the String is empty", || {
                let mut s = String::new();
                let c = s.pop();

                expect(c).to().be().none()
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
