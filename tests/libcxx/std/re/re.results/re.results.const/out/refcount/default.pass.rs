extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_convertible_imp_0(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_1(mut _a0: ()) -> () {
    return true;
}
pub fn test_convertible_imp_2(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_imp_3(mut _a0: ()) -> () {
    return false;
}
pub fn test_convertible_4() -> () {
    return ({ test_convertible_imp_0(0_()) });
}
pub fn test_convertible_5() -> () {
    return ({ test_convertible_imp_2(0_()) });
}
pub fn test_convertible_6() -> () {
    return ({ test_convertible_imp_1(0_()) });
}
pub fn test_convertible_7() -> () {
    return ({ test_convertible_imp_3(0_()) });
}
pub fn test_implicit_8() {
    const _: () = assert!(true, "test_convertible<T>()");;
    const _: () = assert!(true, "!test_convertible<T, typename T::allocator_type>()");;
}
pub fn test_implicit_9() {
    const _: () = assert!(true, "test_convertible<T>()");;
    const _: () = assert!(true, "!test_convertible<T, typename T::allocator_type>()");;
}
pub fn test_10() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ test_implicit_8() });
}
pub fn test_11() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    ({ test_implicit_9() });
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
    ({ test_10() });
    ({ test_11() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
