//! `custom_literal::int` does not exist. This is a fundamental error not related to any integer,
//! so I expect the span to be at `#[culit]`

use culit::culit;

mod custom_literal {}

#[culit]
fn main() {
    let a = 10km;
}
