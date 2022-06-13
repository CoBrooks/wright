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
