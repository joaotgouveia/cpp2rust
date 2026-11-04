extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
impl CppMove for test_macros_detail_is_same_bool__bool_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let January: Value<std_chrono_month> = Rc::new(RefCell::new(
        (*January_1.with(Value::clone).borrow()).clone(),
    ));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, month{{}}, day{{}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, January, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   month{{}}, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   January, day{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, month{{}}, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   month{{}}, day{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, January, day{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2019}},   January, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 1}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 2}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 3}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 4}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 5}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 6}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 7}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 8}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 9}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{10}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{11}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{12}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 1}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 2}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 3}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 4}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 5}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 6}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 7}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 8}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 9}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{10}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{11}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{12}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   std::chrono::February, day{{29}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   std::chrono::February, day{{29}}}}.ok()"
    );;
    let mut i: u32 = 0_u32;
    'loop_: while (i <= 50_u32) {
        let ym: Value<std_chrono_year_month_day> = Rc::new(RefCell::new({
            let __tmp_0: Value<std_chrono_year> =
                Rc::new(RefCell::new(std_chrono_year::new_1({ 2019 })));
            let __tmp_1: Value<u32> = Rc::new(RefCell::new(i));
            std_chrono_year_month_day::new_1({ __tmp_0.as_pointer() }, { January.as_pointer() }, {
                __tmp_1.as_pointer()
            })
        }));
        assert!(((({ (*ym.borrow()).ok() }) as i32) == (({ i.ok() }) as i32)));
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while (i <= 50_u32) {
        let ym: Value<std_chrono_year_month_day> = Rc::new(RefCell::new({
            let __tmp_2: Value<std_chrono_year> =
                Rc::new(RefCell::new(std_chrono_year::new_1({ 2019 })));
            let __tmp_3: Value<std_chrono_month> =
                Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
            let __tmp_4: Value<u32> = Rc::new(RefCell::new(12_u32));
            std_chrono_year_month_day::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() }, {
                __tmp_4.as_pointer()
            })
        }));
        assert!(
            ((({ (*ym.borrow()).ok() }) as i32)
                == (({ std_chrono_month::new_1({ i },).ok() }) as i32))
        );
        i.prefix_inc();
    }
    let mut ymax: i32 = ({ ({ std_chrono_year::max() })() });
    let mut i: i32 = (ymax - 100);
    'loop_: while (i <= (ymax + 100)) {
        let ym: Value<std_chrono_year_month_day> = Rc::new(RefCell::new({
            let __tmp_5: Value<std_chrono_year> =
                Rc::new(RefCell::new(std_chrono_year::new_1({ i })));
            let __tmp_6: Value<u32> = Rc::new(RefCell::new(12_u32));
            std_chrono_year_month_day::new_1({ __tmp_5.as_pointer() }, { January.as_pointer() }, {
                __tmp_6.as_pointer()
            })
        }));
        assert!(
            ((({ (*ym.borrow()).ok() }) as i32)
                == (({ std_chrono_year::new_1({ i },).ok() }) as i32))
        );
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
