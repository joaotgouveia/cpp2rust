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
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "year_month_weekday{{}}.weekday_indexed() == weekday_indexed{{}}"
    );;
    let mut i: u32 = 1_u32;
    'loop_: while (i <= 50_u32) {
        let ymwd0: Value<std_chrono_year_month_weekday> = Rc::new(RefCell::new({
            let __tmp_0: Value<std_chrono_year> =
                Rc::new(RefCell::new(std_chrono_year::new_1({ 1234 })));
            let __tmp_1: Value<std_chrono_month> =
                Rc::new(RefCell::new(std_chrono_month::new_1({ 2_u32 })));
            let __tmp_2: Value<std_chrono_weekday_indexed> = Rc::new(RefCell::new({
                let __tmp_3: Value<std_chrono_weekday> =
                    Rc::new(RefCell::new(std_chrono_weekday::new_1({ i })));
                std_chrono_weekday_indexed::new_1({ __tmp_3.as_pointer() }, { 1_u32 })
            }));
            std_chrono_year_month_weekday::new_1(
                { __tmp_0.as_pointer() },
                { __tmp_1.as_pointer() },
                { __tmp_2.as_pointer() },
            )
        }));
        assert!(
            (({ ({ ({ (*ymwd0.borrow()).weekday_indexed() }).weekday() }).c_encoding() })
                == (if (i == 7_u32) { 0_u32 } else { i }))
        );
        assert!((({ ({ (*ymwd0.borrow()).weekday_indexed() }).index() }) == 1_u32));
        let ymwd1: Value<std_chrono_year_month_weekday> = Rc::new(RefCell::new({
            let __tmp_4: Value<std_chrono_year> =
                Rc::new(RefCell::new(std_chrono_year::new_1({ 1234 })));
            let __tmp_5: Value<std_chrono_month> =
                Rc::new(RefCell::new(std_chrono_month::new_1({ 2_u32 })));
            let __tmp_6: Value<std_chrono_weekday_indexed> = Rc::new(RefCell::new({
                let __tmp_7: Value<std_chrono_weekday> =
                    Rc::new(RefCell::new(std_chrono_weekday::new_1({ 2_u32 })));
                std_chrono_weekday_indexed::new_1({ __tmp_7.as_pointer() }, { i })
            }));
            std_chrono_year_month_weekday::new_1(
                { __tmp_4.as_pointer() },
                { __tmp_5.as_pointer() },
                { __tmp_6.as_pointer() },
            )
        }));
        assert!(
            (({ ({ ({ (*ymwd1.borrow()).weekday_indexed() }).weekday() }).c_encoding() }) == 2_u32)
        );
        assert!((({ ({ (*ymwd1.borrow()).weekday_indexed() }).index() }) == i));
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
