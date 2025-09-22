//! [![crates.io](https://img.shields.io/crates/v/culit?style=flat-square&logo=rust)](https://crates.io/crates/culit)
//! [![docs.rs](https://img.shields.io/badge/docs.rs-culit-blue?style=flat-square&logo=docs.rs)](https://docs.rs/culit)
//! ![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue?style=flat-square)
//! ![msrv](https://img.shields.io/badge/msrv-1.58-blue?style=flat-square&logo=rust)
//! [![github](https://img.shields.io/github/stars/nik-rev/culit)](https://github.com/nik-rev/culit)
//!
//! You probably know that numbers in Rust can be suffixed to specify their type, e.g. `100i32`.
//! But did you know that syntactically *any* literal can have a suffix? **And did you know that the suffix can be whatever you want**?
//!
//! This crate provides an attribute macro `#[culit]` for "Custom Literals". When applied to any statement, it enables using custom literals in that statement.
//!
//! ```toml
//! [dependencies]
//! culit = "0.3"
//! ```
//!
//! Note: `culit` does not have any dependencies such as `syn` or `quote`, and it is a simple mapping `SourceCode -> SourceCode`, so compile-speeds will be very fast.
//!
//! # Example
//!
//! A [`NonZeroUsize`](std::num::NonZeroUsize) literal that fails to compile if it is `0`: `100nzusize`
//!
//! ```
//! use culit::culit;
//! use std::num::NonZeroUsize;
//!
//! #[culit]
//! fn main() {
//!     assert_eq!(100nzusize, NonZeroUsize::new(100).unwrap());
//!     // COMPILE ERROR!
//!     // let illegal = 0nzusize;
//! }
//!
//! mod custom_literal {
//!     pub mod integer {
//!         macro_rules! nzusize {
//!             // handle `0` specially
//!             (0) => {
//!                 compile_error!("`0` is not a valid `NonZeroUsize`")
//!             };
//!             ($value:literal) => {
//!                 const { NonZeroUsize::new($value).unwrap() }
//!             };
//!         }
//!         pub(crate) use nzusize;
//!     }
//! }
//! ```
//!
//! # IDE Support
//!
//! Hovering over the custom literals shows documentation for the macro that generates them. You can also do "goto definition". It's quite nice!
//!
//! ![IDE Support](https://raw.githubusercontent.com/nik-rev/culit/71f4a2b32eb87b955d0c953bd3e90e80bd6a938d/ide_support.png)
//!
//! # More Examples
//!
//!
//! Python-like f-strings: `"hello {name}"f`
//!
//! ```
//! use culit::culit;
//! use std::time::Duration;
//!
//! #[culit]
//! fn main() {
//!     let name = "bob";
//!     let age = 23;
//!
//!     assert_eq!(
//!         "hi, my name is {name} and I am {age} years old"f,
//!         format!("hi, my name is {name} and I am {age} years old")
//!     );
//! }
//!
//! mod custom_literal {
//!     pub mod string {
//!         macro_rules! f {
//!             ($value:literal) => {
//!                 format!($value)
//!             };
//!         }
//!         pub(crate) use f;
//!     }
//! }
//! ```
//!
//! [`Duration`](std::time::Duration) literals: `100m`, `2h`...
//!
//! ```
//! use culit::culit;
//! use std::time::Duration;
//!
//! #[culit]
//! fn main() {
//!     assert_eq!(
//!         100d + 11h + 8m + 7s,
//!         Duration::from_secs(100 * 60 * 60 * 24)
//!         + Duration::from_secs(11 * 60 * 60)
//!         + Duration::from_secs(8 * 60)
//!         + Duration::from_secs(7)
//!     );
//! }
//!
//! mod custom_literal {
//!     pub mod integer {
//!         // day
//!         macro_rules! d {
//!             ($value:literal) => {
//!                 Duration::from_secs(60 * 60 * 24 * $value)
//!             };
//!         }
//!         pub(crate) use d;
//!
//!         // hour
//!         macro_rules! h {
//!             ($value:literal) => {
//!                 Duration::from_secs(60 * 60 * $value)
//!             };
//!         }
//!         pub(crate) use h;
//!
//!         // minute
//!         macro_rules! m {
//!             ($value:literal) => {
//!                 Duration::from_secs(60 * $value)
//!             };
//!         }
//!         pub(crate) use m;
//!
//!         // second
//!         macro_rules! s {
//!             ($value:literal) => {
//!                 Duration::from_secs($value)
//!             };
//!         }
//!         pub(crate) use s;
//!     }
//! }
//! ```
//!
//! The possibilities are *endless!*
//!
//! # Details
//!
//! `#[culit]` replaces every literal that has a custom suffix with a call to the macro
//! at `crate::custom_literal::<type>::<suffix>!($value)`, where `$value` is the literal with the suffix stripped:
//!
//! |literal|expansion|
//! |---|---|
//! | `100km` | `crate::custom_literal::int::km!(100)` |
//! | `70.008e7feet` | `crate::custom_literal::float::feet!(70.008e7)` |
//! | `'a'ascii` | `crate::custom_literal::char::ascii!('a')` |
//! | `b'a'ascii` | `crate::custom_literal::byte_char::ascii!(b'a')` |
//! | `"foo"bar` | `crate::custom_literal::str::bar!("foo")` |
//! | `b"foo"bar` | `crate::custom_literal::byte_str::bar!(b"foo")` |
//! | `c"foo"bar` | `crate::custom_literal::c_str::bar!(c"foo")` |
//!
//! Notes:
//!
//! - Built-in suffixes like `usize` and `f32` do **not** expand, so you cannot overwrite them.
//! - Escapes are fully processed, so there's no `raw_byte_str`. `rb#"f\oo"#` just becomes `b"f\\oo"`
//!
//! ## Skeleton
//!
//! Here's a skeleton for the `custom_literal` module which must exist at `crate::custom_literal`.
//! This module adds a new literal for every type of literal:
//!
//! ```
//! mod custom_literal {
//!     pub mod integer {
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod decimal {
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod string {
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod character {
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod byte_character {
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod byte_string {
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod c_string {
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//! }
//! ```
//!
//! # Nightly
//!
//! You need to use `#[culit]` attribute everywhere you want to use these literals. On nightly, you can apply it on the module:
//!
//! ```ignore
//! #![feature(custom_inner_attributes)]
//! #![feature(proc_macro_hygiene)]
//! #![culit::culit]
//! ```
//!
//! While this *works*, I wouldn't recommend it - currently rust-analyzer is unable to properly work with custom inner attributes
//! that modify the whole crate. For example, if you write `0nzusize` which produces a compiler error, the span of the error will point to
//! the macro `crate::custom_literal::int::nzusize` but *not* the actual `0nzusize`, which makes it very hard to debug these
#![allow(clippy::needless_doctest_main)]

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// Supports using custom literals such as `10km` defined at `crate::custom_literal::int::km`
///
/// # Example
///
/// ```
/// # use culit::culit;
/// # #[derive(PartialEq, Debug)]
/// struct Kilometers(u32);
///
/// mod custom_literal {
///     pub mod integer {
///         macro_rules! km {
///             ($value:literal) => {
///                 $crate::Kilometers($value)
///             }
///         }
///         pub(crate) use km;
///     }
/// }
///
/// #[culit]
/// fn main() {
///     assert_eq!(10km, Kilometers(10));
/// }
/// ```
///
/// For more information, see the [crate-level](crate) documentation
#[proc_macro_attribute]
pub fn culit(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        panic!("`#[culit]` does not take any arguments between `(...)`")
    }

    transform(input)
}

/// Recursively replaces all literals in the `TokenStream` with a call to `crate::custom_literal::$literal_type::$suffix!($ts)`
fn transform(ts: TokenStream) -> TokenStream {
    ts.into_iter()
        .flat_map(|tt| {
            // I1 = [TokenTree; 12]
            // I2 = [TokenTree; 1]
            // I3 = [TokenTree; 3]

            match tt {
                TokenTree::Literal(tt_lit) => {
                    let span = tt_lit.span();

                    // NOTE: `litrs::Literal::from(token_tree::Literal) exists but it unnecessarily takes by-value,
                    // so we avoid an unnecessary clone here
                    let lit = litrs::Literal::parse(tt_lit.to_string()).expect(concat!(
                        "bug in the implementation of `litrs`, ",
                        "`token_tree::Literal` -> `litrs::Literal` is infallible"
                    ));

                    let suffix = lit.suffix();

                    if suffix.is_empty() {
                        // Totally skip this literal as there's no suffix
                        return AnonIter::I2([TokenTree::Literal(tt_lit)].into_iter());
                    }

                    const RESERVED_MESSAGE: &str = concat!(
                        " is not currently used ",
                        "by rust, but it likely will be in the future",
                        ". To avoid breakage and not compromise rust's compatibility guarantees, ",
                        "we forbid this suffix"
                    );

                    match &lit {
                        litrs::Literal::Integer(integer_lit) => {
                            if INT_SUFFIXES.contains(&suffix) {
                                return AnonIter::I2([TokenTree::Literal(tt_lit)].into_iter());
                            } else if INT_SUFFIXES_RESERVED.contains(&suffix) {
                                return AnonIter::I3(
                                    CompileError::new(
                                        span,
                                        format!("suffix {suffix} {RESERVED_MESSAGE}"),
                                    )
                                    .into_iter(),
                                );
                            }

                            let mut int = String::with_capacity(integer_lit.raw_input().len());
                            int.push_str(integer_lit.base().prefix());
                            int.push_str(integer_lit.raw_main_part());
                            int.parse::<Literal>().expect(concat!(
                                "if it wasn't a valid literal, `litrs::Literal`",
                                " would not be able to parse it"
                            ));

                            AnonIter::I1(
                                expand_custom_literal(
                                    lit_name::INTEGER,
                                    suffix,
                                    span,
                                    TokenStream::from(TokenTree::Literal(
                                        int.parse::<Literal>().expect(concat!(
                                            "if it wasn't a valid literal, `litrs::Literal`",
                                            " would not be able to parse it"
                                        )),
                                    )),
                                )
                                .into_iter(),
                            )
                        }
                        // crate::custom_literal::str::$suffix!($value)
                        litrs::Literal::String(string_lit) => AnonIter::I1(
                            expand_custom_literal(
                                lit_name::STRING,
                                suffix,
                                span,
                                TokenStream::from(
                                    // $value
                                    TokenTree::Literal(Literal::string(string_lit.value()))
                                        .with_span(span),
                                ),
                            )
                            .into_iter(),
                        ),
                        litrs::Literal::Float(float_lit) => {
                            if FLOAT_SUFFIXES.contains(&suffix) {
                                return AnonIter::I2([TokenTree::Literal(tt_lit)].into_iter());
                            } else if FLOAT_SUFFIXES_RESERVED.contains(&suffix) {
                                return AnonIter::I3(
                                    CompileError::new(
                                        span,
                                        format!("suffix {suffix} {RESERVED_MESSAGE}"),
                                    )
                                    .into_iter(),
                                );
                            }

                            AnonIter::I1(
                                expand_custom_literal(
                                    lit_name::DECIMAL,
                                    suffix,
                                    span,
                                    TokenStream::from(TokenTree::Literal(
                                        float_lit.number_part().parse::<Literal>().expect(concat!(
                                            "if it wasn't a valid literal, `litrs::Literal`",
                                            " would not be able to parse it"
                                        )),
                                    )),
                                )
                                .into_iter(),
                            )
                        }
                        // crate::custom_literal::char::$suffix!($value)
                        litrs::Literal::Char(char_lit) => AnonIter::I1(
                            expand_custom_literal(
                                lit_name::CHARACTER,
                                suffix,
                                span,
                                TokenStream::from(
                                    // $value
                                    TokenTree::Literal(Literal::character(char_lit.value()))
                                        .with_span(span),
                                ),
                            )
                            .into_iter(),
                        ),
                        // crate::custom_literal::byte_char::$suffix!($value)
                        litrs::Literal::Byte(byte_lit) => AnonIter::I1(
                            expand_custom_literal(
                                lit_name::BYTE_CHARACTER,
                                suffix,
                                span,
                                TokenStream::from(
                                    // $value
                                    TokenTree::Literal(Literal::byte_character(byte_lit.value()))
                                        .with_span(span),
                                ),
                            )
                            .into_iter(),
                        ),
                        // crate::custom_literal::byte_str::$suffix!($value)
                        litrs::Literal::ByteString(byte_string_lit) => {
                            AnonIter::I1(
                                expand_custom_literal(
                                    lit_name::BYTE_STRING,
                                    suffix,
                                    span,
                                    TokenStream::from(
                                        // $value
                                        TokenTree::Literal(Literal::byte_string(
                                            byte_string_lit.value(),
                                        ))
                                        .with_span(span),
                                    ),
                                )
                                .into_iter(),
                            )
                        }
                        litrs::Literal::CString(cstring_lit) => {
                            AnonIter::I1(
                                expand_custom_literal(
                                    lit_name::C_STRING,
                                    suffix,
                                    span,
                                    TokenStream::from(
                                        // $value
                                        TokenTree::Literal(Literal::c_string(cstring_lit.value()))
                                            .with_span(span),
                                    ),
                                )
                                .into_iter(),
                            )
                        }
                        litrs::Literal::Bool(_bool_lit) => {
                            unreachable!(
                                "booleans aren't `TokenTree::Literal`, they're `TokenTree::Ident`"
                            )
                        }
                    }
                }
                TokenTree::Group(group) => {
                    AnonIter::I2(
                        [TokenTree::Group(Group::new(
                            group.delimiter(),
                            // Recurse
                            transform(group.stream()),
                        ))]
                        .into_iter(),
                    )
                }
                next_tt => AnonIter::I2([next_tt].into_iter()),
            }
        })
        .collect()
}

/// Expands a custom literal into `crate::custom_literal::$literal_type::$suffix!($ts)`
fn expand_custom_literal(
    literal_type: &str,
    suffix: &str,
    span: Span,
    ts: TokenStream,
) -> [TokenTree; 12] {
    [
        TokenTree::Ident(Ident::new("crate", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("custom_literal", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new(literal_type, Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new(suffix, span)),
        TokenTree::Punct(Punct::new('!', Spacing::Joint)).with_span(span),
        TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, ts)).with_span(span),
    ]
}

/// `.into_iter()` generates `compile_error!($message)` at `$span`
struct CompileError {
    /// Where the compile error is generates
    pub span: Span,
    /// Message of the compile error
    pub message: String,
}

impl CompileError {
    /// Create a new compile error
    pub fn new(span: Span, message: impl AsRef<str>) -> Self {
        Self {
            span,
            message: message.as_ref().to_string(),
        }
    }
}

impl IntoIterator for CompileError {
    type Item = TokenTree;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [
            TokenTree::Ident(Ident::new("compile_error", self.span)),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)).with_span(self.span),
            TokenTree::Group(Group::new(Delimiter::Brace, {
                TokenStream::from(
                    TokenTree::Literal(Literal::string(&self.message)).with_span(self.span),
                )
            }))
            .with_span(self.span),
        ]
        .into_iter()
    }
}

trait TokenTreeExt {
    /// Set span of `TokenTree` without needing to create a new binding
    fn with_span(self, span: Span) -> TokenTree;
}

impl TokenTreeExt for TokenTree {
    fn with_span(mut self, span: Span) -> TokenTree {
        self.set_span(span);
        self
    }
}

// NOTE: Renaming them is a BREAKING CHANGE

/// Name of modules for all literal types
mod lit_name {
    pub const INTEGER: &str = "integer";
    pub const DECIMAL: &str = "decimal";
    pub const STRING: &str = "string";
    pub const CHARACTER: &str = "character";
    pub const BYTE_CHARACTER: &str = "byte_character";
    pub const BYTE_STRING: &str = "byte_string";
    pub const C_STRING: &str = "c_string";
}

// NOTE: Adding or modifying the constants is a BREAKING CHANGE

/// List of all integer suffixes currently accepted by Rust
#[rustfmt::skip]
const INT_SUFFIXES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
];

/// Integer suffixes currently not accepted, but could be in the future
const INT_SUFFIXES_RESERVED: &[&str] = &["i256", "u256"];

/// Float suffixes currently accepted by Rust
const FLOAT_SUFFIXES: &[&str] = &["f32", "f64"];

/// Float suffixes currently not accepted, but could be in the future
const FLOAT_SUFFIXES_RESERVED: &[&str] = &["f16", "f128"];

/// Wraps many `impl Iterator` which may be of different types
///
/// Functions returning `-> impl Iterator` must have the same return type
/// from all branches, but this is overly restrictive.
///
/// We may want to return 2 or more different iterators from the same function,
/// and this type allows that by wrapping each unique iterator in a variant of
/// this enum.
enum AnonIter<T, I1: Iterator<Item = T>, I2: Iterator<Item = T>, I3: Iterator<Item = T>> {
    /// The first `impl Iterator`
    I1(I1),
    /// The second `impl Iterator`
    I2(I2),
    /// The third `impl Iterator`
    I3(I3),
}

impl<T, I1: Iterator<Item = T>, I2: Iterator<Item = T>, I3: Iterator<Item = T>> Iterator
    for AnonIter<T, I1, I2, I3>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AnonIter::I1(i1) => i1.next(),
            AnonIter::I2(i2) => i2.next(),
            AnonIter::I3(i3) => i3.next(),
        }
    }
}
