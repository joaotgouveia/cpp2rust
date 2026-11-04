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
    const _: () = assert!(true, "");;
    let md0: Value<std_chrono_month_weekday> = Rc::new(RefCell::new({
        let __tmp_0: Value<std_chrono_month> = Rc::new(RefCell::new(<std_chrono_month>::default()));
        let __tmp_1: Value<std_chrono_weekday_indexed> =
            Rc::new(RefCell::new(<std_chrono_weekday_indexed>::default()));
        std_chrono_month_weekday::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    const _: () = assert!(true, "md0.month()           == month{{}}");;
    const _: () = assert!(true, "md0.weekday_indexed() == weekday_indexed{{}}");;
    const _: () = assert!(true, "!md0.ok()");;
    let md1: Value<std_chrono_month_weekday> = Rc::new(RefCell::new({
        let __tmp_2: Value<std_chrono_weekday_indexed> = Rc::new(RefCell::new(
            std_chrono_weekday_indexed::new_1({ Friday_1.with(|v| v.as_pointer()) }, { 4_u32 }),
        ));
        std_chrono_month_weekday::new({ January_0.with(|v| v.as_pointer()) }, {
            __tmp_2.as_pointer()
        })
    }));
    const _: () = assert!(true, "md1.month() == std::chrono::January");;
    const _: () = assert!(
        true,
        "md1.weekday_indexed() == weekday_indexed{{std::chrono::Friday, 4}}"
    );;
    const _: () = assert!(true, "md1.ok()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
