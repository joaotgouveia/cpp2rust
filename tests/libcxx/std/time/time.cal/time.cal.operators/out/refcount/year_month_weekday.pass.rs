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
pub struct test_macros_detail_is_same_std_chrono_year_month_weekday__std_chrono_year_month_weekday_
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
    let Tuesday: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let February: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    {
        let Feb2018: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "(Feb2018/weekday_indexed{{Tuesday, 2}}).year()    == year{{2018}}"
        );;
        const _: () = assert!(
            true,
            "(Feb2018/weekday_indexed{{Tuesday, 2}}).month()   == February"
        );;
        const _: () = assert!(
            true,
            "(Feb2018/weekday_indexed{{Tuesday, 2}}).weekday() == Tuesday"
        );;
        let mut i: () = 1000_();
        'loop_: while (i < 1010_()) {
            let mut j: () = 1_();
            'loop_: while (j <= 12_()) {
                let mut k: () = 0_();
                'loop_: while (k <= 6_()) {
                    let mut l: () = 1_();
                    'loop_: while (l <= 5_()) {
                        let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let wd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let ymd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        l.prefix_inc();
                    }
                    k.prefix_inc();
                }
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let Feb1stTues: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "(year{{2018}}/Feb1stTues).year()    == year{{2018}}");;
        const _: () = assert!(true, "(year{{2018}}/Feb1stTues).month()   == February");;
        const _: () = assert!(true, "(year{{2018}}/Feb1stTues).weekday() == Tuesday");;
        let mut i: () = 1000_();
        'loop_: while (i < 1010_()) {
            let mut j: () = 1_();
            'loop_: while (j <= 12_()) {
                let mut k: () = 0_();
                'loop_: while (k <= 6_()) {
                    let mut l: () = 1_();
                    'loop_: while (l <= 5_()) {
                        let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let wd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mwd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let ymd1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let ymd2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        l.prefix_inc();
                    }
                    k.prefix_inc();
                }
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let Feb1stTues: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "(2018/Feb1stTues).year()    == year{{2018}}");;
        const _: () = assert!(true, "(2018/Feb1stTues).month()   == February");;
        const _: () = assert!(true, "(2018/Feb1stTues).weekday() == Tuesday");;
        let mut i: () = 1000_();
        'loop_: while (i < 1010_()) {
            let mut j: () = 1_();
            'loop_: while (j <= 12_()) {
                let mut k: () = 0_();
                'loop_: while (k <= 6_()) {
                    let mut l: () = 1_();
                    'loop_: while (l <= 5_()) {
                        let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let wd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let mwd: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let ymd1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        let ymd2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        unimplemented!();
                        l.prefix_inc();
                    }
                    k.prefix_inc();
                }
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
