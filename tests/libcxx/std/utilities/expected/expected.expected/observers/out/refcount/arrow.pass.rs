extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(!(false), "!ArrowNoexcept<int>");
const _: () = assert!(true, "ArrowNoexcept<std::expected<int, int>>");
const _: () = assert!(true, "ArrowNoexcept<const std::expected<int, int>>");
pub fn test_0() -> bool {
    {
        let e: Value<std_expected_int__int_> = Rc::new(RefCell::new(5));
        let mut x: Ptr<i32> = ({ (*e.borrow()).operator_arrow_26() });
        assert!(({ ({ (x).clone() } == { ({ (*e.borrow()).value_34() }) }) }));
        assert!(((x.read()) == 5));
    }
    {
        let e: Value<std_expected_int__int_> = Rc::new(RefCell::new(5));
        let mut x: Ptr<i32> = ({ (*e.borrow()).operator_arrow_27() });
        assert!(({ ({ (x).clone() } == { ({ (*e.borrow()).value_35() }) }) }));
        assert!(((x.read()) == 5));
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
