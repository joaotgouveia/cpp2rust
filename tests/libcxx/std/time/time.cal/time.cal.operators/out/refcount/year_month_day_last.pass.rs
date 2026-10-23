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
pub struct test_macros_detail_is_same_std_chrono_year_month_day_last__std_chrono_year_month_day_last_
{}
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
    let February: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let Feb2018: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "(Feb2018/last).year()  == year{{2018}}");;
        const _: () = assert!(true, "(Feb2018/last).month() == February");;
        let mut i: () = 1000_();
        'loop_: while (i < 1010_()) {
            let mut j: () = 1_();
            'loop_: while (j <= 12_()) {
                let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let ymdl: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "(year{{2018}}/month_day_last{{February}}).month() == February"
        );;
        const _: () = assert!(
            true,
            "(year{{2018}}/month_day_last{{February}}).year()  == year{{2018}}"
        );;
        const _: () = assert!(
            true,
            "(month_day_last{{February}}/year{{2018}}).month() == February"
        );;
        const _: () = assert!(
            true,
            "(month_day_last{{February}}/year{{2018}}).year()  == year{{2018}}"
        );;
        let mut i: () = 1000_();
        'loop_: while (i < 1010_()) {
            let mut j: () = 1_();
            'loop_: while (j <= 12_()) {
                let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let ymdl1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let ymdl2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "(2018/month_day_last{{February}}).month() == February"
        );;
        const _: () = assert!(
            true,
            "(2018/month_day_last{{February}}).year()  == year{{2018}}"
        );;
        const _: () = assert!(
            true,
            "(month_day_last{{February}}/2018).month() == February"
        );;
        const _: () = assert!(
            true,
            "(month_day_last{{February}}/2018).year()  == year{{2018}}"
        );;
        let mut i: () = 1000_();
        'loop_: while (i < 1010_()) {
            let mut j: () = 1_();
            'loop_: while (j <= 12_()) {
                let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let ymdl1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                let ymdl2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let ymdl: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*d.borrow_mut());
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
