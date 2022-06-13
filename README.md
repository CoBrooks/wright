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
wright = { "git": "https://github.com/CoBrooks/wright" }
```

### src/test.rs

```rust
use wright::*;

fn main() {
    describe("Vec", || {
        describe("a newly instantiated Vec", || {
            let v: Vec<()> = Vec::new();

            it("should be empty", move || {
                expect(v).to().be().empty()
            });
        });
    });
    
    describe("Option", || {
        describe("Some(..)", || {
            let s: Option<()> = Some(());

            it("should be some", move || {
                expect(s).to().be().some()
            });
            
            let n: Option<()> = None;
            it("should not be none", move || {
                expect(n).to().not().be().none()
            });
        });
    });
}
```

## Running the Tests

```bash
$ cargo test --test wright

  Vec
    a newly instantiated Vec
      ✔ should be empty

SUCCEEDED: 1    FAILED: 0    TOTAL: 1

  Option
    Some(..)
      ✔ should be some
      ✘ should not be none

SUCCEEDED: 2    FAILED: 1    TOTAL: 3
```
