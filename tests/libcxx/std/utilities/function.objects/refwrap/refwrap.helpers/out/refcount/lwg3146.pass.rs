extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        let ri: Value<std_reference_wrapper_int_> =
            Rc::new(RefCell::new(std_reference_wrapper_int_::new({
                i.as_pointer()
            })));
        let rri: Value<std_reference_wrapper_std_reference_wrapper_int__> = Rc::new(RefCell::new(
            std_reference_wrapper_std_reference_wrapper_int__::new_1({ ri.as_pointer() }),
        ));
        let rrj: Value<std_reference_wrapper_std_reference_wrapper_int__> =
            Rc::new(RefCell::new(({ ref_1((*rri.borrow()).clone()) })));
        const _: () = assert!(true, "");;
        ({ cpp2rust_assert_fail_2((({ (*rrj.borrow()).get() }) == (ri.as_pointer()))) });
    }
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        let ri: Value<std_reference_wrapper_int_> =
            Rc::new(RefCell::new(std_reference_wrapper_int_::new({
                i.as_pointer()
            })));
        let rri: Value<std_reference_wrapper_const_std_reference_wrapper_int__> =
            Rc::new(RefCell::new(
                std_reference_wrapper_const_std_reference_wrapper_int__::new_1({ ri.as_pointer() }),
            ));
        let rrj: Value<std_reference_wrapper_const_std_reference_wrapper_int__> =
            Rc::new(RefCell::new(({ ref_3((*rri.borrow()).clone()) })));
        const _: () = assert!(true, "");;
        ({ cpp2rust_assert_fail_2((({ (*rrj.borrow()).get() }) == (ri.as_pointer()))) });
    }
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        let ri: Value<std_reference_wrapper_int_> =
            Rc::new(RefCell::new(std_reference_wrapper_int_::new({
                i.as_pointer()
            })));
        let rri: Value<std_reference_wrapper_std_reference_wrapper_int__> = Rc::new(RefCell::new(
            std_reference_wrapper_std_reference_wrapper_int__::new_1({ ri.as_pointer() }),
        ));
        let rrj: Value<std_reference_wrapper_const_std_reference_wrapper_int__> =
            Rc::new(RefCell::new(({ cref_4((*rri.borrow()).clone()) })));
        const _: () = assert!(true, "");;
        ({ cpp2rust_assert_fail_2((({ (*rrj.borrow()).get() }) == (ri.as_pointer()))) });
    }
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        let ri: Value<std_reference_wrapper_int_> =
            Rc::new(RefCell::new(std_reference_wrapper_int_::new({
                i.as_pointer()
            })));
        let rri: Value<std_reference_wrapper_const_std_reference_wrapper_int__> =
            Rc::new(RefCell::new(
                std_reference_wrapper_const_std_reference_wrapper_int__::new_1({ ri.as_pointer() }),
            ));
        let rrj: Value<std_reference_wrapper_const_std_reference_wrapper_int__> =
            Rc::new(RefCell::new(({ cref_5((*rri.borrow()).clone()) })));
        const _: () = assert!(true, "");;
        ({ cpp2rust_assert_fail_2((({ (*rrj.borrow()).get() }) == (ri.as_pointer()))) });
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
