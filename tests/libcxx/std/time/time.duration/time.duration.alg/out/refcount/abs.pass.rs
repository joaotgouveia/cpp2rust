extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static has_abs_v_0: Value<bool> = Rc::new(RefCell::new(true));
);
thread_local!(
    pub static has_abs_v_1: Value<bool> = Rc::new(RefCell::new(false));
);
const _: () = assert!(true, "has_abs_v<std::chrono::milliseconds>");
const _: () = assert!(true, "!has_abs_v<std::chrono::duration<unsigned>>");
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
        let _f: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(7290000));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_0.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_1: Value<i32> = Rc::new(RefCell::new(7290000));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_1.as_pointer() })
        }));
        test_2(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_2: Value<i32> = Rc::new(RefCell::new(-7290000_i32));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_2.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_1__1000__> = Rc::new(RefCell::new({
            let __tmp_3: Value<i32> = Rc::new(RefCell::new(7290000));
            std_chrono_duration_long__std_ratio_1__1000__::new_1({ __tmp_3.as_pointer() })
        }));
        test_2(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<std_chrono_duration_long__std_ratio_60__1__> = Rc::new(RefCell::new({
            let __tmp_4: Value<i32> = Rc::new(RefCell::new(122));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_4.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_60__1__> = Rc::new(RefCell::new({
            let __tmp_5: Value<i32> = Rc::new(RefCell::new(122));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_5.as_pointer() })
        }));
        test_3(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<std_chrono_duration_long__std_ratio_60__1__> = Rc::new(RefCell::new({
            let __tmp_6: Value<i32> = Rc::new(RefCell::new(-122_i32));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_6.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_60__1__> = Rc::new(RefCell::new({
            let __tmp_7: Value<i32> = Rc::new(RefCell::new(122));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_7.as_pointer() })
        }));
        test_3(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<std_chrono_duration_long__std_ratio_3600__1__> = Rc::new(RefCell::new({
            let __tmp_8: Value<i32> = Rc::new(RefCell::new(0));
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_8.as_pointer() })
        }));
        let _d: Value<std_chrono_duration_long__std_ratio_3600__1__> = Rc::new(RefCell::new({
            let __tmp_9: Value<i32> = Rc::new(RefCell::new(0));
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_9.as_pointer() })
        }));
        test_4(_f.as_pointer(), _d.as_pointer())
    });
    {
        let h1: Value<std_chrono_duration_long__std_ratio_3600__1__> = Rc::new(RefCell::new(
            ({
                abs_5({
                    let __tmp_10: Value<i32> = Rc::new(RefCell::new(-3_i32));
                    std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_10.as_pointer() })
                })
            }),
        ));
        const _: () = assert!(true, "h1.count() == 3");;
        let h2: Value<std_chrono_duration_long__std_ratio_3600__1__> = Rc::new(RefCell::new(
            ({
                abs_5({
                    let __tmp_11: Value<i32> = Rc::new(RefCell::new(3));
                    std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_11.as_pointer() })
                })
            }),
        ));
        const _: () = assert!(true, "h2.count() == 3");;
    }
    {
        let d: Value<std_chrono_duration_int__std_ratio_60__100__> = Rc::new(RefCell::new(
            ({
                abs_6({
                    let __tmp_12: Value<i32> = Rc::new(RefCell::new(2));
                    std_chrono_duration_int__std_ratio_60__100__::new_1({ __tmp_12.as_pointer() })
                })
            }),
        ));
        const _: () = assert!(true, "d.count() == 2");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = has_abs_v_0.with(|_| ());
    let _ = has_abs_v_1.with(|_| ());
}
