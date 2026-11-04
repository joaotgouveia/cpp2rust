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
    {
        let x: Value<i32> = Rc::new(RefCell::new(0));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        ({
            let _a: Ptr<i32> = x.as_pointer();
            let _b: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            let _y: Ptr<i32> = y.as_pointer();
            test_0(_a, _b, <std_greater_int_>::default(), _x, _y)
        });
        ({
            let _a: Ptr<i32> = y.as_pointer();
            let _b: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            let _y: Ptr<i32> = x.as_pointer();
            test_0(_a, _b, <std_greater_int_>::default(), _x, _y)
        });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(0));
        let y: Value<i32> = Rc::new(RefCell::new(1));
        ({
            let _a: Ptr<i32> = x.as_pointer();
            let _b: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            let _y: Ptr<i32> = x.as_pointer();
            test_0(_a, _b, <std_greater_int_>::default(), _x, _y)
        });
        ({
            let _a: Ptr<i32> = y.as_pointer();
            let _b: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = y.as_pointer();
            let _y: Ptr<i32> = x.as_pointer();
            test_0(_a, _b, <std_greater_int_>::default(), _x, _y)
        });
    }
    {
        let x: Value<i32> = Rc::new(RefCell::new(1));
        let y: Value<i32> = Rc::new(RefCell::new(0));
        ({
            let _a: Ptr<i32> = x.as_pointer();
            let _b: Ptr<i32> = y.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            let _y: Ptr<i32> = y.as_pointer();
            test_0(_a, _b, <std_greater_int_>::default(), _x, _y)
        });
        ({
            let _a: Ptr<i32> = y.as_pointer();
            let _b: Ptr<i32> = x.as_pointer();
            let _x: Ptr<i32> = x.as_pointer();
            let _y: Ptr<i32> = y.as_pointer();
            test_0(_a, _b, <std_greater_int_>::default(), _x, _y)
        });
    }
    {
        let p1: Value<std_pair_const_int_ref__const_int_ref_> = Rc::new(RefCell::new(
            ({
                minmax_3(
                    x_1.with(|v| v.as_pointer()),
                    y_2.with(|v| v.as_pointer()),
                    <std_greater_void_>::default(),
                )
            }),
        ));
        const _: () = assert!(true, "p1.first  == x");;
        const _: () = assert!(true, "p1.second == y");;
        let p2: Value<std_pair_const_int_ref__const_int_ref_> = Rc::new(RefCell::new(
            ({
                minmax_3(
                    y_2.with(|v| v.as_pointer()),
                    x_1.with(|v| v.as_pointer()),
                    <std_greater_void_>::default(),
                )
            }),
        ));
        const _: () = assert!(true, "p2.first  == x");;
        const _: () = assert!(true, "p2.second == y");;
    }
    return 0;
}
thread_local!(
    static x_1: Value<i32> = Rc::new(RefCell::new(1));
);
thread_local!(
    static y_2: Value<i32> = Rc::new(RefCell::new(0));
);
pub fn __cpp2rust_init_globals() {}
