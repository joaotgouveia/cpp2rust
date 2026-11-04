extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static has_ceil_v_0: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_ceil_v_1: Value<bool> = Rc::new(RefCell::new(false));
);
const _: () = assert!(true, "has_ceil_v<std::chrono::seconds>");
const _: () = assert!(true, "!has_ceil_v<int>");
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
    ({
        let _df: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(7290000));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_0.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_3600__1__> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(3));
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_1.as_pointer() })
        }));
        test_2(_df.as_pointer(), _d.as_pointer())
    });
    ({
        let _df: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(-7290000_i32));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_2.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_3600__1__> = Rc::new(RefCell::new({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(-2_i32));
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_3.as_pointer() })
        }));
        test_2(_df.as_pointer(), _d.as_pointer())
    });
    ({
        let _df: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(7290000));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_4.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_60__1__> = Rc::new(RefCell::new({
            let __tmp_5: Value<i32> = Rc::new(RefCell::new(122));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_5.as_pointer() })
        }));
        test_3(_df.as_pointer(), _d.as_pointer())
    });
    ({
        let _df: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_6: Value<i32> = Rc::new(RefCell::new(-7290000_i32));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_6.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_60__1__> = Rc::new(RefCell::new({
            let __tmp_7: Value<i32> = Rc::new(RefCell::new(-121_i32));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_7.as_pointer() })
        }));
        test_3(_df.as_pointer(), _d.as_pointer())
    });
    ({ test_constexpr_4() });
    ({ test_constexpr_5() });
    ({ test_constexpr_6() });
    ({ test_constexpr_7() });
    ({ test_constexpr_8() });
    ({ test_constexpr_9() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = has_ceil_v_0.with(|_| ());
    let _ = has_ceil_v_1.with(|_| ());
}
