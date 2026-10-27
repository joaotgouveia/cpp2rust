extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_function_imp_0() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "!std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub fn test_function_imp_1() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "!std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub fn test_function_imp_2() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "!std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub fn test_function_imp_3() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "!std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub fn test_function_4() {
    ({ test_function_imp_0() });
    ({ test_function_imp_0() });
    ({ test_function_imp_0() });
    ({ test_function_imp_0() });
}
pub fn test_function_5() {
    ({ test_function_imp_1() });
    ({ test_function_imp_1() });
    ({ test_function_imp_1() });
    ({ test_function_imp_1() });
}
pub fn test_function_6() {
    ({ test_function_imp_2() });
    ({ test_function_imp_2() });
    ({ test_function_imp_2() });
    ({ test_function_imp_2() });
}
pub fn test_function_7() {
    ({ test_function_imp_3() });
    ({ test_function_imp_3() });
    ({ test_function_imp_3() });
    ({ test_function_imp_3() });
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
    ({ test_function_4() });
    ({ test_function_5() });
    ({ test_function_6() });
    ({ test_function_7() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
