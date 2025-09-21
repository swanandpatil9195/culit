use culit::culit;

#[test]
fn ui() {
    let trybuild = trybuild::TestCases::new();
    trybuild.compile_fail("tests/ui/*.rs");
}

#[test]
#[culit]
fn int() {
    assert_eq!(0b100id, stringify!("100" 2));
    assert_eq!(0o100id, stringify!("100" 8));
    assert_eq!(100id, stringify!("100" 10));
    assert_eq!(0x100id, stringify!("100" 16));

    assert_eq!(0b100_id, stringify!("100" 2));
    assert_eq!(0o100_id, stringify!("100" 8));
    assert_eq!(100_id, stringify!("100" 10));
    assert_eq!(0x100_id, stringify!("100" 16));

    assert_eq!(0b1_00id, stringify!("100" 2));
    assert_eq!(0o_1_00id, stringify!("100" 8));
    assert_eq!(1_00id, stringify!("100" 10));
    assert_eq!(0x1_00id, stringify!("100" 16));

    assert_eq!(0b_1_0_0_id, stringify!("100" 2));
    assert_eq!(0o_1_0_0_id, stringify!("100" 8));
    assert_eq!(1_0_0_id, stringify!("100" 10));
    assert_eq!(0x_1_0_0_id, stringify!("100" 16));
}

#[test]
#[culit]
fn float() {
    assert_eq!(70.8e7id, stringify!("70" "8" "e7"));
    assert_eq!(70e7id, stringify!("70" "" "e7"));
    assert_eq!(70.0id, stringify!("70" "0" ""));
    assert_eq!(70.0e7id, stringify!("70" "0" "e7"));
    assert_eq!(70.0_e7id, stringify!("70" "0" "e7"));
    assert_eq!(7_0.0e7id, stringify!("70" "0" "e7"));
    assert_eq!(7_0.0e7_id, stringify!("70" "0" "e7"));
    assert_eq!(7_0_.0_e_7_id, stringify!("70" "0" "e7"));
}

#[test]
#[culit]
fn str() {
    assert_eq!("foo"id, stringify!("foo"));
    assert_eq!("\nfoo"id, stringify!("\nfoo"));
    assert_eq!(r"foo"id, stringify!("foo"));
    assert_eq!(r#"foo"#id, stringify!("foo"));
    assert_eq!(r#"foo\"#id, stringify!("foo\\"));
}

#[test]
#[culit]
fn byte_char() {
    assert_eq!(b'a'id, stringify!(97));
}

#[test]
#[culit]
fn byte_str() {
    assert_eq!(b"hello"id, stringify!(b"hello"));
    assert_eq!(b"hell\\o"id, stringify!(b"hell\\o"));
    assert_eq!(br"hell\o"id, stringify!(b"hell\\o"));
    assert_eq!(br#"hello"#id, stringify!(b"hello"));
}

#[test]
#[culit]
fn char() {
    assert_eq!('a'id, stringify!('a'));
}

#[test]
#[culit]
fn c_str() {
    assert_eq!(c"hello"id, stringify!(c"hello"));
    assert_eq!(c"hell\\o"id, stringify!(c"hell\\o"));
    assert_eq!(cr"hell\o"id, stringify!(c"hell\\o"));
    assert_eq!(cr#"hello"#id, stringify!(c"hello"));
}

mod custom_literal {
    // `id` for "Identity"
    macro_rules! id {
        ($($tt:tt)*) => {
            stringify!($($tt)*)
        };
    }

    pub(crate) use id;

    pub mod int {
        pub(crate) use super::id;
    }

    pub mod float {
        pub(crate) use super::id;
    }

    pub mod str {
        pub(crate) use super::id;
    }

    pub mod char {
        pub(crate) use super::id;
    }

    pub mod byte_char {
        pub(crate) use super::id;
    }

    pub mod byte_str {
        pub(crate) use super::id;
    }

    pub mod c_str {
        pub(crate) use super::id;
    }
}
