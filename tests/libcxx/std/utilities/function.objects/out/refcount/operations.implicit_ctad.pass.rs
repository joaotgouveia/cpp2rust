extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

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
        let f: Value<std_plus_void_> = Rc::new(RefCell::new(<std_plus_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_minus_void_> = Rc::new(RefCell::new(<std_minus_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_multiplies_void_> =
            Rc::new(RefCell::new(<std_multiplies_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_divides_void_> = Rc::new(RefCell::new(<std_divides_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_modulus_void_> = Rc::new(RefCell::new(<std_modulus_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_negate_void_> = Rc::new(RefCell::new(<std_negate_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_bit_and_void_> = Rc::new(RefCell::new(<std_bit_and_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_bit_not_void_> = Rc::new(RefCell::new(<std_bit_not_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_bit_or_void_> = Rc::new(RefCell::new(<std_bit_or_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_bit_xor_void_> = Rc::new(RefCell::new(<std_bit_xor_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_equal_to_void_> = Rc::new(RefCell::new(<std_equal_to_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_not_equal_to_void_> =
            Rc::new(RefCell::new(<std_not_equal_to_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_less_void_> = Rc::new(RefCell::new(<std_less_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_less_equal_void_> =
            Rc::new(RefCell::new(<std_less_equal_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_greater_equal_void_> =
            Rc::new(RefCell::new(<std_greater_equal_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_greater_void_> = Rc::new(RefCell::new(<std_greater_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_logical_and_void_> =
            Rc::new(RefCell::new(<std_logical_and_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_logical_not_void_> =
            Rc::new(RefCell::new(<std_logical_not_void_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let f: Value<std_logical_or_void_> =
            Rc::new(RefCell::new(<std_logical_or_void_>::default()));
        const _: () = assert!(true, "");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
