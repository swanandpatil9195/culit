//! No `custom_literal` module in the crate root.
//! I expect span to be at `#[culit]` because it's a fundamental error,
//! unrelated to any integer in particular

use culit::culit;

#[culit]
fn main() {
    let a = 10km;
}
