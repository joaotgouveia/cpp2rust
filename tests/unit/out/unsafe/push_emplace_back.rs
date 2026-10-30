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
pub struct Chunk {
    pub data: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Writer {
    pub output: *mut (),
    pub chunk: Chunk,
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct JPEGData {
    pub com_data: (),
    pub app_data: (),
}
impl JPEGData {
    pub unsafe fn move_from(_a0: *mut JPEGData) -> Self {
        let mut this = Self {
            com_data: unimplemented!(),
            app_data: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut JPEGData) -> *mut JPEGData {
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut JPEGData));
    }
}
pub unsafe fn push_param_0(mut dest: *mut ()) {
    unimplemented!();
}
pub unsafe fn push_local_from_field_1(mut jpg: *mut JPEGData, mut cond: ()) {
    let mut head: [(); 3] = [1_(), 2_(), 3_()];
    let mut dest: *mut () = std::ptr::null_mut();
    if cond {
        dest = (&mut (*jpg).com_data as *mut ());
    } else {
        dest = (&mut (*jpg).app_data as *mut ());
    }
    unimplemented!();
}
pub unsafe fn shrink_through_ptr_2(mut comps: *mut ()) {
    unimplemented!();
}
pub unsafe fn nested_push_move_3(mut bw: *mut Writer) {
    unimplemented!();
}
pub unsafe fn emplace_local_from_field_4(mut jpg: *mut JPEGData, mut cond: ()) {
    let mut head: [(); 3] = [1_(), 2_(), 3_()];
    let mut dest: *mut () = std::ptr::null_mut();
    if cond {
        dest = (&mut (*jpg).com_data as *mut ());
    } else {
        dest = (&mut (*jpg).app_data as *mut ());
    }
    unimplemented!();
}
pub unsafe fn nested_emplace_move_5(mut bw: *mut Writer) {
    unimplemented!();
}
pub unsafe fn self_ref_push_6(mut comps: *mut ()) {
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Pair {
    pub first: (),
    pub second: (),
}
impl Pair {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            first: -1_(),
            second: -1_(),
        };
        this
    }
    pub unsafe fn new_2(mut a: ()) -> Self {
        let mut this = Self {
            first: a,
            second: 0_(),
        };
        this
    }
    pub unsafe fn new_3(mut a: (), mut b: ()) -> Self {
        let mut this = Self {
            first: a,
            second: ((b) * (2_())),
        };
        this
    }
}
impl Default for Pair {
    fn default() -> Self {
        unsafe { Pair::new_1() }
    }
}
pub unsafe fn emplace_ctor_args_7(mut pairs: *mut ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn emplace_deque_8(mut queue: *mut ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn emplace_scalar_9(mut values: *mut (), mut x: ()) {
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut vecs: () = unimplemented!();
    (unsafe { push_param_0((&mut vecs as *mut ())) });
    unimplemented!();
    unimplemented!();
    let mut jpg: JPEGData = <JPEGData>::default();
    (unsafe { push_local_from_field_1((&mut jpg as *mut JPEGData), true) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut chunks: () = unimplemented!();
    (unsafe { shrink_through_ptr_2((&mut chunks as *mut ())) });
    unimplemented!();
    let mut w: Writer = <Writer>::default();
    w.chunk.data = 42_();
    w.output = (&mut chunks as *mut ());
    (unsafe { nested_push_move_3((&mut w as *mut Writer)) });
    unimplemented!();
    unimplemented!();
    (unsafe { emplace_local_from_field_4((&mut jpg as *mut JPEGData), false) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    w.chunk.data = 99_();
    w.output = (&mut chunks as *mut ());
    (unsafe { nested_emplace_move_5((&mut w as *mut Writer)) });
    unimplemented!();
    unimplemented!();
    (unsafe { self_ref_push_6((&mut chunks as *mut ())) });
    unimplemented!();
    unimplemented!();
    let mut pairs: () = unimplemented!();
    (unsafe { emplace_ctor_args_7((&mut pairs as *mut ())) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut queue: () = unimplemented!();
    (unsafe { emplace_deque_8((&mut queue as *mut ())) });
    unimplemented!();
    unimplemented!();
    let mut values: () = unimplemented!();
    (unsafe { emplace_scalar_9((&mut values as *mut ()), 7_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
