extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
}
pub fn test_1() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
}
pub fn test_2() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
}
pub fn test_3() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
}
pub fn test_4() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
}
pub fn test_5() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
}
pub fn test_6() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
}
pub fn test_7() {
    const _: () = assert!(true, "(result == std::ratio_not_equal<Rat1, Rat2>::value)");;
    const _: () = assert!(true, "(result == std::ratio_not_equal_v<Rat1, Rat2>)");;
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
    {
        ({ test_0() });
    }
    {
        ({ test_1() });
    }
    {
        ({ test_2() });
    }
    {
        ({ test_3() });
    }
    {
        ({ test_4() });
    }
    {
        ({ test_5() });
    }
    {
        ({ test_6() });
    }
    {
        ({ test_7() });
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
