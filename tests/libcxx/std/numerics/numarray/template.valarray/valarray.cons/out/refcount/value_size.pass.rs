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
        let v: Value<std_valarray_int_> = Rc::new(RefCell::new({
            let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
            std_valarray_int_::new_1({ __tmp_0.as_pointer() }, { 100_usize })
        }));
        assert!(({ (({ (*v.borrow()).size() }) == 100_usize) }));
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            assert!(({ ({ (*v.borrow())[(i as usize)] } == 5) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_valarray_double_> = Rc::new(RefCell::new({
            let __tmp_1: Value<f64> = Rc::new(RefCell::new(2.5_f64));
            std_valarray_double_::new_1({ __tmp_1.as_pointer() }, { 100_usize })
        }));
        assert!(({ (({ (*v.borrow()).size() }) == 100_usize) }));
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            assert!(({ ({ (*v.borrow())[(i as usize)] } == 2.5_f64) }));
            i.prefix_inc();
        }
    }
    {
        let v: Value<std_valarray_std_valarray_double__> = Rc::new(RefCell::new({
            let __tmp_2: Value<std_valarray_double_> =
                Rc::new(RefCell::new(std_valarray_double_::new_2({ 10_usize })));
            std_valarray_std_valarray_double__::new_1({ __tmp_2.as_pointer() }, { 100_usize })
        }));
        assert!(({ (({ (*v.borrow()).size() }) == 100_usize) }));
        let mut i: i32 = 0;
        'loop_: while (i < 100) {
            assert!(({ (({ (*v.borrow())[(i as usize)].size() }) == 10_usize) }));
            i.prefix_inc();
        }
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
