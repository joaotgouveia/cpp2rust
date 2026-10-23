extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_rank_0() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_1() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_2() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_3() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_4() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_5() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_6() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_7() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_8() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_9() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_10() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
pub fn test_rank_11() {
    const _: () = assert!(true, "std::rank<T>::value == A");;
    const _: () = assert!(true, "std::rank<const T>::value == A");;
    const _: () = assert!(true, "std::rank<volatile T>::value == A");;
    const _: () = assert!(true, "std::rank<const volatile T>::value == A");;
    const _: () = assert!(true, "std::rank_v<T> == A");;
    const _: () = assert!(true, "std::rank_v<const T> == A");;
    const _: () = assert!(true, "std::rank_v<volatile T> == A");;
    const _: () = assert!(true, "std::rank_v<const volatile T> == A");;
}
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
    ({ test_rank_0() });
    ({ test_rank_1() });
    ({ test_rank_2() });
    ({ test_rank_3() });
    ({ test_rank_4() });
    ({ test_rank_5() });
    ({ test_rank_6() });
    ({ test_rank_7() });
    ({ test_rank_8() });
    ({ test_rank_9() });
    ({ test_rank_10() });
    ({ test_rank_11() });
    return 0_();
}
pub trait ClassImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl ClassImpl for Ptr<Class> {}
pub fn __cpp2rust_init_globals() {}
