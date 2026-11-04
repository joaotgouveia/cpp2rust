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
    let January: Value<std_chrono_month> = Rc::new(RefCell::new(
        (*January_0.with(Value::clone).borrow()).clone(),
    ));
    let ymdl0: Value<std_chrono_year_month_day_last> = Rc::new(RefCell::new({
        let __tmp_0: Value<std_chrono_year> = Rc::new(RefCell::new(<std_chrono_year>::default()));
        let __tmp_1: Value<std_chrono_month_day_last> = Rc::new(RefCell::new({
            let __tmp_2: Value<std_chrono_month> =
                Rc::new(RefCell::new(<std_chrono_month>::default()));
            std_chrono_month_day_last::new({ __tmp_2.as_pointer() })
        }));
        std_chrono_year_month_day_last::new({ __tmp_0.as_pointer() }, { __tmp_1.as_pointer() })
    }));
    const _: () = assert!(true, "ymdl0.year()           == year{{}}");;
    const _: () = assert!(true, "ymdl0.month()          == month{{}}");;
    const _: () = assert!(
        true,
        "ymdl0.month_day_last() == month_day_last{{month{{}}}}"
    );;
    const _: () = assert!(true, "!ymdl0.ok()");;
    let ymdl1: Value<std_chrono_year_month_day_last> = Rc::new(RefCell::new({
        let __tmp_3: Value<std_chrono_year> =
            Rc::new(RefCell::new(std_chrono_year::new_1({ 2019 })));
        let __tmp_4: Value<std_chrono_month_day_last> =
            Rc::new(RefCell::new(std_chrono_month_day_last::new({
                January.as_pointer()
            })));
        std_chrono_year_month_day_last::new({ __tmp_3.as_pointer() }, { __tmp_4.as_pointer() })
    }));
    const _: () = assert!(true, "ymdl1.year()           == year{{2019}}");;
    const _: () = assert!(true, "ymdl1.month()          == January");;
    const _: () = assert!(true, "ymdl1.month_day_last() == month_day_last{{January}}");;
    const _: () = assert!(true, "ymdl1.ok()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
