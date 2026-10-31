extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub unsafe fn test_1() -> bool {
    {
        let mut il: Vec<i32> = Vec::from([3, 2, 1]);
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
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
    }
    {
        let mut il: Vec<i32> = Vec::from([1, 2, 3]);
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
        let cil: *const Vec<i32> = &il;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
