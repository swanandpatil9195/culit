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

    pub mod int {
        pub(crate) use super::w;
    }

    pub mod float {
        pub(crate) use super::w;
    }

    pub mod str {
        pub(crate) use super::w;
    }

    pub mod char {
        pub(crate) use super::w;
    }

    pub mod byte_char {
        pub(crate) use super::w;
    }

    pub mod byte_str {
        pub(crate) use super::w;
    }

    pub mod c_str {
        pub(crate) use super::w;
    }
}
