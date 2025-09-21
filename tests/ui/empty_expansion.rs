// Empty expansion is a compile error.
// I expect the compile error span to be exactly at `10km`

mod custom_literal {
    pub mod int {
        macro_rules! km {
            ($value:literal $base:literal) => {};
        }
        pub(crate) use km;
    }
}

#[culit]
fn foo() {
    let a = 10km;
}
