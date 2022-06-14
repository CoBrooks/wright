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
