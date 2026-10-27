extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn truncate_fp_0(mut val: ()) -> () {
    let mut sink: () = val;
    return sink;
}
pub fn truncate_fp_1(mut val: ()) -> () {
    let mut sink: () = val;
    return sink;
}
pub fn truncate_fp_2(mut val: ()) -> () {
    let mut sink: () = val;
    return sink;
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
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_3(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_4(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_5(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_6(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_7(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_8(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_9(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_10(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_11(r.as_pointer()) });
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut f: () = ({ generate_canonical_12(r.as_pointer()) });
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
