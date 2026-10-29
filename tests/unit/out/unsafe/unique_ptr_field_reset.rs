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
pub struct Data {
    pub v: (),
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Holder {
    pub data: std_unique_ptr_Data__std_default_delete_Data__,
    pub n: (),
}
impl Holder {
    pub unsafe fn set(&mut self, mut p: *mut Data) {
        (unsafe { self.data.reset(p) });
    }
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            data: (*_a0).data,
            n: (*_a0).n,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.data = ((*_a0).data).clone();
        self.n = (*_a0).n;
        return &mut (*(self as *mut Holder));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut h: Holder = <Holder>::default();
    h.n = 1_();
    let mut hp: *mut Holder = (&mut h as *mut Holder);
    (unsafe {
        (*hp)
            .data
            .reset((Box::leak(Box::new(Data { v: 3_() })) as *mut Data))
    });
    unimplemented!();
    (unsafe { Holder::set(&mut h, (Box::leak(Box::new(Data { v: 4_() })) as *mut Data)) });
    unimplemented!();
    (*(*hp).data).v += (*hp).n;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
