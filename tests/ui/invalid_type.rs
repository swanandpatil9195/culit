// Type is wrong here
// I expect the type mismatch error to be exactly at `0km`

use culit::culit;

struct Kilometer(u32);

mod custom_literal {
    pub mod int {
        macro_rules! km {
            ($value:literal $base:literal) => {
                const { std::num::NonZeroU32::new($value).unwrap() }
            };
        }
        pub(crate) use km;
    }
}

#[culit]
fn foo() {
    let a = 0km;
}
