extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut val: i8) {
    let mut empty_s: Vec<i8> = vec![i8::default()];
    let mut short_s: Vec<i8> = vec![val; (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
        .collect();
    let mut long_s: Vec<i8> = vec![val; (1100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
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
    let mut long_s2: Vec<i8> = vec![val; (11100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut long_s3: Vec<i8> = vec![val; (111_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
        .collect();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_1(mut val: i32) {
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
pub fn test_2(mut val: u16) {
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
pub fn test_3(mut val: u32) {
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
pub fn test_4(mut val: u8) {
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
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0(('x' as i8)) });
    ({ test_1((120 as i32)) });
    ({ test_2((120 as u16)) });
    ({ test_3((120 as u32)) });
    ({ test_4((120 as u8)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
