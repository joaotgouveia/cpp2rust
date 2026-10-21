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
pub trait SImpl {
    fn operator_add_1(&self, o: Ptr<S>) -> S;
    fn operator_sub_2(&self, o: Ptr<S>) -> S;
    fn operator_mul(&self, o: Ptr<S>) -> S;
    fn operator_div(&self, o: Ptr<S>) -> S;
    fn operator_rem(&self, o: Ptr<S>) -> S;
    fn operator_pos_6(&self) -> S;
    fn operator_neg_7(&self) -> S;
    fn operator_inc_8(&self) -> Ptr<S>;
    fn operator_post_inc_9(&self, _a0: ()) -> S;
    fn operator_dec_10(&self) -> Ptr<S>;
    fn operator_post_dec_11(&self, _a0: ()) -> S;
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
    fn operator_post_inc_9(&self, mut _a0: ()) -> S {
        let mut old: S = (*(*self).upgrade().deref()).clone();
        field!((*self), v).with_mut(|__v| __v.prefix_inc());
        return old;
    }
    fn operator_dec_10(&self) -> Ptr<S> {
        field!((*self), v).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_11(&self, mut _a0: ()) -> S {
        let mut old: S = (*(*self).upgrade().deref()).clone();
        field!((*self), v).with_mut(|__v| __v.prefix_dec());
        return old;
    }
}
pub fn __cpp2rust_init_globals() {}
