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
    pub unsafe fn operator_bitnot(&self) -> S {
        return S { v: !self.v };
    }
    pub unsafe fn operator_bitand(&self, o: *const S) -> S {
        return S {
            v: ((self.v) & ((*o).v)),
        };
    }
    pub unsafe fn operator_bitor(&self, o: *const S) -> S {
        return S {
            v: ((self.v) | ((*o).v)),
        };
    }
    pub unsafe fn operator_bitxor(&self, o: *const S) -> S {
        return S {
            v: ((self.v) ^ ((*o).v)),
        };
    }
    pub unsafe fn operator_shl(&self, mut n: ()) -> S {
        return S {
            v: ((self.v) << (n)),
        };
    }
    pub unsafe fn operator_shr(&self, mut n: ()) -> S {
        return S {
            v: ((self.v) >> (n)),
        };
    }
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
