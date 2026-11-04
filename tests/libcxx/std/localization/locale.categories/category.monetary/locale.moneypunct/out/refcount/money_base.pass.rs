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
    let mb: Value<std_money_base> = Rc::new(RefCell::new(<std_money_base>::default()));
    (&(*mb.borrow_mut()));
    const _: () = assert!(true, "std::money_base::none == 0");;
    const _: () = assert!(true, "std::money_base::space == 1");;
    const _: () = assert!(true, "std::money_base::symbol == 2");;
    const _: () = assert!(true, "std::money_base::sign == 3");;
    const _: () = assert!(true, "std::money_base::value == 4");;
    const _: () = assert!((4usize == 4_usize), "sizeof(std::money_base::pattern) == 4");;
    let p: Value<std_money_base_pattern> =
        Rc::new(RefCell::new(<std_money_base_pattern>::default()));
    &(*p.borrow_mut());
    array_elem!(p.as_pointer(), field, 0).write((std_money_base_part_none as i8));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
