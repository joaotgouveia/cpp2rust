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
    pub v: u32,
}
pub fn operator_bitnot_0(a: Ptr<S>) -> S {
    return S {
        v: !a.with(|__s| __s.v),
    };
}
pub fn operator_bitand_1(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } & { b.with(|__s| __s.v) }),
    };
}
pub fn operator_bitor_2(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } | { b.with(|__s| __s.v) }),
    };
}
pub fn operator_bitxor_3(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } ^ { b.with(|__s| __s.v) }),
    };
}
pub fn operator_shl_4(a: Ptr<S>, mut n: i32) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } << { n }),
    };
}
pub fn operator_shr_5(a: Ptr<S>, mut n: i32) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } >> { n }),
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 12_u32 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 10_u32 }));
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitnot_0(_a)
        })
        .v
    } == !12_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(~a).v == ~0b1100u"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitand_1(_a, b.as_pointer())
        })
        .v
    } == 8_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(a & b).v == 0b1000"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitor_2(_a, b.as_pointer())
        })
        .v
    } == 14_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(a | b).v == 0b1110"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_bitxor_3(_a, b.as_pointer())
        })
        .v
    } == 6_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(a ^ b).v == 0b0110"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_shl_4(_a, 2)
        })
        .v
    } == 48_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(a << 2).v == 0b110000"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_shr_5(_a, 2)
        })
        .v
    } == 3_u32)
    {
        &(0);
    } else {
        ({
            __assert_fail_6(
                Ptr::<i8>::from_string_literal(b"(a >> 2).v == 0b11"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_6(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
