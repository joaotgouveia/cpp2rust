extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut short_s1: Vec<libc::c_char> = vec![('a' as libc::c_char); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    let mut long_s1: Vec<libc::c_char> = vec![('c' as libc::c_char); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        short_s1.clear();
        short_s1.push((0 as libc::c_char))
    };
    {
        long_s1.clear();
        long_s1.push((0 as libc::c_char))
    };
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut short_s2: Vec<libc::c_char> = vec![('a' as libc::c_char); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    let mut long_s2: Vec<libc::c_char> = vec![('c' as libc::c_char); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_1() {
    let mut short_s1: Vec<i32> = vec![(('a' as libc::c_char) as i32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    let mut long_s1: Vec<i32> = vec![(('c' as libc::c_char) as i32); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        short_s1.clear();
        short_s1.push(0_i32)
    };
    {
        long_s1.clear();
        long_s1.push(0_i32)
    };
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut short_s2: Vec<i32> = vec![(('a' as libc::c_char) as i32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    let mut long_s2: Vec<i32> = vec![(('c' as libc::c_char) as i32); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_2() {
    let mut short_s1: Vec<u16> = vec![(('a' as libc::c_char) as u16); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    let mut long_s1: Vec<u16> = vec![(('c' as libc::c_char) as u16); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        short_s1.clear();
        short_s1.push(0_u16)
    };
    {
        long_s1.clear();
        long_s1.push(0_u16)
    };
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut short_s2: Vec<u16> = vec![(('a' as libc::c_char) as u16); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    let mut long_s2: Vec<u16> = vec![(('c' as libc::c_char) as u16); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_3() {
    let mut short_s1: Vec<u32> = vec![(('a' as libc::c_char) as u32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    let mut long_s1: Vec<u32> = vec![(('c' as libc::c_char) as u32); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        short_s1.clear();
        short_s1.push(0_u32)
    };
    {
        long_s1.clear();
        long_s1.push(0_u32)
    };
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut short_s2: Vec<u32> = vec![(('a' as libc::c_char) as u32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    let mut long_s2: Vec<u32> = vec![(('c' as libc::c_char) as u32); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_4() {
    let mut short_s1: Vec<u8> = vec![(('a' as libc::c_char) as u8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    let mut long_s1: Vec<u8> = vec![(('c' as libc::c_char) as u8); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        short_s1.clear();
        short_s1.push(0_u8)
    };
    {
        long_s1.clear();
        long_s1.push(0_u8)
    };
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut short_s2: Vec<u8> = vec![(('a' as libc::c_char) as u8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    let mut long_s2: Vec<u8> = vec![(('c' as libc::c_char) as u8); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
