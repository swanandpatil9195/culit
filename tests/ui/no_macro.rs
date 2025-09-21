mod custom_literal {
    pub mod int {}
}

#[culit]
fn foo() {
    let a = 10km;
}
