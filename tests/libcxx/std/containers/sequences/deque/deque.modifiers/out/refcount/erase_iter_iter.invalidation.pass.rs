extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_double_ended_contiguous_container_asan_correct_0(_a0: Ptr<()>) -> () {
    return true;
}
pub fn del_at_start_1(c: (), mut num: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let first: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let it3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn del_at_end_2(c: (), mut num: ()) {
    let c: Value<()> = Rc::new(RefCell::new(c));
    let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let first: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let it3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    let queue: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 20_()) {
        unimplemented!();
        i.prefix_inc();
    }
    'loop_: while (unimplemented!() > 1_()) {
        let mut i: () = 1_();
        'loop_: while (i < unimplemented!()) {
            ({ del_at_start_1(unimplemented!(), i) });
            ({ del_at_end_2(unimplemented!(), i) });
            i.prefix_inc();
        }
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
