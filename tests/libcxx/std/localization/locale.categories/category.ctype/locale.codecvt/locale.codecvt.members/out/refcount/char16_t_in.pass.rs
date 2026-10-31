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
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let from: Value<Box<[i8]>> = Rc::new(RefCell::new(i8::array_from_literal(b"some text\0")));
    let to: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..9).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let f: Ptr<()> = unimplemented!();
    let mbs: Value<::libc::mbstate_t> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    let mut from_next: Ptr<i8> = Ptr::<i8>::null();
    let mut to_next: Ptr<()> = Ptr::<()>::null();
    assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
    assert!((((from_next).clone() - (from.as_pointer() as Ptr::<i8>)) as i64 == 9_i64));
    assert!((((to_next).clone() - (to.as_pointer() as Ptr::<()>)) as i64 == 9_i64));
    let mut i: u32 = 0_u32;
    'loop_: while (i < 9_u32) {
        assert!((((*to.borrow())[(i) as usize] as i32) == ((*from.borrow())[(i) as usize] as i32)));
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
