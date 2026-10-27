extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_chrono_weekday_indexed__std_chrono_weekday_indexed_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_chrono_weekday_last__std_chrono_weekday_last_ {}
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
    let Sunday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "Sunday[2].weekday() == Sunday");;
    const _: () = assert!(true, "Sunday[2].index  () == 2");;
    let mut i: () = 0_();
    'loop_: while (i <= 6_()) {
        let wd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let wdl: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i <= 6_()) {
        let mut j: () = 1_();
        'loop_: while (j <= 5_()) {
            let wd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let wdi: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
