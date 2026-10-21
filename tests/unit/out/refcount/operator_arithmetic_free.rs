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
    pub v: (),
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
pub fn operator_post_inc_8(a: Ptr<S>, mut _a1: ()) -> S {
    let mut old: S = (*a.upgrade().deref()).clone();
    field!(a, v).with_mut(|__v| __v.prefix_inc());
    return old;
}
pub fn operator_dec_9(a: Ptr<S>) -> Ptr<S> {
    field!(a, v).with_mut(|__v| __v.prefix_dec());
    return (a).clone();
}
pub fn operator_post_dec_10(a: Ptr<S>, mut _a1: ()) -> S {
    let mut old: S = (*a.upgrade().deref()).clone();
    field!(a, v).with_mut(|__v| __v.prefix_dec());
    return old;
}
pub fn operator_add_11(a: Ptr<S>, mut b: ()) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } + { b }),
    };
}
pub fn operator_add_12(mut a: (), b: Ptr<S>) -> S {
    return S {
        v: ({ a } + { b.with(|__s| __s.v) }),
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 7_() }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
