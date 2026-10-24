extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sputn_seekoff_0(
    mut buf: Ptr<()>,
    mut buf_size: (),
    mut chunk_size1: (),
    mut offset1: (),
    mut chunk_size2: (),
) {
    let mut data: () = unimplemented!();
    let mut data_size: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut result_size: () = (((5_() + chunk_size1) + chunk_size2) + 1_());
    if (offset1 > 0_()) {
        result_size = { (result_size).wrapping_add(offset1) };
    }
    let mut result: () = unimplemented!();
    {
        let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut chunk: () = unimplemented!();
        unimplemented!();
        let mut len1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut c: () = unimplemented!();
        if (unimplemented!() < 0_()) {
            (*p1.borrow_mut()) = unimplemented!();
            unimplemented!();
            c = ('^' as ());
        } else {
            unimplemented!();
            if (unimplemented!() > data_size) {
                c = ('_' as ());
            } else {
                c = (unimplemented!().read());
            }
        }
        unimplemented!().write(c);
        unimplemented!();
        unimplemented!();
        unimplemented!().write(('A' as ()));
        unimplemented!();
        let end_pos: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!().write(('Z' as ()));
        unimplemented!();
        unimplemented!().write(({ toupper_1(c) }));
        unimplemented!();
        let new_pos: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut len2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut check: () = unimplemented!();
    let mut len: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < len) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn sputn_not_open_2() {
    let mut data: () = unimplemented!();
    let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut len: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn sputn_not_open_wchar_3() {
    let mut data: () = unimplemented!();
    let f: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut len: () = unimplemented!();
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
    ({ sputn_not_open_2() });
    ({ sputn_not_open_wchar_3() });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 21_(), -27_(), 1_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 1_(), -27_(), 1_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 10_(), 14_(), 12_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 1_(), -2_(), 1_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 10_(), -4_(), 12_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 11_(), -12_(), 3_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 7_(), 3_(), 8_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 5_(), -5_(), 12_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 1_(), 1_(), 1_()) });
    ({ sputn_seekoff_0(Ptr::<()>::null(), 10_(), 9_(), 0_(), 1_()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
