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
    let January: Value<std_chrono_month> = Rc::new(RefCell::new(
        (*January_0.with(Value::clone).borrow()).clone(),
    ));
    let Tuesday: Value<std_chrono_weekday> = Rc::new(RefCell::new(
        (*Tuesday_1.with(Value::clone).borrow()).clone(),
    ));
    const _: () = assert!(true, "");;
    let mwdl1: Value<std_chrono_month_weekday_last> = Rc::new(RefCell::new({
        let __tmp_0: Value<std_chrono_month> = Rc::new(RefCell::new(<std_chrono_month>::default()));
        let __tmp_1: Value<std_chrono_weekday_last> =
            Rc::new(RefCell::new(std_chrono_weekday_last::new({
                Tuesday.as_pointer()
            })));
        std_chrono_month_weekday_last::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    const _: () = assert!(true, "mwdl1.month() == month{{}}");;
    const _: () = assert!(true, "mwdl1.weekday_last() == weekday_last{{Tuesday}}");;
    const _: () = assert!(true, "!mwdl1.ok()");;
    let mwdl2: Value<std_chrono_month_weekday_last> = Rc::new(RefCell::new({
        let __tmp_2: Value<std_chrono_weekday_last> = Rc::new(RefCell::new({
            let __tmp_3: Value<std_chrono_weekday> =
                Rc::new(RefCell::new(std_chrono_weekday::new_1({ 16_u32 })));
            std_chrono_weekday_last::new({ __tmp_3.as_pointer() })
        }));
        std_chrono_month_weekday_last::new({ January.as_pointer() }, { __tmp_2.as_pointer() })
    }));
    const _: () = assert!(true, "mwdl2.month() == January");;
    const _: () = assert!(
        true,
        "mwdl2.weekday_last() == weekday_last{{weekday{{16}}}}"
    );;
    const _: () = assert!(true, "!mwdl2.ok()");;
    let mwdl3: Value<std_chrono_month_weekday_last> = Rc::new(RefCell::new({
        let __tmp_4: Value<std_chrono_weekday_last> = Rc::new(RefCell::new({
            let __tmp_5: Value<std_chrono_weekday> =
                Rc::new(RefCell::new(std_chrono_weekday::new_1({ 4_u32 })));
            std_chrono_weekday_last::new({ __tmp_5.as_pointer() })
        }));
        std_chrono_month_weekday_last::new({ January.as_pointer() }, { __tmp_4.as_pointer() })
    }));
    const _: () = assert!(true, "mwdl3.month() == January");;
    const _: () = assert!(true, "mwdl3.weekday_last() == weekday_last{{weekday{{4}}}}");;
    const _: () = assert!(true, "mwdl3.ok()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
