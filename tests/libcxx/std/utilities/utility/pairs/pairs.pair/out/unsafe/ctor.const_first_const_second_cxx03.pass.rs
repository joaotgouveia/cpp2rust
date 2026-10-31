extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct A {
    data_: i32,
}
impl A {
    pub unsafe fn new(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_eq(&self, a: *const A) -> bool {
        return ((self.data_) == ((*a).data_));
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { A::operator_eq(self, other as *const A) }
    }
}
impl std::cmp::Eq for A {}
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut p: (f32, *mut i16) = unimplemented!();
        assert!(((p.0) == (3.5_f32)));
        assert!((p.1).is_null());
    }
    {
        let mut p: (A, i32) = (A::new({ 1 }), 2);
        assert!(
            (unsafe {
                let mut _a: A = A::new({ 1 });
                A::operator_eq(&p.0, &mut _a)
            })
        );
        assert!(((p.1) == (2)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
