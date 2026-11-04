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
    let f: Value<std_multiplies_int_> = Rc::new(RefCell::new(<std_multiplies_int_>::default()));
    assert!(
        (({
            let ___x: Value<i32> = Rc::new(RefCell::new(3));
            let ___y: Value<i32> = Rc::new(RefCell::new(2));
            (*f.borrow_mut())(___x.as_pointer(), ___y.as_pointer())
        }) == 6)
    );
    let f2: Value<std_multiplies_void_> = Rc::new(RefCell::new(<std_multiplies_void_>::default()));
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(3));
            let ___u: Value<i32> = Rc::new(RefCell::new(2));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 6)
    );
    assert!(
        (({
            let ___t: Value<f64> = Rc::new(RefCell::new(3_f64));
            let ___u: Value<i32> = Rc::new(RefCell::new(2));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 6_f64)
    );
    assert!(
        (({
            let ___t: Value<i32> = Rc::new(RefCell::new(3));
            let ___u: Value<f64> = Rc::new(RefCell::new(2.5_f64));
            (*f2.borrow_mut())(___t.as_pointer(), ___u.as_pointer())
        }) == 7.5_f64)
    );
    let mut foo: i32 = 6;
    const _: () = assert!(true, "foo == 6");;
    let mut bar: f64 = 6_f64;
    const _: () = assert!(true, "bar == 6.0");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
