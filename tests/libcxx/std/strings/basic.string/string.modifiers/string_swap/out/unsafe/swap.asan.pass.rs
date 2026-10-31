extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(val: libc::c_char) {
    let mut empty_s: Vec<libc::c_char> = vec![libc::c_char::default()];
    let mut short_s: Vec<libc::c_char> = vec![val; (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    let mut long_s: Vec<libc::c_char> = vec![val; (1100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s2: Vec<libc::c_char> = vec![val; (11100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s3: Vec<libc::c_char> = vec![val; (111_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_1(val: i32) {
    let mut empty_s: Vec<i32> = vec![i32::default()];
    let mut short_s: Vec<i32> = vec![val; (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    let mut long_s: Vec<i32> = vec![val; (1100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s2: Vec<i32> = vec![val; (11100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s3: Vec<i32> = vec![val; (111_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_2(val: u16) {
    let mut empty_s: Vec<u16> = vec![u16::default()];
    let mut short_s: Vec<u16> = vec![val; (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    let mut long_s: Vec<u16> = vec![val; (1100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s2: Vec<u16> = vec![val; (11100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s3: Vec<u16> = vec![val; (111_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_3(val: u32) {
    let mut empty_s: Vec<u32> = vec![u32::default()];
    let mut short_s: Vec<u32> = vec![val; (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    let mut long_s: Vec<u32> = vec![val; (1100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s2: Vec<u32> = vec![val; (11100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s3: Vec<u32> = vec![val; (111_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_4(val: u8) {
    let mut empty_s: Vec<u8> = vec![u8::default()];
    let mut short_s: Vec<u8> = vec![val; (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    let mut long_s: Vec<u8> = vec![val; (1100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s2: Vec<u8> = vec![val; (11100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s3: Vec<u8> = vec![val; (111_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
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
    (unsafe { test_0(('x' as libc::c_char)) });
    (unsafe { test_1((120 as i32)) });
    (unsafe { test_2((120 as u16)) });
    (unsafe { test_3((120 as u32)) });
    (unsafe { test_4((120 as u8)) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
