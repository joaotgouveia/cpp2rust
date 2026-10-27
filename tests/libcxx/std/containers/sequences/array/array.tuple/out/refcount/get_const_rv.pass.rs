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
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<const T&&, decltype(std::get<0>(std::move(c)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(c)))");;
        let t: Ptr<()> = ({ get_0(c.as_pointer()) });
        unimplemented!();
    }
    {
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(std::move(c)) == 1");;
        const _: () = assert!(true, "std::get<1>(std::move(c)) == 2");;
        const _: () = assert!(true, "std::get<2>(std::move(c)) == 3.5");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
