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
#[cpp_from(Vec<i32 > , new, value)]
pub struct A {}
impl A {
    pub unsafe fn new(mut il: Vec<i32>) -> Self {
        let mut this = Self {};
        let mut b: *const i32 = unimplemented!();
        let mut e: *const i32 = unimplemented!();
        assert!(((il.len()) == (3_usize)));
        assert!(
            (((((e as usize - b as usize) / ::std::mem::size_of::<i32>()) as i64) as usize)
                == (il.len()))
        );
        assert!(((*b.postfix_inc()) == (3)));
        assert!(((*b.postfix_inc()) == (2)));
        assert!(((*b.postfix_inc()) == (1)));
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Vec<i32 > , new, value)]
pub struct B {}
impl B {
    pub unsafe fn new(mut il: Vec<i32>) -> Self {
        let mut this = Self {};
        let mut b: *const i32 = unimplemented!();
        let mut e: *const i32 = unimplemented!();
        assert!(((il.len()) == (3_usize)));
        assert!(
            (((((e as usize - b as usize) / ::std::mem::size_of::<i32>()) as i64) as usize)
                == (il.len()))
        );
        assert!(((*b.postfix_inc()) == (3)));
        assert!(((*b.postfix_inc()) == (2)));
        assert!(((*b.postfix_inc()) == (1)));
        this
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    let mut test1: A = A::new({ Vec::from([3, 2, 1]) });
    &(test1);
    let test2: B = B::new({ Vec::from([3, 2, 1]) });
    &(test2);
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
