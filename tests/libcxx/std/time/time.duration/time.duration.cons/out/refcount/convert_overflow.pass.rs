extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static called_0: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn f_1(_a0: std_chrono_duration_long__std_ratio_1__1__) {
    let _a0: Value<std_chrono_duration_long__std_ratio_1__1__> = Rc::new(RefCell::new(_a0));
    called_0.with(|rc| *rc.borrow_mut() = true);
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
    {
        let r: Value<std_chrono_duration_int__std_ratio_1000000000000000000__1__> =
            Rc::new(RefCell::new({
                let __tmp_0: Value<i32> = Rc::new(RefCell::new(1));
                std_chrono_duration_int__std_ratio_1000000000000000000__1__::new_1({
                    __tmp_0.as_pointer()
                })
            }));
        ({
            f_1(std_chrono_duration_long__std_ratio_1__1__::new_1({
                r.as_pointer()
            }))
        });
        assert!(called_0.with(|rc| *rc.borrow()));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = called_0.with(|_| ());
}
