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
    pub unsafe fn operator_add_1(&self, o: *const S) -> S {
        return S {
            v: ((self.v) + ((*o).v)),
        };
    }
    pub unsafe fn operator_sub_2(&self, o: *const S) -> S {
        return S {
            v: ((self.v) - ((*o).v)),
        };
    }
    pub unsafe fn operator_mul(&self, o: *const S) -> S {
        return S {
            v: ((self.v) * ((*o).v)),
        };
    }
    pub unsafe fn operator_div(&self, o: *const S) -> S {
        return S {
            v: ((self.v) / ((*o).v)),
        };
    }
    pub unsafe fn operator_rem(&self, o: *const S) -> S {
        return S {
            v: ((self.v) % ((*o).v)),
        };
    }
    pub unsafe fn operator_pos_6(&self) -> S {
        return S { v: self.v };
    }
    pub unsafe fn operator_neg_7(&self) -> S {
        return S { v: -self.v };
    }
    pub unsafe fn operator_inc_8(&mut self) -> *mut S {
        self.v.prefix_inc();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_post_inc_9(&mut self, mut _a0: ()) -> S {
        let mut old: S = (*(self as *mut S));
        self.v.prefix_inc();
        return old;
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut S {
        self.v.prefix_dec();
        return &mut (*(self as *mut S));
    }
    pub unsafe fn operator_post_dec_11(&mut self, mut _a0: ()) -> S {
        let mut old: S = (*(self as *mut S));
        self.v.prefix_dec();
        return old;
    }
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
