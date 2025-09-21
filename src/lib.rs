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
//! culit = "0.1"
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
//!     pub mod int {
//!         macro_rules! nzusize {
//!             // handle `0` specially
//!             ("0" $base:literal) => {
//!                 compile_error!("`0` is not a valid `NonZeroUsize`")
//!             };
//!             ($value:literal $base:literal) => {
//!                 NonZeroUsize::new(usize::from_str_radix($value, $base).unwrap()).unwrap()
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
//! ![IDE Support](https://raw.githubusercontent.com/nik-rev/culit/0b7695e6a79cfbe2873a2c9d5936c6a93a5cbaaa/ide_support.png)
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
//!     pub mod str {
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
//!     pub mod int {
//!         // day
//!         macro_rules! d {
//!             ($value:literal $base:literal) => {
//!                 Duration::from_secs(60 * 60 * 24 * u64::from_str_radix($value, $base).unwrap())
//!             };
//!         }
//!         pub(crate) use d;
//!
//!         // hour
//!         macro_rules! h {
//!             ($value:literal $base:literal) => {
//!                 Duration::from_secs(60 * 60 * u64::from_str_radix($value, $base).unwrap())
//!             };
//!         }
//!         pub(crate) use h;
//!
//!         // minute
//!         macro_rules! m {
//!             ($value:literal $base:literal) => {
//!                 Duration::from_secs(60 * u64::from_str_radix($value, $base).unwrap())
//!             };
//!         }
//!         pub(crate) use m;
//!
//!         // second
//!         macro_rules! s {
//!             ($value:literal $base:literal) => {
//!                 Duration::from_secs(u64::from_str_radix($value, $base).unwrap())
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
//! `#[culit]` recursively replaces every literal that has a non-standard suffix with a call to the macro
//! at `crate::custom_literal::<type>::<suffix>!(...)`, for example:
//!
//! - `100km` expands to `crate::custom_literal::int::km!("100" 10)`
//!     - `"100"` is the value
//!     - `10` is the base (decimal)
//! - `70.8e7feet` expands to `crate::custom_literal::float::feet!("70" "8" "e7")`
//!     - `"70"` is the part before the decimal
//!     - `"8"` is the part after the decimal
//!     - `"e7"` is the exponent
//! - `'a'ascii` expands to `crate::custom_literal::char::ascii!('a')`
//! - `b'a'ascii` expands to `crate::custom_literal::byte_char::ascii!(97)`
//! - `"foo"bar` expands to `crate::custom_literal::str::bar!("foo")`
//! - `b"foo"bar` expands to `crate::custom_literal::byte_str::bar!(b"foo")`
//! - `c"foo"bar` expands to `crate::custom_literal::c_str::bar!(c"foo")`
//!
//! ## Negative literals
//!
//! Whatever the macros in `custom_literal::float` or `custom_literal::int` expand to needs to implement the [`Neg`](std::ops::Neg) trait in order to allow using `-` with the custom numeric literals.
//!
//! ### Details on negative literals
//!
//! You might think that a number like `-100` is a single literal, but it is not. It is 2 tokens: a punctuation `,` followed by a literal `100`. `-100km` expands like this:
//! - `-` is a punctuation, it is kept as-is
//! - `100km` is a literal `100` with suffix `km`. It expands to `crate::custom_literal::int::km!("100" 10)`.
//! - `"100"` is string representation of the number, `10` is the base (which could also be `2`, `8` or `16`)
//! - The macro receives a string `"100"` instead of an integer `100` because procedural macros cannot create integer literals that are larger than `u128`, but we want to support integer literals of arbitrary size.
//! - More importantly, interpreting the number itself without the base is a logic error, so passing a string instead of integer makes it far less likely that you'll make mistakes
//! - `-100km` overall expands to `-crate::custom_literal::int::km!("100" 10)`. Notice the `-` at the beginning, it is kept the same. Whatever `km!` expands to needs to implement the [`Neg`](std::ops::Neg) trait to be able to be used with the `-` operator.
//!
//! ## Skeleton
//!
//! Here's a skeleton for the `custom_literal` module which must exist at `crate::custom_literal`.
//! This module adds a new literal for every type of literal:
//!
//! ```
//! mod custom_literal {
//!     pub mod int {
//!         // 0x100custom
//!         //
//!         // ^^ base - `16`. Can be one of: 16, 10, 8 or 2
//!         //   ^^^ value - "100"
//!         macro_rules! custom {
//!             ($value:literal $base:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod float {
//!         // 70.3141e100custom
//!         //
//!         // ^^ before_decimal - "70"
//!         //    ^^^^ after_decimal - "3141". Can be "" if no after_decimal
//!         //         ^^^ exponent - "100". Can be "" if no exponent
//!         macro_rules! custom {
//!             ($before_decimal:literal $after_decimal:literal $exponent:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod str {
//!         // "foo_bar"custom
//!         // ^^^^^^^^^ value - "foo_bar"
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod char {
//!         // 'x'custom
//!         // ^^^ value - 'x'
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod byte_char {
//!         // b'a'custom
//!         //   ^ value - 97
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod byte_str {
//!         // b"foo_bar"custom
//!         // ^^^^^^^^^^ value - b"foo_bar"
//!         macro_rules! custom {
//!             ($value:literal) => {
//!                 // ...
//!             }
//!         }
//!         pub(crate) use custom;
//!     }
//!
//!     pub mod c_str {
//!         // c"string"custom
//!         // ^^^^^^^^^ value - c"string"
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
///     pub mod int {
///         macro_rules! km {
///             ($value:literal $base:literal) => {
///                 $crate::Kilometers(u32::from_str_radix($value, $base).unwrap())
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
                        return TokenStream::from(TokenTree::Literal(tt_lit));
                    }

                    const RESERVED_MESSAGE: &str = concat!(
                        " is not currently used ",
                        "by rust, but it likely will be in the future",
                        ". To avoid breakage and not compromise rust's compatibility guarantees, ",
                        "we forbid this suffix"
                    );

                    match &lit {
                        litrs::Literal::Bool(_bool_lit) => {
                            unreachable!(
                                "booleans aren't `TokenTree::Literal`, they're `TokenTree::Ident`"
                            )
                        }
                        litrs::Literal::Integer(integer_lit) => {
                            if INT_SUFFIXES.contains(&suffix) {
                                return TokenStream::from(TokenTree::Literal(tt_lit));
                            } else if INT_SUFFIXES_RESERVED.contains(&suffix) {
                                return CompileError::new(
                                    span,
                                    format!("suffix {suffix} {RESERVED_MESSAGE}"),
                                )
                                .into_iter()
                                .collect();
                            }

                            // crate::custom_literal::int::$suffix!($value, $base)
                            expand_custom_literal(
                                lit_name::INT,
                                suffix,
                                span,
                                TokenStream::from_iter([
                                    // $value
                                    TokenTree::Literal(Literal::string(
                                        &integer_lit.raw_main_part().split('_').collect::<String>(),
                                    ))
                                    .with_span(span),
                                    // $base
                                    TokenTree::Literal(Literal::u8_unsuffixed(
                                        integer_lit.base().value(),
                                    ))
                                    .with_span(span),
                                ]),
                            )
                        }
                        litrs::Literal::Float(float_lit) => {
                            if FLOAT_SUFFIXES.contains(&suffix) {
                                return TokenStream::from(TokenTree::Literal(tt_lit));
                            }
                            if FLOAT_SUFFIXES_RESERVED.contains(&suffix) {
                                return CompileError::new(
                                    span,
                                    format!("suffix {suffix} {RESERVED_MESSAGE}"),
                                )
                                .into_iter()
                                .collect();
                            }

                            // crate::custom_literal::float::$suffix!($before_decimal $after_decimal $exponent)
                            expand_custom_literal(
                                lit_name::FLOAT,
                                suffix,
                                span,
                                TokenStream::from_iter([
                                    // $before_decimal
                                    TokenTree::Literal(Literal::string(
                                        &float_lit.integer_part().split('_').collect::<String>(),
                                    ))
                                    .with_span(span),
                                    // $after_decimal
                                    TokenTree::Literal(Literal::string(
                                        &float_lit
                                            .fractional_part()
                                            .unwrap_or_default()
                                            .split('_')
                                            .collect::<String>(),
                                    ))
                                    .with_span(span),
                                    // $exponent
                                    TokenTree::Literal(Literal::string(
                                        &float_lit.exponent_part().split('_').collect::<String>(),
                                    ))
                                    .with_span(span),
                                ]),
                            )
                        }
                        // crate::custom_literal::char::$suffix!($value)
                        litrs::Literal::Char(char_lit) => expand_custom_literal(
                            lit_name::CHAR,
                            suffix,
                            span,
                            TokenStream::from(
                                // $value
                                TokenTree::Literal(Literal::character(char_lit.value()))
                                    .with_span(span),
                            ),
                        ),
                        // crate::custom_literal::byte_char::$suffix!($value)
                        litrs::Literal::Byte(byte_lit) => expand_custom_literal(
                            lit_name::BYTE_CHAR,
                            suffix,
                            span,
                            TokenStream::from(
                                // $value
                                TokenTree::Literal(Literal::u8_unsuffixed(byte_lit.value()))
                                    .with_span(span),
                            ),
                        ),
                        // crate::custom_literal::str::$suffix!($value)
                        litrs::Literal::String(string_lit) => expand_custom_literal(
                            lit_name::STR,
                            suffix,
                            span,
                            TokenStream::from(
                                // $value
                                TokenTree::Literal(Literal::string(string_lit.value()))
                                    .with_span(span),
                            ),
                        ),
                        // crate::custom_literal::byte_str::$suffix!($value)
                        litrs::Literal::ByteString(byte_string_lit) => expand_custom_literal(
                            lit_name::BYTE_STR,
                            suffix,
                            span,
                            TokenStream::from(
                                // $value
                                TokenTree::Literal(Literal::byte_string(byte_string_lit.value()))
                                    .with_span(span),
                            ),
                        ),
                        #[cfg(not(has_c_string))]
                        litrs::Literal::CString(_cstring_lit) => {
                            return CompileError::new(
                                tt_lit.span(),
                                concat!(
                                    "custom c-string literal with suffix ",
                                    "is only supported on Rust version >=1.79"
                                ),
                            )
                            .into_iter()
                            .collect();
                        }
                        // crate::custom_literal::c_str::$suffix!($value)
                        #[cfg(has_c_string)]
                        // lints for usage of "Literal::c_string" but we explicitly
                        // check that we are on a version that allows it
                        #[cfg_attr(has_c_string, allow(clippy::incompatible_msrv))]
                        litrs::Literal::CString(cstring_lit) => expand_custom_literal(
                            lit_name::C_STR,
                            suffix,
                            span,
                            TokenStream::from(
                                // $value
                                TokenTree::Literal(Literal::c_string(cstring_lit.value()))
                                    .with_span(span),
                            ),
                        ),
                    }
                }
                TokenTree::Group(group) => TokenStream::from(TokenTree::Group(Group::new(
                    group.delimiter(),
                    // Recurse
                    transform(group.stream()),
                ))),
                tt => TokenStream::from(tt),
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
) -> TokenStream {
    TokenStream::from_iter([
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
    ])
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
    pub const INT: &str = "int";
    pub const FLOAT: &str = "float";
    pub const STR: &str = "str";
    pub const CHAR: &str = "char";
    pub const BYTE_CHAR: &str = "byte_char";
    pub const BYTE_STR: &str = "byte_str";
    #[cfg(has_c_string)]
    pub const C_STR: &str = "c_str";
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
