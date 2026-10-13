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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 7 }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2 }));
    (if ({ ({ SImpl::operator_add_1(&a.as_pointer(), b.as_pointer()) }).v } == 9) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a + b).v == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_sub_2(&a.as_pointer(), b.as_pointer()) }).v } == 5) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a - b).v == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_mul(&a.as_pointer(), b.as_pointer()) }).v } == 14) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a * b).v == 14"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_div(&a.as_pointer(), b.as_pointer()) }).v } == 3) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a / b).v == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_rem(&a.as_pointer(), b.as_pointer()) }).v } == 1) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(a % b).v == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_pos_6(&a.as_pointer()) }).v } == 7) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(+a).v == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_neg_7(&a.as_pointer()) }).v } == -7_i32) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(-a).v == -7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_inc_8(&a.as_pointer()) }).with(|__s| (__s).v) == 8) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(++a).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_post_inc_9(&a.as_pointer(), 0) }).v } == 8) {
        &(0);
    } else {
        ({
            __assert_fail_0(
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
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"a.v == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_dec_10(&a.as_pointer()) }).with(|__s| (__s).v) == 8) {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(--a).v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ ({ SImpl::operator_post_dec_11(&a.as_pointer(), 0) }).v } == 8) {
        &(0);
    } else {
        ({
            __assert_fail_0(
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
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"a.v == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ SImpl::operator_inc_8(&({ SImpl::operator_inc_8(&a.as_pointer()) })) })
        .with(|__s| (__s).v)
        == 9)
    {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(++(++a)).v == 9"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({
        ({
            let _o: Value<S> = Rc::new(RefCell::new(S { v: 4 }));
            SImpl::operator_add_1(
                &Rc::new(RefCell::new(S { v: 3 })).as_pointer(),
                _o.as_pointer(),
            )
        })
        .v
    } == 7)
    {
        &(0);
    } else {
        ({
            __assert_fail_0(
                Ptr::<i8>::from_string_literal(b"(S{3} + S{4}).v == 7"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
pub fn __assert_fail_0(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait SImpl {
    fn operator_add_1(&self, o: Ptr<S>) -> S;
    fn operator_sub_2(&self, o: Ptr<S>) -> S;
    fn operator_mul(&self, o: Ptr<S>) -> S;
    fn operator_div(&self, o: Ptr<S>) -> S;
    fn operator_rem(&self, o: Ptr<S>) -> S;
    fn operator_pos_6(&self) -> S;
    fn operator_neg_7(&self) -> S;
    fn operator_inc_8(&self) -> Ptr<S>;
    fn operator_post_inc_9(&self, _a0: i32) -> S;
    fn operator_dec_10(&self) -> Ptr<S>;
    fn operator_post_dec_11(&self, _a0: i32) -> S;
}
impl SImpl for Ptr<S> {
    fn operator_add_1(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } + { o.with(|__s| __s.v) }),
        };
    }
    fn operator_sub_2(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }),
        };
    }
    fn operator_mul(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } * { o.with(|__s| __s.v) }),
        };
    }
    fn operator_div(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } / { o.with(|__s| __s.v) }),
        };
    }
    fn operator_rem(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } % { o.with(|__s| __s.v) }),
        };
    }
    fn operator_pos_6(&self) -> S {
        return S {
            v: (*self).with(|__s| __s.v),
        };
    }
    fn operator_neg_7(&self) -> S {
        return S {
            v: -(*self).with(|__s| __s.v),
        };
    }
    fn operator_inc_8(&self) -> Ptr<S> {
        field!((*self), v).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_9(&self, mut _a0: i32) -> S {
        let mut old: S = (*(*self).upgrade().deref()).clone();
        field!((*self), v).with_mut(|__v| __v.prefix_inc());
        return old;
    }
    fn operator_dec_10(&self) -> Ptr<S> {
        field!((*self), v).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_11(&self, mut _a0: i32) -> S {
        let mut old: S = (*(*self).upgrade().deref()).clone();
        field!((*self), v).with_mut(|__v| __v.prefix_dec());
        return old;
    }
}
pub fn __cpp2rust_init_globals() {}
