extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_none_0() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((0_u64) == (0_u64)) as i32)));
    {}
}
pub unsafe fn test_none_1() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((1_u64) == (0_u64)) as i32)));
    {}
}
pub unsafe fn test_none_2() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((31_u64) == (0_u64)) as i32)));
    {
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_none_3() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((32_u64) == (0_u64)) as i32)));
    {
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_none_4() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((33_u64) == (0_u64)) as i32)));
    {
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_none_5() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((63_u64) == (0_u64)) as i32)));
    {
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_none_6() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((64_u64) == (0_u64)) as i32)));
    {
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_none_7() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((65_u64) == (0_u64)) as i32)));
    {
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_none_8() {
    let mut v: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!() as i32) == (true as i32)));
    unimplemented!();
    assert!(((unimplemented!() as i32) == (((1000_u64) == (0_u64)) as i32)));
    {
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        unimplemented!();
        unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_9() -> bool {
    (unsafe { test_none_0() });
    (unsafe { test_none_1() });
    (unsafe { test_none_2() });
    (unsafe { test_none_3() });
    (unsafe { test_none_4() });
    (unsafe { test_none_5() });
    (unsafe { test_none_6() });
    (unsafe { test_none_7() });
    (unsafe { test_none_8() });
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
    (unsafe { test_9() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
