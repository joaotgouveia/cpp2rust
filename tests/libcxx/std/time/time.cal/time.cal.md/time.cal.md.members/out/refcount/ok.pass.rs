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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "!month_day{{}}.ok()");;
    const _: () = assert!(true, "month_day{{std::chrono::May, day{{2}}}}.ok()");;
    assert!(
        !({
            {
                let __tmp_0: Value<u32> = Rc::new(RefCell::new(0_u32));
                std_chrono_month_day::new_1({ April_1.with(|v| v.as_pointer()) }, {
                    __tmp_0.as_pointer()
                })
            }
            .ok()
        })
    );
    assert!(
        ({
            {
                let __tmp_1: Value<u32> = Rc::new(RefCell::new(1_u32));
                std_chrono_month_day::new_1({ March_2.with(|v| v.as_pointer()) }, {
                    __tmp_1.as_pointer()
                })
            }
            .ok()
        })
    );
    let mut i: u32 = 1_u32;
    'loop_: while (i <= 12_u32) {
        let mut is31: bool = ((((((i == 1_u32) || (i == 3_u32)) || (i == 5_u32)) || (i == 7_u32))
            || (i == 8_u32))
            || (i == 10_u32))
            || (i == 12_u32);
        assert!(
            !({
                {
                    let __tmp_2: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                    let __tmp_3: Value<u32> = Rc::new(RefCell::new(0_u32));
                    std_chrono_month_day::new_1({ __tmp_2.as_pointer() }, { __tmp_3.as_pointer() })
                }
                .ok()
            })
        );
        assert!(
            ({
                {
                    let __tmp_4: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                    let __tmp_5: Value<u32> = Rc::new(RefCell::new(1_u32));
                    std_chrono_month_day::new_1({ __tmp_4.as_pointer() }, { __tmp_5.as_pointer() })
                }
                .ok()
            })
        );
        assert!(
            ({
                {
                    let __tmp_6: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                    let __tmp_7: Value<u32> = Rc::new(RefCell::new(10_u32));
                    std_chrono_month_day::new_1({ __tmp_6.as_pointer() }, { __tmp_7.as_pointer() })
                }
                .ok()
            })
        );
        assert!(
            ({
                {
                    let __tmp_8: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                    let __tmp_9: Value<u32> = Rc::new(RefCell::new(29_u32));
                    std_chrono_month_day::new_1({ __tmp_8.as_pointer() }, { __tmp_9.as_pointer() })
                }
                .ok()
            })
        );
        assert!(
            ((({
                {
                    let __tmp_10: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                    let __tmp_11: Value<u32> = Rc::new(RefCell::new(30_u32));
                    std_chrono_month_day::new_1({ __tmp_10.as_pointer() }, {
                        __tmp_11.as_pointer()
                    })
                }
                .ok()
            }) as i32)
                == ((i != 2_u32) as i32))
        );
        assert!(
            ((({
                {
                    let __tmp_12: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                    let __tmp_13: Value<u32> = Rc::new(RefCell::new(31_u32));
                    std_chrono_month_day::new_1({ __tmp_12.as_pointer() }, {
                        __tmp_13.as_pointer()
                    })
                }
                .ok()
            }) as i32)
                == (is31 as i32))
        );
        assert!(
            !({
                {
                    let __tmp_14: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ i })));
                    let __tmp_15: Value<u32> = Rc::new(RefCell::new(32_u32));
                    std_chrono_month_day::new_1({ __tmp_14.as_pointer() }, {
                        __tmp_15.as_pointer()
                    })
                }
                .ok()
            })
        );
        i.prefix_inc();
    }
    let mut i: u32 = 1_u32;
    'loop_: while (i <= 35_u32) {
        assert!(
            !({
                {
                    let __tmp_16: Value<std_chrono_month> =
                        Rc::new(RefCell::new(std_chrono_month::new_1({ 13_u32 })));
                    let __tmp_17: Value<u32> = Rc::new(RefCell::new(i));
                    std_chrono_month_day::new_1({ __tmp_16.as_pointer() }, {
                        __tmp_17.as_pointer()
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
