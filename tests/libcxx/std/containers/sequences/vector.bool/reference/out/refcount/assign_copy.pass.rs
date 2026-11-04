extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let vec_: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
    {
        let a0_clone = true.clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    {
        let a0_clone = false.clone();
        (*vec_.borrow_mut()).push(a0_clone)
    };
    let mut ref1: Ptr<bool> = vec_.decay().offset(0_usize as isize);
    let ref2: Value<Ptr<bool>> = Rc::new(RefCell::new(vec_.decay().offset(1_usize as isize)));
    {
        (*ref2.borrow()).write(ref1.read());
        ({ (*ref2.borrow()).clone() })
    };
    {
        match ({ vec_.decay().offset(0_usize as isize) },) {
            (mut __tmp_0,) => {
                __tmp_0.write(false);
                (__tmp_0).clone()
            }
        };
        match ({ vec_.decay().offset(1_usize as isize) },) {
            (mut __tmp_1,) => {
                __tmp_1.write(true);
                (__tmp_1).clone()
            }
        };
        {
            ref1.write((*ref2.borrow()).read());
            (ref1).clone()
        };
        assert!(vec_.decay().offset(0_usize as isize).read());
        assert!(vec_.decay().offset(1_usize as isize).read());
    }
    {
        match ({ vec_.decay().offset(0_usize as isize) },) {
            (mut __tmp_2,) => {
                __tmp_2.write(true);
                (__tmp_2).clone()
            }
        };
        match ({ vec_.decay().offset(1_usize as isize) },) {
            (mut __tmp_3,) => {
                __tmp_3.write(false);
                (__tmp_3).clone()
            }
        };
        {
            ref1.write((*ref2.borrow()).read());
            (ref1).clone()
        };
        assert!(!(vec_.decay().offset(0_usize as isize).read()));
        assert!(!(vec_.decay().offset(1_usize as isize).read()));
    }
    {
        match ({ vec_.decay().offset(0_usize as isize) },) {
            (mut __tmp_4,) => {
                __tmp_4.write(false);
                (__tmp_4).clone()
            }
        };
        match ({ vec_.decay().offset(1_usize as isize) },) {
            (mut __tmp_5,) => {
                __tmp_5.write(true);
                (__tmp_5).clone()
            }
        };
        {
            ref1.write((*ref2.borrow()).read());
            (ref1).clone()
        };
        assert!(vec_.decay().offset(0_usize as isize).read());
        assert!(vec_.decay().offset(1_usize as isize).read());
    }
    {
        match ({ vec_.decay().offset(0_usize as isize) },) {
            (mut __tmp_6,) => {
                __tmp_6.write(true);
                (__tmp_6).clone()
            }
        };
        match ({ vec_.decay().offset(1_usize as isize) },) {
            (mut __tmp_7,) => {
                __tmp_7.write(false);
                (__tmp_7).clone()
            }
        };
        {
            ref1.write((*ref2.borrow()).read());
            (ref1).clone()
        };
        assert!(!(vec_.decay().offset(0_usize as isize).read()));
        assert!(!(vec_.decay().offset(1_usize as isize).read()));
    }
    {
        match ({ vec_.decay().offset(0_usize as isize) },) {
            (mut __tmp_8,) => {
                __tmp_8.write(false);
                (__tmp_8).clone()
            }
        };
        match ({ vec_.decay().offset(1_usize as isize) },) {
            (mut __tmp_9,) => {
                __tmp_9.write(true);
                (__tmp_9).clone()
            }
        };
        {
            ref1.write((*ref2.borrow()).read());
            (ref1).clone()
        };
        assert!(vec_.decay().offset(0_usize as isize).read());
        assert!(vec_.decay().offset(1_usize as isize).read());
    }
    {
        match ({ vec_.decay().offset(0_usize as isize) },) {
            (mut __tmp_10,) => {
                __tmp_10.write(true);
                (__tmp_10).clone()
            }
        };
        match ({ vec_.decay().offset(1_usize as isize) },) {
            (mut __tmp_11,) => {
                __tmp_11.write(false);
                (__tmp_11).clone()
            }
        };
        {
            ref1.write((*ref2.borrow()).read());
            (ref1).clone()
        };
        assert!(!(vec_.decay().offset(0_usize as isize).read()));
        assert!(!(vec_.decay().offset(1_usize as isize).read()));
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
