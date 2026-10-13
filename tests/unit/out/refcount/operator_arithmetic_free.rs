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
pub fn operator_add_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } + { b.with(|__s| __s.v) }),
    };
}
pub fn operator_sub_1(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } - { b.with(|__s| __s.v) }),
    };
}
pub fn operator_mul_2(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } * { b.with(|__s| __s.v) }),
    };
}
pub fn operator_div_3(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } / { b.with(|__s| __s.v) }),
    };
}
pub fn operator_rem_4(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } % { b.with(|__s| __s.v) }),
    };
}
pub fn operator_pos_5(a: Ptr<S>) -> S {
    return S {
        v: a.with(|__s| __s.v),
    };
}
pub fn operator_neg_6(a: Ptr<S>) -> S {
    return S {
        v: -a.with(|__s| __s.v),
    };
}
pub fn operator_inc_7(a: Ptr<S>) -> Ptr<S> {
    field!(a, v).with_mut(|__v| __v.prefix_inc());
    return (a).clone();
}
pub fn operator_post_inc_8(a: Ptr<S>, mut _a1: i32) -> S {
    let mut old: S = (*a.upgrade().deref()).clone();
    field!(a, v).with_mut(|__v| __v.prefix_inc());
    return old;
}
pub fn operator_dec_9(a: Ptr<S>) -> Ptr<S> {
    field!(a, v).with_mut(|__v| __v.prefix_dec());
    return (a).clone();
}
pub fn operator_post_dec_10(a: Ptr<S>, mut _a1: i32) -> S {
    let mut old: S = (*a.upgrade().deref()).clone();
    field!(a, v).with_mut(|__v| __v.prefix_dec());
    return old;
}
pub fn operator_add_11(a: Ptr<S>, mut b: i32) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } + { b }),
    };
}
pub fn operator_add_12(mut a: i32, b: Ptr<S>) -> S {
    return S {
        v: ({ a } + { b.with(|__s| __s.v) }),
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 7 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_add_0(_a, b.as_pointer())
        })
        .v
    } == 9)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a + b).v == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_sub_1(_a, b.as_pointer())
        })
        .v
    } == 5)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a - b).v == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_mul_2(_a, b.as_pointer())
        })
        .v
    } == 14)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a * b).v == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_div_3(_a, b.as_pointer())
        })
        .v
    } == 3)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a / b).v == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_rem_4(_a, b.as_pointer())
        })
        .v
    } == 1)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a % b).v == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_pos_5(_a)
        })
        .v
    } == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(+a).v == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_neg_6(_a)
        })
        .v
    } == -7_i32)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(-a).v == -7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: Ptr<S> = a.as_pointer();
        operator_inc_7(_a)
    })
    .with(|__s| (__s).v)
        == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(++a).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_post_inc_8(_a, 0)
        })
        .v
    } == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a++).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*a.borrow()).v } == 9) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"a.v == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({
        let _a: Ptr<S> = a.as_pointer();
        operator_dec_9(_a)
    })
    .with(|__s| (__s).v)
        == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(--a).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_post_dec_10(_a, 0)
        })
        .v
    } == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a--).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*a.borrow()).v } == 7) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"a.v == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _a: Ptr<S> = a.as_pointer();
            operator_add_11(_a, 1)
        })
        .v
    } == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(a + 1).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ operator_add_12(1, a.as_pointer()) }).v } == 8) {
        &(0);
    } else {
        ({
            __assert_fail_13(
                Ptr::<i8>::from_string_literal(b"(1 + a).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_13(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn __cpp2rust_init_globals() {}
