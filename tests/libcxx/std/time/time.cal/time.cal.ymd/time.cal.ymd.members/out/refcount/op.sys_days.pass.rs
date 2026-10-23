extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn RunTheExample_0() {
    const _: () = assert!(
        true,
        "year_month_day{{sys_days{{year{{2017}}/January/0}}}}  == year{{2016}}/December/31"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{sys_days{{year{{2017}}/January/31}}}} == year{{2017}}/January/31"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{sys_days{{year{{2017}}/January/32}}}} == year{{2017}}/February/1"
    );;
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
    const _: () = assert!(true, "");;
    ({ RunTheExample_0() });
    {
        let ymd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sd.time_since_epoch() == days{{0}}");;
        const _: () = assert!(true, "year_month_day{{sd}} == ymd");;
    }
    {
        let ymd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sd.time_since_epoch() == days{{10957+32}}");;
        const _: () = assert!(true, "year_month_day{{sd}} == ymd");;
    }
    {
        let ymd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sd.time_since_epoch() == days{{-10957}}");;
        const _: () = assert!(true, "year_month_day{{sd}} == ymd");;
    }
    {
        let ymd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let sd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let ymd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!ymd.ok()");;
        const _: () = assert!(true, "ymd.year().ok()");;
        const _: () = assert!(true, "ymd.month().ok()");;
        const _: () = assert!(true, "!ymd.day().ok()");;
        let sd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sd.time_since_epoch() == days{{364}}");;
        const _: () = assert!(
            true,
            "sd == sys_days{{ymd.year()/ymd.month()/day{{1}}}} + (ymd.day() - day{{1}})"
        );;
    }
    {
        let ymd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "!ymd.ok()");;
        const _: () = assert!(true, "ymd.year().ok()");;
        const _: () = assert!(true, "ymd.month().ok()");;
        const _: () = assert!(true, "!ymd.day().ok()");;
        let sd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sd.time_since_epoch() == days{{365}}");;
        const _: () = assert!(
            true,
            "sd == sys_days{{ymd.year()/ymd.month()/day{{1}}}} + (ymd.day() - day{{1}})"
        );;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
