extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_is_same_0() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub fn test_is_same_1() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub fn test_is_same_2() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub fn test_is_same_3() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub fn test_is_same_ref_4() {
    const _: () = assert!(true, "(std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "(std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "(std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "(std::is_same_v<const T, const U>)");;
}
pub fn test_is_not_same_5() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub fn test_is_not_same_6() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub fn test_is_not_same_7() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub fn test_is_not_same_8() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub fn test_is_not_same_9() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct OverloadTest_char_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Class {}
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
    ({ test_is_same_0() });
    ({ test_is_same_1() });
    ({ test_is_same_2() });
    ({ test_is_same_3() });
    ({ test_is_same_ref_4() });
    ({ test_is_not_same_5() });
    ({ test_is_not_same_6() });
    ({ test_is_not_same_7() });
    ({ test_is_not_same_8() });
    ({ test_is_not_same_9() });
    let mut t: OverloadTest_char_ = <OverloadTest_char_>::default();
    &(t);
    return 0_();
}
pub trait ClassImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl ClassImpl for Ptr<Class> {}
pub trait OverloadTest_char_Impl {
    fn fn_1(&self, _a0: ()) {
        unimplemented!()
    }
    fn fn_2(&self, _a0: ()) {
        unimplemented!()
    }
    fn x(&self) {
        unimplemented!()
    }
}
impl OverloadTest_char_Impl for Ptr<OverloadTest_char_> {}
pub fn __cpp2rust_init_globals() {}
