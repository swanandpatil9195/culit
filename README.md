# `docstr`

<!-- cargo-rdme start -->

[![crates.io](https://img.shields.io/crates/v/culit?style=flat-square&logo=rust)](https://crates.io/crates/culit)
[![docs.rs](https://img.shields.io/badge/docs.rs-culit-blue?style=flat-square&logo=docs.rs)](https://docs.rs/culit)
![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
![msrv](https://img.shields.io/badge/msrv-1.56-blue?style=flat-square&logo=rust)
[![github](https://img.shields.io/github/stars/nik-rev/culit)](https://github.com/nik-rev/culit)

You probably know that numbers in Rust can be suffixed to specify their type, e.g. `100i32`.
But did you know that syntactically *any* literal can have a suffix? **And did you know that the suffix can be whatever you want**?

This crate provides an attribute macro `#[culit]` for "Custom Literals". When applied to any statement, it enables using custom literals in that statement.

```toml
[dependencies]
culit = "0.1"
```

## Examples

The possibilities are *endless!*

[`NonZeroUsize`](std::num::NonZeroUsize) literal that fails to compile if it is `0`

```rust
use culit::culit;
use std::num::NonZeroUsize;

mod custom_literal {
    pub mod int {
        macro_rules! nzusize {
            // handle `0` specially
            ("0" $base:literal) => {
                compile_error!("`0` is not a valid `NonZeroUsize`")
            };
            ($value:literal $base:literal) => {
                NonZeroUsize::new(usize::from_str_radix($value, $base).unwrap()).unwrap()
            };
        }
        pub(crate) use nzusize;
    }
}

#[culit]
fn main() {
    assert_eq!(100nzusize, NonZeroUsize::new(100).unwrap());
    // COMPILE ERROR!
    // let illegal = 0nzusize;
}
```

Python-like f-strings

```rust
use culit::culit;
use std::time::Duration;

mod custom_literal {
    pub mod str {
        macro_rules! f {
            ($value:literal) => {
                format!($value)
            };
        }
        pub(crate) use f;
    }
}

#[culit]
fn main() {
    let name = "bob";
    let age = 23;

    assert_eq!(
        "hi, my name is {name} and I am {age} years old"f,
        format!("hi, my name is {name} and I am {age} years old")
    );
}
```

[`Duration`](std::time::Duration) literals

```rust
use culit::culit;
use std::time::Duration;

mod custom_literal {
    pub mod int {
        // day
        macro_rules! d {
            ($value:literal $base:literal) => {
                Duration::from_secs(60 * 60 * 24 * u64::from_str_radix($value, $base).unwrap())
            };
        }
        pub(crate) use d;

        // hour
        macro_rules! h {
            ($value:literal $base:literal) => {
                Duration::from_secs(60 * 60 * u64::from_str_radix($value, $base).unwrap())
            };
        }
        pub(crate) use h;

        // minute
        macro_rules! m {
            ($value:literal $base:literal) => {
                Duration::from_secs(60 * u64::from_str_radix($value, $base).unwrap())
            };
        }
        pub(crate) use m;

        // second
        macro_rules! s {
            ($value:literal $base:literal) => {
                Duration::from_secs(u64::from_str_radix($value, $base).unwrap())
            };
        }
        pub(crate) use s;
    }
}

#[culit]
fn main() {
    assert_eq!(
        100d + 11h + 8m + 7s,
        Duration::from_secs(100 * 60 * 60 * 24)
        + Duration::from_secs(11 * 60 * 60)
        + Duration::from_secs(8 * 60)
        + Duration::from_secs(7)
    );
}
```

## Details

`#[culit]` recursively replaces every literal that has a non-standard suffix with a call to the macro
at `crate::custom_literal::<type>::<suffix>!(...)`, for example:

- `100km` expands to `crate::custom_literal::int::km!("100" 10)`
    - `10` is the base (decimal)
    - `"100"` is the value
- `70.8e7feet` expands to `crate::custom_literal::float::feet!("70" "8" "e7")`
    - `"70"` is the part before the decimal
    - `"8"` is the part after the decimal
    - `"e7"` is the exponent
- `'a'ascii` expands to `crate::custom_literal::char::ascii!('a')`
- `b'a'ascii` expands to `crate::custom_literal::byte_char::ascii!(97)`
- `"foo"bar` expands to `crate::custom_literal::str::bar!("foo")`
- `b"foo"bar` expands to `crate::custom_literal::byte_str::bar!(b"foo")`
- `c"foo"bar` expands to `crate::custom_literal::c_str::bar!(c"foo")`

**Note**: Negative numbers like `-100` aren't literal themselves, instead it is 2 tokens: `-` followed by the literal `100`.
Implement [`Not`](std::ops::Not) for whatever your custom numeric literal expands to

Here's a skeleton for the `custom_literal` module which must exist at `crate::custom_literal` (you can `#[doc(hidden)]` it if you want).
This module adds a new literal for every type of literal:

```rust
mod custom_literal {
    pub mod int {
        // 0x100custom
        //
        // ^^ base - `16`. Can be one of: 16, 10, 8 or 2
        //   ^^^ value - "100"
        macro_rules! custom {
            ($value:literal $base:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod float {
        // 70.3141e100custom
        //
        // ^^ before_decimal - "70"
        //    ^^^^ after_decimal - "3141". Can be "" if no after_decimal
        //         ^^^ exponent - "100". Can be "" if no exponent
        macro_rules! custom {
            ($before_decimal:literal $after_decimal:literal $exponent:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod str {
        // "foo_bar"custom
        // ^^^^^^^^^ value - "foo_bar"
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod char {
        // 'x'custom
        // ^^^ value - 'x'
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod byte_char {
        // b'a'custom
        //   ^ value - 97
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod byte_str {
        // b"foo_bar"custom
        // ^^^^^^^^^^ value - b"foo_bar"
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }

    pub mod c_str {
        // c"string"custom
        // ^^^^^^^^^ value - c"string"
        macro_rules! custom {
            ($value:literal) => {
                // ...
            }
        }
        pub(crate) use custom;
    }
}
```

## Nightly

You need to use `#[culit]` attribute everywhere you want to use these literals. On nightly, you can apply it on the module:

```rust
#![feature(custom_inner_attributes)]
#![feature(proc_macro_hygiene)]
#![culit::culit]
```

While this *works*, I wouldn't recommend it - currently rust-analyzer is unable to properly work with custom inner attributes
that modify the whole crate. For example, if you write `0nzusize` which produces a compiler error, the span of the error will point to
the macro `crate::custom_literal::int::nzusize` but *not* the actual `0nzusize`, which makes it very hard to debug these

<!-- cargo-rdme end -->
