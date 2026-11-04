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
    const _: () = assert!(true, "check_hours(std::chrono::minutes( 1)) == 0");;
    const _: () = assert!(true, "check_hours(std::chrono::minutes(-1)) == 0");;
    assert!(
        (({
            check_hours_0({
                let __tmp_0: Value<i32> = Rc::new(RefCell::new(5000));
                std_chrono_duration_long__std_ratio_1__1__::new_1({ __tmp_0.as_pointer() })
            })
        }) == 1_i64)
    );
    assert!(
        (({
            check_hours_0({
                let __tmp_1: Value<i32> = Rc::new(RefCell::new(-5000_i32));
                std_chrono_duration_long__std_ratio_1__1__::new_1({ __tmp_1.as_pointer() })
            })
        }) == 1_i64)
    );
    assert!(
        (({
            check_hours_1({
                let __tmp_2: Value<i32> = Rc::new(RefCell::new(5000));
                std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_2.as_pointer() })
            })
        }) == 83_i64)
    );
    assert!(
        (({
            check_hours_1({
                let __tmp_3: Value<i32> = Rc::new(RefCell::new(-5000_i32));
                std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_3.as_pointer() })
            })
        }) == 83_i64)
    );
    assert!(
        (({
            check_hours_2({
                let __tmp_4: Value<i32> = Rc::new(RefCell::new(11));
                std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_4.as_pointer() })
            })
        }) == 11_i64)
    );
    assert!(
        (({
            check_hours_2({
                let __tmp_5: Value<i32> = Rc::new(RefCell::new(-11_i32));
                std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_5.as_pointer() })
            })
        }) == 11_i64)
    );
    assert!(
        (({
            check_hours_3({
                let __tmp_6: Value<i64> = Rc::new(RefCell::new(123456789_i64));
                std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_6.as_pointer() })
            })
        }) == 34_i64)
    );
    assert!(
        (({
            check_hours_3({
                let __tmp_7: Value<i64> = Rc::new(RefCell::new(-123456789_i64));
                std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_7.as_pointer() })
            })
        }) == 34_i64)
    );
    assert!(
        (({
            check_hours_4({
                let __tmp_8: Value<i64> = Rc::new(RefCell::new(123456789_i64));
                std_chrono_duration_long__std_ratio_1__1000000__::new_1({ __tmp_8.as_pointer() })
            })
        }) == 0_i64)
    );
    assert!(
        (({
            check_hours_4({
                let __tmp_9: Value<i64> = Rc::new(RefCell::new(-123456789_i64));
                std_chrono_duration_long__std_ratio_1__1000000__::new_1({ __tmp_9.as_pointer() })
            })
        }) == 0_i64)
    );
    assert!(
        (({
            check_hours_5({
                let __tmp_10: Value<i64> = Rc::new(RefCell::new(123456789_i64));
                std_chrono_duration_long__std_ratio_1__1000000000__::new_1({
                    __tmp_10.as_pointer()
                })
            })
        }) == 0_i64)
    );
    assert!(
        (({
            check_hours_5({
                let __tmp_11: Value<i64> = Rc::new(RefCell::new(-123456789_i64));
                std_chrono_duration_long__std_ratio_1__1000000000__::new_1({
                    __tmp_11.as_pointer()
                })
            })
        }) == 0_i64)
    );
    assert!(
        (({
            check_hours_6({
                let __tmp_12: Value<i32> = Rc::new(RefCell::new(1000));
                std_chrono_duration_int__std_ratio_756__625__::new_1({ __tmp_12.as_pointer() })
            })
        }) == 0_i64)
    );
    assert!(
        (({
            check_hours_6({
                let __tmp_13: Value<i32> = Rc::new(RefCell::new(-1000_i32));
                std_chrono_duration_int__std_ratio_756__625__::new_1({ __tmp_13.as_pointer() })
            })
        }) == 0_i64)
    );
    assert!(
        (({
            check_hours_6({
                let __tmp_14: Value<i32> = Rc::new(RefCell::new(10000));
                std_chrono_duration_int__std_ratio_756__625__::new_1({ __tmp_14.as_pointer() })
            })
        }) == 3_i64)
    );
    assert!(
        (({
            check_hours_6({
                let __tmp_15: Value<i32> = Rc::new(RefCell::new(-10000_i32));
                std_chrono_duration_int__std_ratio_756__625__::new_1({ __tmp_15.as_pointer() })
            })
        }) == 3_i64)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
