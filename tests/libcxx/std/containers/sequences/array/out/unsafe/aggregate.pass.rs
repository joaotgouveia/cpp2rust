extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_aggregate_0() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_1() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_2() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_3() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_4() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_5() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_6() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_7() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub unsafe fn check_aggregate_8() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trivial {
    pub i: (),
    pub j: (),
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct NonTrivial {
    pub i: (),
    pub j: (),
}
impl NonTrivial {
    pub unsafe fn copy_from(_a0: *const NonTrivial) -> Self {
        let mut this = Self {
            i: unimplemented!(),
            j: unimplemented!(),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NonTrivial) -> *mut NonTrivial {
        self.i = (*_a0).i;
        self.j = (*_a0).j;
        return &mut (*(self as *mut NonTrivial));
    }
}
impl Clone for NonTrivial {
    fn clone(&self) -> Self {
        unsafe { NonTrivial::copy_from(self as *const NonTrivial) }
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { check_aggregate_0() });
    (unsafe { check_aggregate_1() });
    (unsafe { check_aggregate_2() });
    (unsafe { check_aggregate_3() });
    (unsafe { check_aggregate_4() });
    (unsafe { check_aggregate_5() });
    (unsafe { check_aggregate_6() });
    (unsafe { check_aggregate_7() });
    (unsafe { check_aggregate_8() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
