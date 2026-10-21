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
    let a: Value<S> = Rc::new(RefCell::new(S { v: 12_() }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 10_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn operator_bitnot(&self) -> S;
    fn operator_bitand(&self, o: Ptr<S>) -> S;
    fn operator_bitor(&self, o: Ptr<S>) -> S;
    fn operator_bitxor(&self, o: Ptr<S>) -> S;
    fn operator_shl(&self, n: ()) -> S;
    fn operator_shr(&self, n: ()) -> S;
}
impl SImpl for Ptr<S> {
    fn operator_bitnot(&self) -> S {
        return S {
            v: !(*self).with(|__s| __s.v),
        };
    }
    fn operator_bitand(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } & { o.with(|__s| __s.v) }),
        };
    }
    fn operator_bitor(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } | { o.with(|__s| __s.v) }),
        };
    }
    fn operator_bitxor(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ (*self).with(|__s| __s.v) } ^ { o.with(|__s| __s.v) }),
        };
    }
    fn operator_shl(&self, mut n: ()) -> S {
        return S {
            v: ((*self).with(|__s| __s.v) << n),
        };
    }
    fn operator_shr(&self, mut n: ()) -> S {
        return S {
            v: ((*self).with(|__s| __s.v) >> n),
        };
    }
}
pub fn __cpp2rust_init_globals() {}
