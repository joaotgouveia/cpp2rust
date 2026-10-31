extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn hash_identity_0(val: u64) -> usize {
    let val: Value<u64> = Rc::new(RefCell::new(val));
    return ((*val.borrow()) as usize);
}
pub fn hash_identity_1(val: bool) -> usize {
    let val: Value<bool> = Rc::new(RefCell::new(val));
    return ((*val.borrow()) as usize);
}
pub fn hash_neg_2(mut val: u64) -> usize {
    return ((u64::MAX as u64).wrapping_sub(val) as usize);
}
pub fn hash_neg_3(mut val: bool) -> usize {
    return (((true as i32) - (val as i32)) as usize);
}
pub fn hash_scale_4(mut val: u64) -> usize {
    return ((val << 1) as usize);
}
pub fn hash_scale_5(mut val: bool) -> usize {
    return (((val as i32) << 1) as usize);
}
pub fn hash_even_6(mut val: u64) -> usize {
    return (if ((val & 1_u64) != 0) { 1 } else { 0 } as usize);
}
pub fn hash_even_7(mut val: bool) -> usize {
    return (if (((val as i32) & 1) != 0) { 1 } else { 0 } as usize);
}
pub fn hash_same_8(mut _a0: u64) -> usize {
    return 1_usize;
}
pub fn hash_same_9(mut _a0: bool) -> usize {
    return 1_usize;
}
pub fn hash_same_10(mut _a0: Ptr<i8>) -> usize {
    return 1_usize;
}
pub fn hash_identity_11(mut val: Ptr<i8>) -> usize {
    return ((val.read()) as usize);
}
pub fn hash_neg_12(mut val: Ptr<i8>) -> usize {
    return (({ (libc::c_char::MAX as i32) } - { ((val.read()) as i32) }) as usize);
}
pub fn hash_scale_13(mut val: Ptr<i8>) -> usize {
    return ((((val.read()) as i32) << 1) as usize);
}
pub fn hash_even_14(mut val: Ptr<i8>) -> usize {
    return (if ((((val.read()) as i32) & 1) != 0) {
        1
    } else {
        0
    } as usize);
}
pub fn test_15(vals: Ptr<u64>) {
    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn test_16(vals: Ptr<bool>) {
    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub fn test_17(vals: Ptr<Ptr<i8>>) {
    let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let c5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
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
    {
        let vals: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
            1_usize, 2_usize, 2_usize, 3_usize, 3_usize, 3_usize, 4_usize, 4_usize, 4_usize,
            4_usize, 5_usize, 5_usize, 5_usize, 5_usize, 5_usize, 6_usize, 6_usize, 6_usize,
            6_usize, 6_usize, 6_usize, 7_usize, 7_usize, 7_usize, 7_usize, 7_usize, 7_usize,
            7_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize, 8_usize,
            9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize, 9_usize,
            10_usize,
        ])));
        ({ test_15((vals.as_pointer() as Ptr<usize>).reinterpret_cast::<u64>()) });
    }
    {
        let vals: Value<Box<[bool]>> = Rc::new(RefCell::new(Box::new([true, false])));
        ({ test_16(vals.as_pointer()) });
    }
    {
        let vals: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(Box::new([
            Ptr::<i8>::from_string_literal(b"a").reinterpret_cast::<i8>(),
            Ptr::<i8>::from_string_literal(b"b").reinterpret_cast::<i8>(),
            Ptr::<i8>::from_string_literal(b"cde").reinterpret_cast::<i8>(),
        ])));
        ({ test_17(vals.as_pointer()) });
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
