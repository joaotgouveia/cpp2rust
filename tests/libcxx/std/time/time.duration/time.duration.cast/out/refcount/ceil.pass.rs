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
            let __tmp_0: Value<()> = Rc::new(RefCell::new(7290000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_0.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(3_()));
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_1.as_pointer() })
        }));
        test_0(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(-7290000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_2.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<()> = Rc::new(RefCell::new(-2_()));
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ __tmp_3.as_pointer() })
        }));
        test_0(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_4: Value<()> = Rc::new(RefCell::new(7290000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_4.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_5: Value<()> = Rc::new(RefCell::new(122_()));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_5.as_pointer() })
        }));
        test_1(_f.as_pointer(), _d.as_pointer())
    });
    ({
        let _f: Value<()> = Rc::new(RefCell::new({
            let __tmp_6: Value<()> = Rc::new(RefCell::new(-7290000_()));
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ __tmp_6.as_pointer() })
        }));
        let _d: Value<()> = Rc::new(RefCell::new({
            let __tmp_7: Value<()> = Rc::new(RefCell::new(-121_()));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_7.as_pointer() })
        }));
        test_1(_f.as_pointer(), _d.as_pointer())
    });
    {
        let h1: Value<()> = Rc::new(RefCell::new(
            ({
                let ___d: Value<()> = Rc::new(RefCell::new({
                    let __tmp_8: Value<()> = Rc::new(RefCell::new(9000000_()));
                    std_chrono_duration_long_long__std_ratio_1__1000__::new_1({
                        __tmp_8.as_pointer()
                    })
                }));
                ceil_2(___d.as_pointer())
            }),
        ));
        const _: () = assert!(true, "h1.count() == 3");;
        let h2: Value<()> = Rc::new(RefCell::new(
            ({
                let ___d: Value<()> = Rc::new(RefCell::new({
                    let __tmp_9: Value<()> = Rc::new(RefCell::new(-9000000_()));
                    std_chrono_duration_long_long__std_ratio_1__1000__::new_1({
                        __tmp_9.as_pointer()
                    })
                }));
                ceil_2(___d.as_pointer())
            }),
        ));
        const _: () = assert!(true, "h2.count() == -2");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
