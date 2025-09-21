use culit::culit;

mod custom_literal {}

#[culit]
fn foo() {
    let a = 10km;
}
