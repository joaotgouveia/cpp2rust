extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub v: (),
}
pub unsafe fn operator_bitnot_0(a: *const S) -> S {
    return S { v: !(*a).v };
}
pub unsafe fn operator_bitand_1(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) & ((*b).v)),
    };
}
pub unsafe fn operator_bitor_2(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) | ((*b).v)),
    };
}
pub unsafe fn operator_bitxor_3(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) ^ ((*b).v)),
    };
}
pub unsafe fn operator_shl_4(a: *const S, mut n: ()) -> S {
    return S {
        v: (((*a).v) << (n)),
    };
}
pub unsafe fn operator_shr_5(a: *const S, mut n: ()) -> S {
    return S {
        v: (((*a).v) >> (n)),
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: S = S { v: 12_() };
    let mut b: S = S { v: 10_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
