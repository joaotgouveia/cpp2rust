extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: i32,
}
pub fn operator_comma_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ (a.with(|__s| __s.v) * 10) } + { b.with(|__s| __s.v) }),
    };
}
pub fn operator_literal__k_1(mut v: u64) -> i64 {
    return (((v).wrapping_mul(1000_u64)) as i64);
}
pub fn operator_literal__half_2(mut v: f64) -> f64 {
    return (v / 2_f64);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 3 }));
    let t: Value<S> = Rc::new(RefCell::new(S { v: 4 }));
    (if ({
        ({
            let _a: Ptr<S> = s.as_pointer();
            operator_comma_0(_a, t.as_pointer())
        })
        .v
    } == 34)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"(s, t).v == 34"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Value<S> = Rc::new(RefCell::new(
                ({
                    let _a: Ptr<S> = s.as_pointer();
                    operator_comma_0(_a, t.as_pointer())
                }),
            ));
            let _b: Ptr<S> = s.as_pointer();
            operator_comma_0(_a.as_pointer(), _b)
        })
        .v
    } == 343)
    {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"(s, t, s).v == 343"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ operator_literal__k_1(2_u64) }) == 2000_i64) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"2_k == 2000"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ operator_literal__half_2(3.0E+0) }) == 1.5E+0) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"3.0_half == 1.5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ operator_literal__k_1(4_u64) }) == 4000_i64) {
        &(0);
    } else {
        ({
            __assert_fail_3(
                Ptr::<i8>::from_string_literal(b"operator\"\"_k(4) == 4000"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_3(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
