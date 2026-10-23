extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_has_trivial_assign_0() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_trivial_assign_1() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_trivial_assign_2() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_trivial_assign_3() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_trivial_assign_4() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_trivial_assign_5() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_trivial_assign_6() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_trivial_assign_7() {
    const _: () = assert!(true, "std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_not_trivial_assign_8() {
    const _: () = assert!(true, "!std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_not_trivial_assign_9() {
    const _: () = assert!(true, "!std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_not_trivial_assign_10() {
    const _: () = assert!(true, "!std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_not_trivial_assign_11() {
    const _: () = assert!(true, "!std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable_v<T>");;
}
pub fn test_has_not_trivial_assign_12() {
    const _: () = assert!(true, "!std::is_trivially_move_assignable<T>::value");;
    const _: () = assert!(true, "!std::is_trivially_move_assignable_v<T>");;
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
    pub anon_13: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
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
    ({ test_has_trivial_assign_0() });
    ({ test_has_trivial_assign_1() });
    ({ test_has_trivial_assign_2() });
    ({ test_has_trivial_assign_3() });
    ({ test_has_trivial_assign_4() });
    ({ test_has_trivial_assign_5() });
    ({ test_has_trivial_assign_6() });
    ({ test_has_trivial_assign_7() });
    ({ test_has_not_trivial_assign_8() });
    ({ test_has_not_trivial_assign_9() });
    ({ test_has_not_trivial_assign_10() });
    ({ test_has_not_trivial_assign_11() });
    ({ test_has_not_trivial_assign_12() });
    return 0_();
}
pub trait AImpl {
    fn copy_assign(&self, _a0: Ptr<A>) -> Ptr<A> {
        unimplemented!()
    }
}
impl AImpl for Ptr<A> {}
pub fn __cpp2rust_init_globals() {}
