extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NonCopyable {}
impl NonCopyable {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NonMovable {}
impl NonMovable {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn return_int_0() -> () {
    return unimplemented!();
}
pub unsafe fn return_noncopyable_1() -> () {
    return unimplemented!();
}
pub unsafe fn return_nonmovable_2() -> () {
    return unimplemented!();
}
const _: () = assert!(
    true,
    "has_and_then<std::expected<void, int>&, decltype(return_int)>"
);
const _: () = assert!(
    !(false),
    "!has_and_then<std::expected<void, NonCopyable>&, decltype(return_noncopyable)>"
);
const _: () = assert!(
    true,
    "has_and_then<const std::expected<void, int>&, decltype(return_int)>"
);
const _: () = assert!(
    !(false),
    "!has_and_then<const std::expected<void, NonCopyable>&, decltype(return_noncopyable)>"
);
const _: () = assert!(
    true,
    "has_and_then<std::expected<void, int>&&, decltype(return_int)>"
);
const _: () = assert!(
    !(false),
    "!has_and_then<std::expected<void, NonMovable>&&, decltype(return_nonmovable)>"
);
const _: () = assert!(
    true,
    "has_and_then<const std::expected<void, int>&&, decltype(return_int)>"
);
const _: () = assert!(
    !(false),
    "!has_and_then<const std::expected<void, NonMovable>&&, decltype(return_nonmovable)>"
);
const _: () = assert!(
    !(false),
    "!has_and_then<const std::expected<int, std::unique_ptr<int>>&, int()>"
);
const _: () = assert!(
    !(false),
    "!has_and_then<const std::expected<int, std::unique_ptr<int>>&&, int()>"
);
pub unsafe fn test_val_types_3() {
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return unimplemented!();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return unimplemented!();
            }
        });
        let v: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
        const _: () = assert!(
            true,
            "std::is_same_v< decltype(v.and_then(l)), std::expected<int, int>>"
        );;
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return unimplemented!();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut l: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                return unimplemented!();
            }
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(unimplemented!());
    }
}
pub unsafe fn test_fail_4() {
    {
        let mut f: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                assert!(false);
                return unimplemented!();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
    {
        let mut f: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                assert!(false);
                return unimplemented!();
            }
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
    {
        let mut f: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                assert!(false);
                return unimplemented!();
            }
        });
        let mut v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
    {
        let mut f: FnPtr<fn() -> ()> = FnPtr::<fn() -> ()>::new(|| -> () {
            unsafe {
                assert!(false);
                return unimplemented!();
            }
        });
        let v: () = unimplemented!();
        let mut val: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
}
pub unsafe fn test_5() -> bool {
    (unsafe { test_fail_4() });
    (unsafe { test_val_types_3() });
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
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
