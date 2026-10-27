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
pub struct test_macros_detail_is_same_bool__bool_ {}
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
    let January: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Monday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Tuesday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Wednesday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Thursday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Friday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Saturday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let Sunday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "!year_month_weekday{{}}.ok()");;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, month{{}}, weekday_indexed{{}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, January, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   month{{}}, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, month{{}}, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   month{{}}, weekday_indexed{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, January, weekday_indexed{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, static_cast<unsigned>(-1)}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Wednesday, 0}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, 4}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Monday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Wednesday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Thursday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Friday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Saturday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Sunday, 5}}}}.ok()"
    );;
    let mut i: () = 0_();
    'loop_: while (i <= 50_()) {
        let ym: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i <= 50_()) {
        let ym: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while (i <= 50_()) {
        let ym: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        i.prefix_inc();
    }
    let mut ymax: () = unimplemented!();
    let mut i: () = (ymax - 100_());
    'loop_: while (i <= (ymax + 100_())) {
        let ym: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
