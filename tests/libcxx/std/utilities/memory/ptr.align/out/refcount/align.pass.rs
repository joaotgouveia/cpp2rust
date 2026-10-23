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
    let N: Value<()> = Rc::new(RefCell::new(20_()));
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..20).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut r: () = unimplemented!();
    let mut p: () = (((buf.as_pointer() as Ptr<()>).offset(0_())) as Ptr<()>).to_any();
    let mut s: () = (*N.borrow());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(1_())) as Ptr<()>).to_any();
    s = (*N.borrow());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(2_())) as Ptr<()>).to_any();
    s = (*N.borrow());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(3_())) as Ptr<()>).to_any();
    s = (*N.borrow());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(4_())) as Ptr<()>).to_any();
    s = (*N.borrow());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(0_())) as Ptr<()>).to_any();
    s = (*N.borrow());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(1_())) as Ptr<()>).to_any();
    s = (*N.borrow()).wrapping_sub(1_());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(1_())) as Ptr<()>).to_any();
    s = (*N.borrow()).wrapping_sub(1_());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = (((buf.as_pointer() as Ptr<()>).offset(0_())) as Ptr<()>).to_any();
    s = (*N.borrow());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
