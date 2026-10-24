extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn tempArray_0(mut args_0: (), mut args_1: (), mut args_2: ()) -> () {
    return unimplemented!();
}
pub fn tests_1() -> () {
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        ({ get_2(array.as_pointer()) }).write(99.1_f64);
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        ({ get_3(array.as_pointer()) }).write(99.1_f64);
        ({ get_4(array.as_pointer()) }).write(99.2_f64);
        unimplemented!();
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        ({ get_5(array.as_pointer()) }).write(99.2_f64);
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<double&, decltype(std::get<0>(array))>::value"
        );;
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
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
    ({ tests_1() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
