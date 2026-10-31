extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    {
        let mut ii1: UnsafeRandomAccessIterator<libc::c_char> =
            UnsafeRandomAccessIterator::default();
        let mut ii2: UnsafeRandomAccessIterator<libc::c_char> =
            UnsafeRandomAccessIterator::default();
        let mut ii4: UnsafeRandomAccessIterator<libc::c_char> = ii1.clone();
        let mut cii: UnsafeRandomAccessIterator<libc::c_char> =
            UnsafeRandomAccessIterator::default();
        assert!(ii1 == ii2);
        assert!(ii1 == ii4);
        assert!(!(ii1 != ii2));
        assert!((ii1 == cii));
        assert!((cii == ii1));
        assert!(!(ii1 != cii));
        assert!(!(cii != ii1));
        assert!(!(ii1 < (cii).clone().into()));
        assert!(!(cii < (ii1).clone().into()));
        assert!((ii1 <= (cii).clone().into()));
        assert!((cii <= (ii1).clone().into()));
        assert!(!(ii1 > (cii).clone().into()));
        assert!(!(cii > (ii1).clone().into()));
        assert!((ii1 >= (cii).clone().into()));
        assert!((cii >= (ii1).clone().into()));
        assert!(((cii.iter_distance(&(ii1).clone().into()) as i64) == (0_i64)));
        assert!(((ii1.iter_distance(&(cii).clone().into()) as i64) == (0_i64)));
    }
    {
        let mut a: Vec<libc::c_char> = vec![libc::c_char::default()];
        let mut i1: UnsafeRandomAccessIterator<libc::c_char> =
            UnsafeRandomAccessIterator::Vec(a.as_mut_ptr());
        let mut i2: UnsafeRandomAccessIterator<libc::c_char> =
            UnsafeRandomAccessIterator::default();
        i2 = i1.clone();
        assert!(i1 == i2);
    }
}
pub unsafe fn test_1() {
    {
        let mut ii1: UnsafeRandomAccessIterator<i32> = UnsafeRandomAccessIterator::default();
        let mut ii2: UnsafeRandomAccessIterator<i32> = UnsafeRandomAccessIterator::default();
        let mut ii4: UnsafeRandomAccessIterator<i32> = ii1.clone();
        let mut cii: UnsafeRandomAccessIterator<i32> = UnsafeRandomAccessIterator::default();
        assert!(ii1 == ii2);
        assert!(ii1 == ii4);
        assert!(!(ii1 != ii2));
        assert!((ii1 == cii));
        assert!((cii == ii1));
        assert!(!(ii1 != cii));
        assert!(!(cii != ii1));
        assert!(!(ii1 < (cii).clone().into()));
        assert!(!(cii < (ii1).clone().into()));
        assert!((ii1 <= (cii).clone().into()));
        assert!((cii <= (ii1).clone().into()));
        assert!(!(ii1 > (cii).clone().into()));
        assert!(!(cii > (ii1).clone().into()));
        assert!((ii1 >= (cii).clone().into()));
        assert!((cii >= (ii1).clone().into()));
        assert!(((cii.iter_distance(&(ii1).clone().into()) as i64) == (0_i64)));
        assert!(((ii1.iter_distance(&(cii).clone().into()) as i64) == (0_i64)));
    }
    {
        let mut a: Vec<i32> = vec![i32::default()];
        let mut i1: UnsafeRandomAccessIterator<i32> =
            UnsafeRandomAccessIterator::Vec(a.as_mut_ptr());
        let mut i2: UnsafeRandomAccessIterator<i32> = UnsafeRandomAccessIterator::default();
        i2 = i1.clone();
        assert!(i1 == i2);
    }
}
pub unsafe fn test_2() {
    {
        let mut ii1: UnsafeRandomAccessIterator<u8> = UnsafeRandomAccessIterator::default();
        let mut ii2: UnsafeRandomAccessIterator<u8> = UnsafeRandomAccessIterator::default();
        let mut ii4: UnsafeRandomAccessIterator<u8> = ii1.clone();
        let mut cii: UnsafeRandomAccessIterator<u8> = UnsafeRandomAccessIterator::default();
        assert!(ii1 == ii2);
        assert!(ii1 == ii4);
        assert!(!(ii1 != ii2));
        assert!((ii1 == cii));
        assert!((cii == ii1));
        assert!(!(ii1 != cii));
        assert!(!(cii != ii1));
        assert!(!(ii1 < (cii).clone().into()));
        assert!(!(cii < (ii1).clone().into()));
        assert!((ii1 <= (cii).clone().into()));
        assert!((cii <= (ii1).clone().into()));
        assert!(!(ii1 > (cii).clone().into()));
        assert!(!(cii > (ii1).clone().into()));
        assert!((ii1 >= (cii).clone().into()));
        assert!((cii >= (ii1).clone().into()));
        assert!(((cii.iter_distance(&(ii1).clone().into()) as i64) == (0_i64)));
        assert!(((ii1.iter_distance(&(cii).clone().into()) as i64) == (0_i64)));
    }
    {
        let mut a: Vec<u8> = vec![u8::default()];
        let mut i1: UnsafeRandomAccessIterator<u8> =
            UnsafeRandomAccessIterator::Vec(a.as_mut_ptr());
        let mut i2: UnsafeRandomAccessIterator<u8> = UnsafeRandomAccessIterator::default();
        i2 = i1.clone();
        assert!(i1 == i2);
    }
}
pub unsafe fn test_3() {
    {
        let mut ii1: UnsafeRandomAccessIterator<u16> = UnsafeRandomAccessIterator::default();
        let mut ii2: UnsafeRandomAccessIterator<u16> = UnsafeRandomAccessIterator::default();
        let mut ii4: UnsafeRandomAccessIterator<u16> = ii1.clone();
        let mut cii: UnsafeRandomAccessIterator<u16> = UnsafeRandomAccessIterator::default();
        assert!(ii1 == ii2);
        assert!(ii1 == ii4);
        assert!(!(ii1 != ii2));
        assert!((ii1 == cii));
        assert!((cii == ii1));
        assert!(!(ii1 != cii));
        assert!(!(cii != ii1));
        assert!(!(ii1 < (cii).clone().into()));
        assert!(!(cii < (ii1).clone().into()));
        assert!((ii1 <= (cii).clone().into()));
        assert!((cii <= (ii1).clone().into()));
        assert!(!(ii1 > (cii).clone().into()));
        assert!(!(cii > (ii1).clone().into()));
        assert!((ii1 >= (cii).clone().into()));
        assert!((cii >= (ii1).clone().into()));
        assert!(((cii.iter_distance(&(ii1).clone().into()) as i64) == (0_i64)));
        assert!(((ii1.iter_distance(&(cii).clone().into()) as i64) == (0_i64)));
    }
    {
        let mut a: Vec<u16> = vec![u16::default()];
        let mut i1: UnsafeRandomAccessIterator<u16> =
            UnsafeRandomAccessIterator::Vec(a.as_mut_ptr());
        let mut i2: UnsafeRandomAccessIterator<u16> = UnsafeRandomAccessIterator::default();
        i2 = i1.clone();
        assert!(i1 == i2);
    }
}
pub unsafe fn test_4() {
    {
        let mut ii1: UnsafeRandomAccessIterator<u32> = UnsafeRandomAccessIterator::default();
        let mut ii2: UnsafeRandomAccessIterator<u32> = UnsafeRandomAccessIterator::default();
        let mut ii4: UnsafeRandomAccessIterator<u32> = ii1.clone();
        let mut cii: UnsafeRandomAccessIterator<u32> = UnsafeRandomAccessIterator::default();
        assert!(ii1 == ii2);
        assert!(ii1 == ii4);
        assert!(!(ii1 != ii2));
        assert!((ii1 == cii));
        assert!((cii == ii1));
        assert!(!(ii1 != cii));
        assert!(!(cii != ii1));
        assert!(!(ii1 < (cii).clone().into()));
        assert!(!(cii < (ii1).clone().into()));
        assert!((ii1 <= (cii).clone().into()));
        assert!((cii <= (ii1).clone().into()));
        assert!(!(ii1 > (cii).clone().into()));
        assert!(!(cii > (ii1).clone().into()));
        assert!((ii1 >= (cii).clone().into()));
        assert!((cii >= (ii1).clone().into()));
        assert!(((cii.iter_distance(&(ii1).clone().into()) as i64) == (0_i64)));
        assert!(((ii1.iter_distance(&(cii).clone().into()) as i64) == (0_i64)));
    }
    {
        let mut a: Vec<u32> = vec![u32::default()];
        let mut i1: UnsafeRandomAccessIterator<u32> =
            UnsafeRandomAccessIterator::Vec(a.as_mut_ptr());
        let mut i2: UnsafeRandomAccessIterator<u32> = UnsafeRandomAccessIterator::default();
        i2 = i1.clone();
        assert!(i1 == i2);
    }
}
pub unsafe fn test_5() -> bool {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
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
