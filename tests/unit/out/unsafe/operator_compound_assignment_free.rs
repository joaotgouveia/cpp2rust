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
pub unsafe fn operator_add_assign_0(a: *mut S, b: *const S) -> *mut S {
    (*a).v = ((*a).v).wrapping_add((*b).v);
    return a;
}
pub unsafe fn operator_sub_assign_1(a: *mut S, b: *const S) -> *mut S {
    (*a).v = ((*a).v).wrapping_sub((*b).v);
    return a;
}
pub unsafe fn operator_mul_assign_2(a: *mut S, b: *const S) -> *mut S {
    (*a).v = ((*a).v).wrapping_mul((*b).v);
    return a;
}
pub unsafe fn operator_div_assign_3(a: *mut S, b: *const S) -> *mut S {
    (*a).v = ((*a).v).wrapping_div((*b).v);
    return a;
}
pub unsafe fn operator_rem_assign_4(a: *mut S, b: *const S) -> *mut S {
    (*a).v = ((*a).v).wrapping_rem((*b).v);
    return a;
}
pub unsafe fn operator_bitand_assign_5(a: *mut S, b: *const S) -> *mut S {
    (*a).v &= (*b).v;
    return a;
}
pub unsafe fn operator_bitor_assign_6(a: *mut S, b: *const S) -> *mut S {
    (*a).v |= (*b).v;
    return a;
}
pub unsafe fn operator_bitxor_assign_7(a: *mut S, b: *const S) -> *mut S {
    (*a).v ^= (*b).v;
    return a;
}
pub unsafe fn operator_shl_assign_8(a: *mut S, mut n: ()) -> *mut S {
    (*a).v <<= n;
    return a;
}
pub unsafe fn operator_shr_assign_9(a: *mut S, mut n: ()) -> *mut S {
    (*a).v >>= n;
    return a;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: S = S { v: 6_() };
    let mut b: S = S { v: 4_() };
    (unsafe {
        let _a: *mut S = &mut a;
        operator_add_assign_0(_a, &b)
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_sub_assign_1(_a, &b)
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_mul_assign_2(_a, &b)
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_div_assign_3(_a, &b)
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_rem_assign_4(_a, &b)
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_bitor_assign_6(_a, &b)
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_bitand_assign_5(_a, &b)
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_bitxor_assign_7(_a, &b)
    });
    unimplemented!();
    a.v = 3_();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_shl_assign_8(_a, 2_())
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut a;
        operator_shr_assign_9(_a, 1_())
    });
    unimplemented!();
    (unsafe {
        let _a: *mut S = &mut (*(unsafe {
            let _a: *mut S = &mut a;
            operator_add_assign_0(_a, &b)
        }));
        let _b: *const S = &b;
        operator_add_assign_0(_a, _b)
    });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
