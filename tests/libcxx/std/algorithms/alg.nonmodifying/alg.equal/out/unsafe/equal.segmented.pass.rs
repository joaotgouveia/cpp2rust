extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_containers_0() {
    {
        let mut lhs: Vec<i32> = Vec::from([1, 2, 3, 4]);
        let mut rhs: Vec<i32> = unimplemented!();
        let mut ret: bool = unimplemented!();
        assert!((ret as bool));
    }
    {
        let mut lhs: Vec<i32> = Vec::from([1, 2, 3, 4]);
        let mut rhs: Vec<i32> = unimplemented!();
        let mut ret: bool = unimplemented!();
        assert!(!(ret));
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        (*((&mut rhs).last_mut().unwrap())) = -1_i32;
        assert!(!(unimplemented!()));
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = Vec::new();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = Vec::new();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*((&mut rhs).last_mut().unwrap())) = -1_i32;
        assert!(!(unimplemented!()));
    }
}
pub unsafe fn test_containers_1() {
    {
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = Vec::from([1, 2, 3, 4]);
        let mut ret: bool = unimplemented!();
        assert!((ret as bool));
    }
    {
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = Vec::from([2, 2, 3, 4]);
        let mut ret: bool = unimplemented!();
        assert!(!(ret));
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = {
            let mut __v = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        assert!(unimplemented!());
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = {
            let mut __v = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        (*((&mut rhs).last_mut().unwrap())) = -1_i32;
        assert!(!(unimplemented!()));
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = Vec::new();
        {
            let __a0 = 10_usize as usize;
            rhs.resize_with(__a0, || 0_i32)
        };
        {
            let __off = <UnsafeRandomAccessIterator<i32>>::from(rhs.end())
                .iter_distance(&UnsafeRandomAccessIterator::Vec(rhs.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __tmp.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            rhs.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(rhs.as_mut_ptr().add(__off))
        };
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = Vec::new();
        {
            let __a0 = 10_usize as usize;
            rhs.resize_with(__a0, || 0_i32)
        };
        {
            let __off = <UnsafeRandomAccessIterator<i32>>::from(rhs.end())
                .iter_distance(&UnsafeRandomAccessIterator::Vec(rhs.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it = unimplemented!();
            while __it != unimplemented!() {
                __tmp.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            rhs.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(rhs.as_mut_ptr().add(__off))
        };
        unimplemented!();
        (*((&mut rhs).last_mut().unwrap())) = -1_i32;
        assert!(!(unimplemented!()));
    }
}
pub unsafe fn test_containers_2() {
    {
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut ret: bool = unimplemented!();
        assert!((ret as bool));
    }
    {
        let mut lhs: Vec<i32> = unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        let mut ret: bool = unimplemented!();
        assert!(!(ret));
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = unimplemented!();
        (*((&mut rhs).last_mut().unwrap())) = -1_i32;
        assert!(!(unimplemented!()));
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = Vec::new();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut lhs: Vec<i32> = Vec::new();
        unimplemented!();
        let mut rhs: Vec<i32> = Vec::new();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (*((&mut rhs).last_mut().unwrap())) = -1_i32;
        assert!(!(unimplemented!()));
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
    (unsafe { test_containers_0() });
    (unsafe { test_containers_1() });
    (unsafe { test_containers_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
