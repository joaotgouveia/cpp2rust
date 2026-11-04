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
    let Sunday: Value<std_chrono_weekday> = Rc::new(RefCell::new(
        (*Sunday_1.with(Value::clone).borrow()).clone(),
    ));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "!month_weekday{{month{{}}, weekday_indexed{{}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "month_weekday{{std::chrono::May, weekday_indexed{{Sunday, 2}}}}.ok()"
    );;
    assert!(
        !({
            {
                let __tmp_0: Value<std_chrono_weekday_indexed> = Rc::new(RefCell::new(
                    std_chrono_weekday_indexed::new_1({ Sunday.as_pointer() }, { 0_u32 }),
                ));
                std_chrono_month_weekday::new({ April_2.with(|v| v.as_pointer()) }, {
                    __tmp_0.as_pointer()
                })
            }
            .ok()
        })
    );
    assert!(
        ({
            {
                let __tmp_1: Value<std_chrono_weekday_indexed> = Rc::new(RefCell::new(
                    std_chrono_weekday_indexed::new_1({ Sunday.as_pointer() }, { 1_u32 }),
                ));
                std_chrono_month_weekday::new({ March_3.with(|v| v.as_pointer()) }, {
                    __tmp_1.as_pointer()
                })
            }
            .ok()
        })
    );
    let mut i: u32 = 1_u32;
    'loop_: while (i <= 12_u32) {
        let mut j: u32 = 0_u32;
        'loop_: while (j <= 6_u32) {
            let mwd: Value<std_chrono_month_weekday> = Rc::new(RefCell::new({
                let __tmp_2: Value<std_chrono_month> =
                    Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                let __tmp_3: Value<std_chrono_weekday_indexed> = Rc::new(RefCell::new(
                    std_chrono_weekday_indexed::new_1({ Sunday.as_pointer() }, { j }),
                ));
                std_chrono_month_weekday::new({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
            }));
            assert!(
                ({
                    ((({ (*mwd.borrow()).ok() }) as i32) == (((j >= 1_u32) && (j <= 5_u32)) as i32))
                })
            );
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    let mut i: u32 = 1_u32;
    'loop_: while (i <= 10_u32) {
        assert!(
            !({
                {
                    let __tmp_4: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ 13_u32 })));
                    let __tmp_5: Value<std_chrono_weekday_indexed> = Rc::new(RefCell::new(
                        std_chrono_weekday_indexed::new_1({ Sunday.as_pointer() }, { i }),
                    ));
                    std_chrono_month_weekday::new({ __tmp_4.as_pointer() }, {
                        __tmp_5.as_pointer()
                    })
                }
                .ok()
            })
        );
        i.prefix_inc();
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
