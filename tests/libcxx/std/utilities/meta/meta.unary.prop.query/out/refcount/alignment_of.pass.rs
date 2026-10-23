extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_alignment_of_0() {
    let mut AlignofResult: () = ::std::mem::align_of::<*mut ()>();
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_1() {
    let mut AlignofResult: () = 1usize;
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_2() {
    let mut AlignofResult: () = 8usize;
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_3() {
    let mut AlignofResult: () = 8usize;
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_4() {
    let mut AlignofResult: () = ::std::mem::align_of::<[(); 3]>();
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_5() {
    let mut AlignofResult: () = ::std::mem::align_of::<()>();
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_6() {
    let mut AlignofResult: () = ::std::mem::align_of::<()>();
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_7() {
    let mut AlignofResult: () = ::std::mem::align_of::<()>();
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
}
pub fn test_alignment_of_8() {
    let mut AlignofResult: () = ::std::mem::align_of::<()>();
    const _: () = assert!(true, "AlignofResult == A");;
    const _: () = assert!(true, "std::alignment_of<T>::value == AlignofResult");;
    const _: () = assert!(true, "std::alignment_of<T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of<const volatile T>::value == A");;
    const _: () = assert!(true, "std::alignment_of_v<T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<volatile T> == A");;
    const _: () = assert!(true, "std::alignment_of_v<const volatile T> == A");;
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
    ({ test_alignment_of_0() });
    ({ test_alignment_of_1() });
    ({ test_alignment_of_2() });
    ({ test_alignment_of_3() });
    ({ test_alignment_of_4() });
    ({ test_alignment_of_5() });
    ({ test_alignment_of_6() });
    ({ test_alignment_of_7() });
    ({ test_alignment_of_8() });
    return 0_();
}
pub trait ClassImpl {
    fn destructor(&self) {
        unimplemented!()
    }
}
impl ClassImpl for Ptr<Class> {}
pub fn __cpp2rust_init_globals() {}
