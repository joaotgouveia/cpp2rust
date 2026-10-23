extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static a_0: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..10).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
);
pub fn test_1() -> () {
    ({
        test_2(
            begin_3.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({ test_4(end_5.with(|v| v.as_pointer()), a_0.with(|v| v.as_pointer())) });
    ({
        test_6(
            cbegin_7.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_8(
            cdata_9.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_10(
            cend_11.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_12(
            crbegin_13.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_14(
            crend_15.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_16(
            data_17.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_18(
            empty_19.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_20(
            rbegin_21.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_22(
            rend_23.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_24(
            size_25.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    ({
        test_26(
            ssize_27.with(|v| v.as_pointer()),
            a_0.with(|v| v.as_pointer()),
        )
    });
    return true;
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = a_0.with(|_| ());
}
