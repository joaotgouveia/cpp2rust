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
    let ip: Value<(Value<Ptr<i32>>, Value<i64>)> =
        Rc::new(RefCell::new(({ get_temporary_buffer_0(5_isize) })));
    assert!(({ !(*(*ip.borrow()).0.borrow()).is_null() }));
    assert!(({ ((*(*ip.borrow()).1.borrow()) == 5_i64) }));
    ({ return_temporary_buffer_1((*(*ip.borrow()).0.borrow()).clone()) });
    {
        let ret: Value<(Value<Ptr<i32>>, Value<i64>)> =
            Rc::new(RefCell::new(({ get_temporary_buffer_0(0_isize) })));
        assert!(({ (*(*ret.borrow()).0.borrow()).is_null() }));
        assert!(({ ((*(*ret.borrow()).1.borrow()) == 0_i64) }));
    }
    {
        let ret: Value<(Value<Ptr<i32>>, Value<i64>)> = Rc::new(RefCell::new(
            ({ get_temporary_buffer_0((-5_i32 as isize)) }),
        ));
        assert!(({ (*(*ret.borrow()).0.borrow()).is_null() }));
        assert!(({ ((*(*ret.borrow()).1.borrow()) == 0_i64) }));
    }
    {
        let ret: Value<(Value<Ptr<i32>>, Value<i64>)> = Rc::new(RefCell::new(
            ({ get_temporary_buffer_0((i64::MIN as isize)) }),
        ));
        assert!(({ (*(*ret.borrow()).0.borrow()).is_null() }));
        assert!(({ ((*(*ret.borrow()).1.borrow()) == 0_i64) }));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
