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
pub fn operator_shl_4(a: Ptr<S>, mut n: ()) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } << { n }),
    };
}
pub fn operator_shr_5(a: Ptr<S>, mut n: ()) -> S {
    return S {
        v: ({ a.with(|__s| __s.v) } >> { n }),
    };
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
pub fn __cpp2rust_init_globals() {}
