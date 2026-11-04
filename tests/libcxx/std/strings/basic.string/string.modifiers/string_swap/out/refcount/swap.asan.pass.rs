extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(mut val: i8) {
    let empty_s: Value<Vec<i8>> = Rc::new(RefCell::new(vec![i8::default()]));
    let short_s: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![val; (3_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i8))
            .collect(),
    ));
    let long_s: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![val; (1100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i8))
            .collect(),
    ));
    cpp_swap!(empty_s.as_pointer(), empty_s.as_pointer());
    cpp_swap!(short_s.as_pointer(), short_s.as_pointer());
    cpp_swap!(long_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![val; (11100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i8))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![val; (111_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i8))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s3.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_1(mut val: i32) {
    let empty_s: Value<Vec<i32>> = Rc::new(RefCell::new(vec![i32::default()]));
    let short_s: Value<Vec<i32>> = Rc::new(RefCell::new(
        vec![val; (3_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i32))
            .collect(),
    ));
    let long_s: Value<Vec<i32>> = Rc::new(RefCell::new(
        vec![val; (1100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i32))
            .collect(),
    ));
    cpp_swap!(empty_s.as_pointer(), empty_s.as_pointer());
    cpp_swap!(short_s.as_pointer(), short_s.as_pointer());
    cpp_swap!(long_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<Vec<i32>> = Rc::new(RefCell::new(
        vec![val; (11100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i32))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<Vec<i32>> = Rc::new(RefCell::new(
        vec![val; (111_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i32))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s3.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_2(mut val: u16) {
    let empty_s: Value<Vec<u16>> = Rc::new(RefCell::new(vec![u16::default()]));
    let short_s: Value<Vec<u16>> = Rc::new(RefCell::new(
        vec![val; (3_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u16))
            .collect(),
    ));
    let long_s: Value<Vec<u16>> = Rc::new(RefCell::new(
        vec![val; (1100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u16))
            .collect(),
    ));
    cpp_swap!(empty_s.as_pointer(), empty_s.as_pointer());
    cpp_swap!(short_s.as_pointer(), short_s.as_pointer());
    cpp_swap!(long_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<Vec<u16>> = Rc::new(RefCell::new(
        vec![val; (11100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u16))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<Vec<u16>> = Rc::new(RefCell::new(
        vec![val; (111_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u16))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s3.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_3(mut val: u32) {
    let empty_s: Value<Vec<u32>> = Rc::new(RefCell::new(vec![u32::default()]));
    let short_s: Value<Vec<u32>> = Rc::new(RefCell::new(
        vec![val; (3_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u32))
            .collect(),
    ));
    let long_s: Value<Vec<u32>> = Rc::new(RefCell::new(
        vec![val; (1100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u32))
            .collect(),
    ));
    cpp_swap!(empty_s.as_pointer(), empty_s.as_pointer());
    cpp_swap!(short_s.as_pointer(), short_s.as_pointer());
    cpp_swap!(long_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<Vec<u32>> = Rc::new(RefCell::new(
        vec![val; (11100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u32))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<Vec<u32>> = Rc::new(RefCell::new(
        vec![val; (111_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u32))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s3.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn test_4(mut val: u8) {
    let empty_s: Value<Vec<u8>> = Rc::new(RefCell::new(vec![u8::default()]));
    let short_s: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![val; (3_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u8))
            .collect(),
    ));
    let long_s: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![val; (1100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u8))
            .collect(),
    ));
    cpp_swap!(empty_s.as_pointer(), empty_s.as_pointer());
    cpp_swap!(short_s.as_pointer(), short_s.as_pointer());
    cpp_swap!(long_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), short_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(empty_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(short_s.as_pointer(), long_s.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s2: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![val; (11100_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u8))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    cpp_swap!(long_s.as_pointer(), long_s2.as_pointer());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let long_s3: Value<Vec<u8>> = Rc::new(RefCell::new(
        vec![val; (111_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_u8))
            .collect(),
    ));
    cpp_swap!(long_s.as_pointer(), long_s3.as_pointer());
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
