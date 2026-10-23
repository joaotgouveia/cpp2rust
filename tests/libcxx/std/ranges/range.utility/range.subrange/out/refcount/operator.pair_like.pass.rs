extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(
    true,
    "std::convertible_to<std::ranges::subrange<int*>, std::pair<int*, int*>>"
);
const _: () = assert!(
    true,
    "std::convertible_to<std::ranges::subrange<int*>, std::tuple<int*, int*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::pair<long*, int*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::pair<int*, long*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::pair<long*, long*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::array<int*, 2>>"
);
pub fn test_0() -> () {
    {
        let data: Value<Box<[()]>> =
            Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let p: Value<()> = Rc::new(RefCell::new(({ (*a.borrow())() })));
            unimplemented!();
            unimplemented!();
        }
        {
            let p: Value<()> = Rc::new(RefCell::new(({ (*a.borrow())() })));
            unimplemented!();
            unimplemented!();
        }
        {
            let p: Value<()> = Rc::new(RefCell::new(({ (*a.borrow())() })));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let data: Value<Box<[()]>> =
            Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            let p: Value<()> = Rc::new(RefCell::new(({ (*a.borrow())() })));
            unimplemented!();
            unimplemented!();
        }
        {
            let p: Value<()> = Rc::new(RefCell::new(({ (*a.borrow())() })));
            unimplemented!();
            unimplemented!();
        }
        {
            let p: Value<()> = Rc::new(RefCell::new(({ (*a.borrow())() })));
            unimplemented!();
            unimplemented!();
        }
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
