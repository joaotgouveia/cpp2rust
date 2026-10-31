extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut a: Vec<u32> = Vec::new();
    {
        let __a1 = 0_u32;
        a.push(__a1)
    };
    {
        let __a1 = 1_u32;
        a.push(__a1)
    };
    {
        let __a1 = 2_u32;
        a.push(__a1)
    };
    {
        let __a1 = 3_u32;
        a.push(__a1)
    };
    {
        let __a1 = 4_u32;
        a.push(__a1)
    };
    if ((a.capacity()) < (7_usize)) {
        if 7_usize as usize > a.capacity() as usize {
            let len_0 = a.len();
            a.reserve_exact(7_usize as usize - len_0 as usize);
        };
    }
    {
        let mut __first = a.begin();
        let __last = a.begin().iter_add(2_isize);
        let mut __out = unimplemented!();
        while __first != __last {
            *__out.iter_deref() = <Vec<u32> as CppFromUnsafe<u32>>::cpp_from(__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
