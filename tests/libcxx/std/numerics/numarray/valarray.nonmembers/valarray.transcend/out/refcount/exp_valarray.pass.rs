extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_about_0(mut x: (), mut y: (), mut significant_digits: ()) -> () {
    let mut epsilon: () = ({ pow_1((1_f64 / 10_f64), significant_digits) });
    return (({ abs_2((x - y)) }) < epsilon);
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
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            -0.9_f64, -0.5_f64, 0_f64, 0.5_f64, 0.75_f64,
        ])));
        let mut a3: [(); 5] = [
            0.4065696597405991_f64,
            0.6065306597126334_f64,
            1_f64,
            1.6487212707001282_f64,
            2.117000016612675_f64,
        ];
        let mut N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let v3: Value<()> = Rc::new(RefCell::new(({ unimplemented!()() })));
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
