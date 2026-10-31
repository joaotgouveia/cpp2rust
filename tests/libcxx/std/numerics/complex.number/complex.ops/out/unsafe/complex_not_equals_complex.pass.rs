extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constexpr_0() {
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        const _: () = assert!(true, "lhs != rhs");;
    }
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        const _: () = assert!(true, "!(lhs != rhs)");;
    }
}
pub unsafe fn test_constexpr_1() {
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        const _: () = assert!(true, "lhs != rhs");;
    }
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        const _: () = assert!(true, "!(lhs != rhs)");;
    }
}
pub unsafe fn test_constexpr_2() {
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        const _: () = assert!(true, "lhs != rhs");;
    }
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        const _: () = assert!(true, "!(lhs != rhs)");;
    }
}
pub unsafe fn test_3() -> bool {
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        assert!(!(unimplemented!()));
    }
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        assert!(!(!(unimplemented!())));
    }
    (unsafe { test_constexpr_0() });
    return true;
}
pub unsafe fn test_4() -> bool {
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        assert!(!(unimplemented!()));
    }
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        assert!(!(!(unimplemented!())));
    }
    (unsafe { test_constexpr_1() });
    return true;
}
pub unsafe fn test_5() -> bool {
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        assert!(!(unimplemented!()));
    }
    {
        let lhs: () = unimplemented!();
        let rhs: () = unimplemented!();
        assert!(!(!(unimplemented!())));
    }
    (unsafe { test_constexpr_2() });
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
    (unsafe { test_4() });
    (unsafe { test_5() });
    const _: () = assert!(true, "test<float>()");;
    const _: () = assert!(true, "test<double>()");;
    const _: () = assert!(true, "test<long double>()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
