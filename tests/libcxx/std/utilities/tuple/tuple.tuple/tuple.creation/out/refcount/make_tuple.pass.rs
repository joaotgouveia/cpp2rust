extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let mut i: i32 = 0;
    let mut j: f32 = 0_f32;
    let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!(((unimplemented!().read()) == 1));
    assert!(((unimplemented!().read()) == 0));
    assert!(((unimplemented!().read()) == 0_f32));
    i = 2;
    j = (3.5_f64 as f32);
    assert!(((unimplemented!().read()) == 1));
    assert!(((unimplemented!().read()) == 2));
    assert!((((unimplemented!().read()) as f64) == 3.5_f64));
    unimplemented!().write(0);
    unimplemented!().write(0_f32);
    assert!((i == 0));
    assert!((j == 0_f32));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i1: i32 = ((unimplemented!().read()) as i32);
        let mut d1: f64 = (unimplemented!().read());
        const _: () = assert!(true, "i1 == 1");;
        const _: () = assert!(true, "d1 == 3.14");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
