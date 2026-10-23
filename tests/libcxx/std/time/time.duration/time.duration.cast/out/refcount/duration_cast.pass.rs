extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_1(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_2(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_3(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_4(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_5(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_6(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub fn test_7(f: Ptr<()>, d: Ptr<()>) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_0.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(2_()));
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_1.as_pointer() })
        }));
        test_0(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_2.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<()> = Rc::new(RefCell::new(121_()));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_3.as_pointer() })
        }));
        test_1(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_4: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_4.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_5: Value<()> = Rc::new(RefCell::new(7265_()));
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ __tmp_5.as_pointer() })
        }));
        test_2(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_6: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_6.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_7: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_7.as_pointer() })
        }));
        test_3(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_8: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_8.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_9: Value<()> = Rc::new(RefCell::new(7265000000_()));
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ __tmp_9.as_pointer() })
        }));
        test_4(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_10: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_10.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_11: Value<()> = Rc::new(RefCell::new(7265000000000_()));
            std_chrono_duration_long_long__std_ratio_1__1000000000__::new_1({
                __tmp_11.as_pointer()
            })
        }));
        test_5(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_12: Value<()> = Rc::new(RefCell::new(7265000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_12.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_13: Value<()> = Rc::new(RefCell::new((7265_f64 / 3600_())));
            std_chrono_duration_double__std_ratio_3600__1__::new_1({ __tmp_13.as_pointer() })
        }));
        test_6(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_14: Value<()> = Rc::new(RefCell::new(9_()));
            std_chrono_duration_int__std_ratio_2__3__::new_1({ __tmp_14.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_15: Value<()> = Rc::new(RefCell::new(10_()));
            std_chrono_duration_int__std_ratio_3__5__::new_1({ __tmp_15.as_pointer() })
        }));
        test_7(_f.as_pointer(), _d.as_pointer())
    });
    {
        let h: Value<()> = Rc::new(RefCell::new(
            ({
                let ___fd: Value<()> = Rc::new(RefCell::new({
                    let __tmp_16: Value<()> = Rc::new(RefCell::new(7265000_()));
                    std_chrono_duration_long_long__std_ratio_1__1000__::new_1({
                        __tmp_16.as_pointer()
                    })
                }));
                duration_cast_8(___fd.as_pointer())
            }),
        ));
        const _: () = assert!(true, "h.count() == 2");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
