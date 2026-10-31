extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    let mut short_s1: Vec<i8> = vec![('a' as i8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
        .collect();
    let mut long_s1: Vec<i8> = vec![('c' as i8); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        short_s1.clear();
        short_s1.push(0_i8)
    };
    {
        long_s1.clear();
        long_s1.push(0_i8)
    };
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut short_s2: Vec<i8> = vec![('a' as i8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
        .collect();
    let mut long_s2: Vec<i8> = vec![('c' as i8); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i8))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_1() {
    let mut short_s1: Vec<i32> = vec![(('a' as i8) as i32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    let mut long_s1: Vec<i32> = vec![(('c' as i8) as i32); (100_usize) as usize]
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
    let mut short_s2: Vec<i32> = vec![(('a' as i8) as i32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    let mut long_s2: Vec<i32> = vec![(('c' as i8) as i32); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_2() {
    let mut short_s1: Vec<u16> = vec![(('a' as i8) as u16); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    let mut long_s1: Vec<u16> = vec![(('c' as i8) as u16); (100_usize) as usize]
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
    let mut short_s2: Vec<u16> = vec![(('a' as i8) as u16); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    let mut long_s2: Vec<u16> = vec![(('c' as i8) as u16); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u16))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_3() {
    let mut short_s1: Vec<u32> = vec![(('a' as i8) as u32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    let mut long_s1: Vec<u32> = vec![(('c' as i8) as u32); (100_usize) as usize]
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
    let mut short_s2: Vec<u32> = vec![(('a' as i8) as u32); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    let mut long_s2: Vec<u32> = vec![(('c' as i8) as u32); (100_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u32))
        .collect();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_4() {
    let mut short_s1: Vec<u8> = vec![(('a' as i8) as u8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    let mut long_s1: Vec<u8> = vec![(('c' as i8) as u8); (100_usize) as usize]
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
    let mut short_s2: Vec<u8> = vec![(('a' as i8) as u8); (3_usize) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_u8))
        .collect();
    let mut long_s2: Vec<u8> = vec![(('c' as i8) as u8); (100_usize) as usize]
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    ({ test_4() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
