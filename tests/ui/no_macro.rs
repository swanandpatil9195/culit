//! `custom_literal::int::km` does not exist, I expect this to be an error directly on `10km`

use culit::culit;

mod custom_literal {
    pub mod int {}
}

#[culit]
fn main() {
    let a = 10km;
}
