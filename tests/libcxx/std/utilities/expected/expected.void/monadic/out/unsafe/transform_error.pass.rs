extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NonCopy {
    pub value: i32,
}
impl NonCopy {
    pub unsafe fn new(mut val: i32) -> Self {
        let mut this = Self { value: val };
        this
    }
}
pub unsafe fn test_val_types_0() {
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            return 1;
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
    }
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            return 1;
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
    }
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            return 1;
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
    }
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            return 1;
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
    }
}
pub unsafe fn test_fail_1() {
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            assert!(false);
            return 0;
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            assert!(false);
            return 0;
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            assert!(false);
            return 0;
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut l: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> i32 {
            assert!(false);
            return 0;
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(unimplemented!());
    }
}
pub unsafe fn test_direct_non_list_init_2() {
    let mut x: FnPtr<fn(i32) -> NonCopy> =
        FnPtr::<fn(i32) -> NonCopy>::new(|mut i: i32| -> NonCopy {
            unsafe {
                return NonCopy::new({ i });
            }
        });
    let mut v: () = unimplemented!();
    let mut nv: () = unimplemented!();
    assert!((((*unimplemented!()).value) == (2)));
}
pub unsafe fn test_3() -> bool {
    (unsafe { test_fail_1() });
    (unsafe { test_val_types_0() });
    (unsafe { test_direct_non_list_init_2() });
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
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
