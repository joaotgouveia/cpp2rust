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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut all_in_64bit_range: () = true;
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            let mut n: () = unimplemented!();
            all_in_64bit_range = { (all_in_64bit_range) && (n <= (18446744073709551615_())) };
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut a: () = ((-9223372036854775807_() - 1_()) * 10_());
        let mut b: () = ((9223372036854775807_()) * 10_());
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut all_in_64bit_range: () = true;
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            let mut n: () = unimplemented!();
            unimplemented!();
            all_in_64bit_range = {
                ((all_in_64bit_range) && ((-9223372036854775807_() - 1_()) <= n))
                    && (n <= (9223372036854775807_()))
            };
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut a: () = ((18446744073709551615_()) as ()).wrapping_div(3_());
        let mut b: () = ((18446744073709551615_()) as ()).wrapping_mul(10_());
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut all_in_64bit_range: () = true;
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            let mut n: () = unimplemented!();
            unimplemented!();
            all_in_64bit_range = { (all_in_64bit_range) && (n <= (18446744073709551615_())) };
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 100_()) {
            let mut n: () = unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
