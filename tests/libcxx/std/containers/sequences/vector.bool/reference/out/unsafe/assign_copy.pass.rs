extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    let mut vec_: Vec<bool> = Vec::new();
    {
        let a0_clone = true.clone();
        vec_.push(a0_clone)
    };
    {
        let a0_clone = false.clone();
        vec_.push(a0_clone)
    };
    let mut ref1: bool = vec_[(0_usize)];
    let mut ref2: bool = vec_[(1_usize)];
    ref2 = ref1;
    {
        vec_[(0_usize)] = false;
        vec_[(1_usize)] = true;
        ref1 = ref2;
        assert!((vec_[(0_usize)] as bool));
        assert!((vec_[(1_usize)] as bool));
    }
    {
        vec_[(0_usize)] = true;
        vec_[(1_usize)] = false;
        ref1 = ref2;
        assert!(!(vec_[(0_usize)] as bool));
        assert!(!(vec_[(1_usize)] as bool));
    }
    {
        vec_[(0_usize)] = false;
        vec_[(1_usize)] = true;
        ref1 = ref2;
        assert!((vec_[(0_usize)] as bool));
        assert!((vec_[(1_usize)] as bool));
    }
    {
        vec_[(0_usize)] = true;
        vec_[(1_usize)] = false;
        ref1 = ref2;
        assert!(!(vec_[(0_usize)] as bool));
        assert!(!(vec_[(1_usize)] as bool));
    }
    {
        vec_[(0_usize)] = false;
        vec_[(1_usize)] = true;
        ref1 = ref2;
        assert!((vec_[(0_usize)] as bool));
        assert!((vec_[(1_usize)] as bool));
    }
    {
        vec_[(0_usize)] = true;
        vec_[(1_usize)] = false;
        ref1 = ref2;
        assert!(!(vec_[(0_usize)] as bool));
        assert!(!(vec_[(1_usize)] as bool));
    }
    return true;
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
