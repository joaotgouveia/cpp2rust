extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_is_trivially_move_constructible_0() {
    const _: () = assert!(true, "std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible_v<T>");;
}
pub fn test_is_trivially_move_constructible_1() {
    const _: () = assert!(true, "std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible_v<T>");;
}
pub fn test_is_trivially_move_constructible_2() {
    const _: () = assert!(true, "std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible_v<T>");;
}
pub fn test_is_trivially_move_constructible_3() {
    const _: () = assert!(true, "std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible_v<T>");;
}
pub fn test_is_trivially_move_constructible_4() {
    const _: () = assert!(true, "std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible_v<T>");;
}
pub fn test_is_trivially_move_constructible_5() {
    const _: () = assert!(true, "std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible_v<T>");;
}
pub fn test_is_trivially_move_constructible_6() {
    const _: () = assert!(true, "std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_constructible_v<T>");;
}
pub fn test_has_not_trivial_move_constructor_7() {
    const _: () = assert!(true, "!std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_constructible_v<T>");;
}
pub fn test_has_not_trivial_move_constructor_8() {
    const _: () = assert!(true, "!std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_constructible_v<T>");;
}
pub fn test_has_not_trivial_move_constructor_9() {
    const _: () = assert!(true, "!std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_constructible_v<T>");;
}
pub fn test_has_not_trivial_move_constructor_10() {
    const _: () = assert!(true, "!std::is_trivially_move_constructible<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_constructible_v<T>");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct NotEmpty {}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(1)]
pub struct Union {
    #[offset(0)]
    #[byte_size(1)]
    __bytes: Box<[u8]>,
}
impl Union {}
impl Default for Union {
    fn default() -> Self {
        Union {
            __bytes: Box::from([0u8; 1]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct bit_zero {
    #[offset(0)]
    pub anon_11: (),
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
impl A {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MoveOnly1 {}
impl MoveOnly1 {}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct MoveOnly2 {}
impl MoveOnly2 {
    pub fn move_from(_a0: Ptr<MoveOnly2>) -> Self {
        Self {}
    }
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
    ({ test_has_not_trivial_move_constructor_7() });
    ({ test_has_not_trivial_move_constructor_8() });
    ({ test_has_not_trivial_move_constructor_9() });
    ({ test_has_not_trivial_move_constructor_10() });
    ({ test_is_trivially_move_constructible_0() });
    ({ test_is_trivially_move_constructible_1() });
    ({ test_is_trivially_move_constructible_2() });
    ({ test_is_trivially_move_constructible_3() });
    ({ test_is_trivially_move_constructible_4() });
    ({ test_is_trivially_move_constructible_5() });
    ({ test_is_trivially_move_constructible_6() });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_constructible<MoveOnly1>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_trivially_move_constructible<MoveOnly2>::value"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
