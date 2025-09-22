//! Each literal emits a type mismatch error.
//! I expect the span of this error to be exactly at the literal

use culit::culit;

struct Kilometer(u32);

#[culit]
fn main() {
    0w;
    0.0w;
    'x'w;
    b'x'w;
    "x"w;
    b"x"w;
    c"x"w;
}

mod custom_literal {
    macro_rules! w {
        ($value:literal $($tt:tt)*) => {{
            let it: () = $value;
            it
        }};
    }
    pub(crate) use w;

    pub mod integer {
        pub(crate) use super::w;
    }

    pub mod decimal {
        pub(crate) use super::w;
    }

    pub mod string {
        pub(crate) use super::w;
    }

    pub mod character {
        pub(crate) use super::w;
    }

    pub mod byte_character {
        pub(crate) use super::w;
    }

    pub mod byte_string {
        pub(crate) use super::w;
    }

    pub mod c_string {
        pub(crate) use super::w;
    }
}
