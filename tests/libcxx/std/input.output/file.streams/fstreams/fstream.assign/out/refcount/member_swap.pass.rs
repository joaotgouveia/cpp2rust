extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_temp_file_name_0() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = unimplemented!();
    if (FD == -1_()) {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub fn glibc_version_less_than_1(mut version: Ptr<()>) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = unimplemented!();
    let mut current_version: Ptr<()> =
        Ptr::alloc_array((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>());
    unimplemented!();
    let result: Value<()> = Rc::new(RefCell::new((unimplemented!() < 0_())));
    current_version.delete();
    return (*result.borrow());
}
pub fn get_temp_file_names_2() -> () {
    let names: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut fd1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
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
    let temp_files: Value<()> = Rc::new(RefCell::new(({ get_temp_file_names_2() })));
    let temp1: Ptr<()> = field_ptr!(temp_files.as_pointer(), first);
    let temp2: Ptr<()> = field_ptr!(temp_files.as_pointer(), second);
    unimplemented!();
    {
        let fs1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let fs2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    {
        let fs1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let fs2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
