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
pub struct Outer_RunInfo {
    pub block_idx: (),
    pub num_extra_zero_runs: (),
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub runs: std_vector_Outer_RunInfo__std_allocator_Outer_RunInfo__,
}
impl Outer {
    pub unsafe fn move_from(_a0: *mut Outer) -> Self {
        let mut this = Self { runs: (*_a0).runs };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        self.runs = ((*_a0).runs).clone();
        return &mut (*(self as *mut Outer));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut o: Outer = <Outer>::default();
    let mut info: Outer_RunInfo = <Outer_RunInfo>::default();
    info.block_idx = 1_();
    info.num_extra_zero_runs = 2_();
    (unsafe { o.runs.push_back_57(&info) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
