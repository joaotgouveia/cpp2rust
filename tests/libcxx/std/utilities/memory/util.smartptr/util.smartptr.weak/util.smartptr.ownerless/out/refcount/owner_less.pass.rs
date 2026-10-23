extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X {}
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
    let p1: Value<()> = Rc::new(RefCell::new(std_shared_ptr_int_::new_1({
        Ptr::alloc(Default::default())
    })));
    let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let p3: Value<()> = Rc::new(RefCell::new(std_shared_ptr_int_::new_1({
        Ptr::alloc(Default::default())
    })));
    let w1: Value<()> = Rc::new(RefCell::new(std_weak_ptr_int_::new_1({ p1.as_pointer() })));
    let w2: Value<()> = Rc::new(RefCell::new(std_weak_ptr_int_::new_1({ p2.as_pointer() })));
    let w3: Value<()> = Rc::new(RefCell::new(std_weak_ptr_int_::new_1({ p3.as_pointer() })));
    {
        let cs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let cs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let sp1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sp2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sp3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let wp1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cmp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let vp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
