extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut g_cursor_0: std::cell::LazyCell<*mut *mut ()> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub unsafe fn advance_1() -> () {
    (*(*std::cell::LazyCell::force_mut(&mut *&raw mut g_cursor_0))).prefix_inc();
    return 10_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub ptr: *mut (),
}
pub unsafe fn by_ref_2(r: *mut *mut ()) {
    (*(*r).offset((1_()) as isize)) += (unsafe { advance_1() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: [(); 8] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_()];
    let mut q: *mut () = a.as_mut_ptr();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut g_cursor_0)) = (&mut q as *mut *mut ());
    (*q.offset((1_()) as isize)) = (unsafe { advance_1() });
    unimplemented!();
    (*q.offset((1_()) as isize)) += (unsafe { advance_1() });
    unimplemented!();
    let mut pq: *mut *mut () = (&mut q as *mut *mut ());
    (*(*pq).offset((1_()) as isize)) += (unsafe { advance_1() });
    unimplemented!();
    (*(*pq).offset((1_()) as isize)) = (unsafe { advance_1() });
    unimplemented!();
    (unsafe { by_ref_2(&mut q) });
    unimplemented!();
    let mut s: S = S {
        ptr: a.as_mut_ptr(),
    };
    (*std::cell::LazyCell::force_mut(&mut *&raw mut g_cursor_0)) = (&mut s.ptr as *mut *mut ());
    (*s.ptr.offset((1_()) as isize)) += (unsafe { advance_1() });
    unimplemented!();
    let mut sp: *mut S = (&mut s as *mut S);
    (*(*sp).ptr.offset((1_()) as isize)) += (unsafe { advance_1() });
    unimplemented!();
    let mut b: *mut () = (a.as_mut_ptr() as *mut ());
    (*b.offset((0_()) as isize)) = 7_();
    (*b.offset((4_()) as isize)) += 1_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_cursor_0);
}
