//! No arguments can be passed to the proc macro

use culit::culit;

mod custom_literal {
    pub mod int {}
}

#[culit(no input allowed)]
fn main() {}
