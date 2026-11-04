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
        ({
            let _arg0: Value<std_tuple__> = Rc::new(RefCell::new(({ forward_as_tuple_0() })));
            test0_1(_arg0.as_pointer())
        });
    }
    {
        ({
            let _t: Value<std_tuple_int_refref_> = Rc::new(RefCell::new(
                ({
                    let ___args: Value<i32> = Rc::new(RefCell::new(1));
                    forward_as_tuple_2(___args.as_pointer())
                }),
            ));
            test1a_3(_t.as_pointer())
        });
    }
    {
        let i: Value<i32> = Rc::new(RefCell::new(2));
        ({
            let _t: Value<std_tuple_int_ref_> =
                Rc::new(RefCell::new(({ forward_as_tuple_4(i.as_pointer()) })));
            test1b_5(_t.as_pointer())
        });
    }
    {
        let i: Value<f64> = Rc::new(RefCell::new(2.5_f64));
        let c: Value<i8> = Rc::new(RefCell::new(('a' as i8)));
        ({
            let _t: Value<std_tuple_double_ref__char_ref_> = Rc::new(RefCell::new(
                ({ forward_as_tuple_6(i.as_pointer(), c.as_pointer()) }),
            ));
            test2a_7(_t.as_pointer())
        });
        const _: () = assert!(true, "test3 (std::forward_as_tuple(i, c)) == 2");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
