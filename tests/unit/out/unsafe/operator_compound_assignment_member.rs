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
impl S {
    pub unsafe fn operator_assign_1(&mut self, mut n: ()) -> *mut S {
        self.v = n;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_add_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_add((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_sub_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_sub((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_mul_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_mul((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_div_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_div((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_rem_assign(&mut self, o: *const S) -> *mut S {
        self.v = (self.v).wrapping_rem((*o).v);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitand_assign(&mut self, o: *const S) -> *mut S {
        self.v &= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitor_assign(&mut self, o: *const S) -> *mut S {
        self.v |= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_bitxor_assign(&mut self, o: *const S) -> *mut S {
        self.v ^= (*o).v;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_shl_assign(&mut self, mut n: ()) -> *mut S {
        self.v <<= n;
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_shr_assign(&mut self, mut n: ()) -> *mut S {
        self.v >>= n;
        return &mut (*(self as *mut S));
    }
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
    (unsafe { S::operator_add_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_sub_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_mul_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_div_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_rem_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_bitor_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_bitand_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_bitxor_assign(&mut a, &b) });
    unimplemented!();
    (unsafe { S::operator_assign_1(&mut a, 3_()) });
    unimplemented!();
    (unsafe { S::operator_shl_assign(&mut a, 2_()) });
    unimplemented!();
    (unsafe { S::operator_shr_assign(&mut a, 1_()) });
    unimplemented!();
    (unsafe {
        let _o: *const S = &b;
        S::operator_add_assign(&mut (*(unsafe { S::operator_add_assign(&mut a, &b) })), _o)
    });
    unimplemented!();
    let mut c: S = S { v: 0_() };
    c = (*(unsafe { S::operator_assign_1(&mut a, 1_()) }));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
