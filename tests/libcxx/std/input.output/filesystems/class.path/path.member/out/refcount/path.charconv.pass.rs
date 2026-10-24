extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_latin_unicode_0() {
    let u16str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([229_(), 228_(), 246_(), 0_()])));
    let u32str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([229_(), 228_(), 246_(), 0_()])));
    let str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        -61_(),
        -91_(),
        -61_(),
        -92_(),
        -61_(),
        -74_(),
        0_(),
    ])));
    let u8str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        195_(),
        165_(),
        195_(),
        164_(),
        195_(),
        182_(),
        0_(),
    ])));
    let wstr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([229_(), 228_(), 246_(), 0_()])));
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_wide_unicode_1() {
    let u16str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([55297_(), 56375_(), 0_()])));
    let u32str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([66615_(), 0_()])));
    let u8str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        240_(),
        144_(),
        144_(),
        183_(),
        0_(),
    ])));
    let mut str: [(); 5] = [-16_(), -112_(), -112_(), -73_(), 0_()];
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let wstr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([66615_(), 0_()])));
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_append_2() {
    let mut u16str: [(); 3] = [55297_(), 56375_(), 0_()];
    let mut u32str: [(); 2] = [66615_(), 0_()];
    let u32ref: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        66615_(),
        unimplemented!(),
        66615_(),
        unimplemented!(),
        66615_(),
        0_(),
    ])));
    let mut str: [(); 5] = [-16_(), -112_(), -112_(), -73_(), 0_()];
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let mut wstr: [(); 2] = [66615_(), 0_()];
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_concat_3() {
    let mut u16str: [(); 3] = [55297_(), 56375_(), 0_()];
    let mut u32str: [(); 2] = [66615_(), 0_()];
    let u32ref: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([66615_(), 66615_(), 66615_(), 0_()])));
    let mut str: [(); 5] = [-16_(), -112_(), -112_(), -73_(), 0_()];
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let mut wstr: [(); 2] = [66615_(), 0_()];
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_append_concat_narrow_4() {
    let mut u16str: [(); 2] = [229_(), 0_()];
    let u32ref_append: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        229_(),
        unimplemented!(),
        229_(),
        0_(),
    ])));
    let u32ref_concat: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([229_(), 229_(), 0_()])));
    {
        let mut u8str: [(); 3] = [195_(), 165_(), 0_()];
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let str: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([-61_(), -91_(), 0_()])));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_latin_unicode_0() });
    ({ test_wide_unicode_1() });
    ({ test_append_2() });
    ({ test_concat_3() });
    ({ test_append_concat_narrow_4() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
