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
pub unsafe fn operator_add_0(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) + ((*b).v)),
    };
}
pub unsafe fn operator_sub_1(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) - ((*b).v)),
    };
}
pub unsafe fn operator_mul_2(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) * ((*b).v)),
    };
}
pub unsafe fn operator_div_3(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) / ((*b).v)),
    };
}
pub unsafe fn operator_rem_4(a: *const S, b: *const S) -> S {
    return S {
        v: (((*a).v) % ((*b).v)),
    };
}
pub unsafe fn operator_pos_5(a: *const S) -> S {
    return S { v: (*a).v };
}
pub unsafe fn operator_neg_6(a: *const S) -> S {
    return S { v: -(*a).v };
}
pub unsafe fn operator_inc_7(a: *mut S) -> *mut S {
    (*a).v.prefix_inc();
    return a;
}
pub unsafe fn operator_post_inc_8(a: *mut S, mut _a1: ()) -> S {
    let mut old: S = (*a);
    (*a).v.prefix_inc();
    return old;
}
pub unsafe fn operator_dec_9(a: *mut S) -> *mut S {
    (*a).v.prefix_dec();
    return a;
}
pub unsafe fn operator_post_dec_10(a: *mut S, mut _a1: ()) -> S {
    let mut old: S = (*a);
    (*a).v.prefix_dec();
    return old;
}
pub unsafe fn operator_add_11(a: *const S, mut b: ()) -> S {
    return S {
        v: (((*a).v) + (b)),
    };
}
pub unsafe fn operator_add_12(mut a: (), b: *const S) -> S {
    return S {
        v: ((a) + ((*b).v)),
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: S = S { v: 7_() };
    let mut b: S = S { v: 2_() };
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
