// Check that macro expanded underscore imports behave as expected

// check-pass

#![feature(decl_macro, rustc_attrs)]

mod x {
    pub use std::ops::BitOr as _;
}

macro m() {
    mod w {
        mod y {
            pub use std::ops::Deref as _;
        }
        use crate::x::*;
        use self::y::*;
        use std::ops::DerefMut as _;
        fn f() {
            false.bitor(true);
            (&()).deref();
            (&mut ()).deref_mut();
        }
    }
}

#[rustc_macro_transparency = "transparent"]
macro n() {
    mod z {
        pub use std::ops::Deref as _;
    }
    use crate::x::*;
    use crate::z::*;
    use std::ops::DerefMut as _;
    fn f() {
        false.bitor(true);
        (&()).deref();
        (&mut ()).deref_mut();
    }
}

m!();
n!();

fn main() {}
