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
