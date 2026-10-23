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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_0: Value<()> = Rc::new(RefCell::new(3_()));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_0.as_pointer() })
        }));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        let m: Value<()> = Rc::new(RefCell::new({
            let __tmp_1: Value<()> = Rc::new(RefCell::new(3_()));
            std_chrono_duration_long__std_ratio_60__1__::new_1({ __tmp_1.as_pointer() })
        }));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "m2.count() == -m.count()");;
    }
    {
        let zero: Value<()> = Rc::new(RefCell::new({
            let __tmp_2: Value<()> = Rc::new(RefCell::new(0_()));
            std_chrono_duration_int__std_ratio_10__10__::new_1({ __tmp_2.as_pointer() })
        }));
        let one: Value<()> = Rc::new(RefCell::new({
            let __tmp_3: Value<()> = Rc::new(RefCell::new(1_()));
            std_chrono_duration_int__std_ratio_10__10__::new_1({ __tmp_3.as_pointer() })
        }));
        const _: () = assert!(
            true,
            "(std::is_same< decltype(-one), decltype(zero-one) >::value)"
        );;
        const _: () = assert!(true, "(std::is_same< decltype(zero-one), D1>::value)");;
        const _: () = assert!(true, "(std::is_same< decltype(-one),     D1>::value)");;
        const _: () = assert!(true, "(std::is_same< decltype(+one),     D1>::value)");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
