extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fw_0(mut x: ()) -> () {
    return (2_() * x);
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
        let pa: Value<std_piecewise_linear_distribution___param_type> = Rc::new(RefCell::new(
            std_piecewise_linear_distribution___param_type::new_1({ 0_() }, { 0_() }, { 1_() }, {
                FnPtr::<fn(()) -> ()>::new(fw_0)
            }),
        ));
        let mut iv: () = ({ (*pa.borrow()).intervals() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut dn: () = ({ (*pa.borrow()).densities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let pa: Value<std_piecewise_linear_distribution___param_type> = Rc::new(RefCell::new(
            std_piecewise_linear_distribution___param_type::new_1(
                { 1_() },
                { 10_() },
                { 12_() },
                { FnPtr::<fn(()) -> ()>::new(fw_0) },
            ),
        ));
        let mut iv: () = ({ (*pa.borrow()).intervals() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut dn: () = ({ (*pa.borrow()).densities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let pa: Value<std_piecewise_linear_distribution___param_type> = Rc::new(RefCell::new(
            std_piecewise_linear_distribution___param_type::new_1({ 2_() }, { 6_() }, { 14_() }, {
                FnPtr::<fn(()) -> ()>::new(fw_0)
            }),
        ));
        let mut iv: () = ({ (*pa.borrow()).intervals() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut dn: () = ({ (*pa.borrow()).densities() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
