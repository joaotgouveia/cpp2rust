extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_implicit_0() -> () {
    return unimplemented!();
}
pub fn test_convertible_imp_1(mut _a0: i32) -> bool {
    return true;
}
pub fn test_convertible_imp_2(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_3(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_4(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_imp_5(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_6() -> bool {
    return ({ test_convertible_imp_1(0) });
}
pub fn test_convertible_7() -> bool {
    return ({ test_convertible_imp_2(0_i64) });
}
pub fn test_convertible_8() -> bool {
    return ({ test_convertible_imp_3(0_i64) });
}
pub fn test_convertible_9() -> bool {
    return ({ test_convertible_imp_4(0_i64) });
}
pub fn test_convertible_10() -> bool {
    return ({ test_convertible_imp_5(0_i64) });
}
pub fn test_implicit_11() {
    const _: () = assert!(true, "test_convertible<D>()");;
    assert!(unimplemented!());
    const _: () = assert!(true, "!test_convertible<D, T>()");;
    const _: () = assert!(true, "!test_convertible<D, T, T>()");;
}
pub fn test_implicit_12() {
    const _: () = assert!(true, "test_convertible<D>()");;
    assert!(unimplemented!());
    const _: () = assert!(true, "!test_convertible<D, T>()");;
    const _: () = assert!(true, "!test_convertible<D, T, T>()");;
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
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 0));
        assert!((unimplemented!() == i32::MAX));
    }
    {
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == -6_i32));
        assert!((unimplemented!() == i32::MAX));
    }
    {
        let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == -6_i32));
        assert!((unimplemented!() == 106));
    }
    ({ test_implicit_11() });
    ({ test_implicit_12() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
