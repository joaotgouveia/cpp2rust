extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
    let b: Value<Box<[Ptr<i32>]>> = Rc::new(RefCell::new(Box::new([
        ((a.as_pointer() as Ptr<i32>).offset(2)),
        ((a.as_pointer() as Ptr<i32>).offset(0)),
        ((a.as_pointer() as Ptr<i32>).offset(1)),
    ])));
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == (b.as_pointer() as Ptr<Ptr<i32>>).offset((0) as isize)));
    assert!((unimplemented!() == (b.as_pointer() as Ptr<Ptr<i32>>).offset((3) as isize)));
    let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == (b.as_pointer() as Ptr<Ptr<i32>>).offset((0) as isize)));
    assert!((unimplemented!() == (b.as_pointer() as Ptr<Ptr<i32>>).offset((3) as isize)));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    assert!((unimplemented!() == (b.as_pointer() as Ptr<Ptr<i32>>).offset((0) as isize)));
    assert!((unimplemented!() == (b.as_pointer() as Ptr<Ptr<i32>>).offset((3) as isize)));
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
