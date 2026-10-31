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
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
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
        let mut a: [i32; 5] = [1, 2, 3, 4, 5];
        let mut v: () = unimplemented!();
        let cv: *const () = &v;
        const _: () = assert!(
            true,
            "std::is_base_of<std::random_access_iterator_tag, std::iterator_traits<It>::iterator_category>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_base_of<std::random_access_iterator_tag, std::iterator_traits<CIt>::iterator_category>::value"
        );;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((&mut (*unimplemented!()) as *mut i32) == (&mut (*unimplemented!()) as *mut i32)));
        assert!(((&(*unimplemented!()) as *const i32) == (&(*unimplemented!()) as *const i32)));
        (*unimplemented!()) = 10;
        assert!(((*unimplemented!()) == (10)));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(
            ((&mut (*libcc2rs::prev_unsafe(unimplemented!(), 1_isize)) as *mut i32)
                == (&mut (*unimplemented!()) as *mut i32))
        );
        assert!(
            ((&(*libcc2rs::prev_unsafe(unimplemented!(), 1_isize)) as *const i32)
                == (&(*unimplemented!()) as *const i32))
        );
    }
    {
        let mut a: [i32; 5] = [1, 2, 3, 4, 5];
        let mut v: () = unimplemented!();
        let mut sum: i32 = 0;
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: *mut i32 = &mut (*i);
            sum += (*i);
        }
        assert!(((sum) == (15)));
    }
    {
        let mut a: [i32; 5] = [1, 2, 3, 4, 5];
        let cv: () = unimplemented!();
        let mut sum: i32 = 0;
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: *const i32 = &(*i);
            sum += (*i);
        }
        assert!(((sum) == (15)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
