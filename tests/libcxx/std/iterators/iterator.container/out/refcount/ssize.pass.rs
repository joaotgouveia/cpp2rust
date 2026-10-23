extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long__long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct short_container {}
pub fn test_container_1(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_container_2(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_container_3(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_container_4(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_const_container_5(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_const_container_6(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_const_container_7(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_const_container_8(c: Ptr<()>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_const_container_9(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_container_10(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    unimplemented!();
}
pub fn test_const_array_11(array: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(array))>");;
    unimplemented!();
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
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(3_());
    let il: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_container_1(v.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_container_2(l.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_container_3(a.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_container_10(il.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_const_container_5(v.as_pointer()) });
    ({ test_const_container_6(l.as_pointer()) });
    ({ test_const_container_7(a.as_pointer()) });
    ({ test_const_container_9(il.as_pointer()) });
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_container_4(sv.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_const_container_8(sv.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(arrA))>");;
    ({ test_const_array_11(arrA_12.with(|v| v.as_pointer())) });
    let mut sc: short_container = <short_container>::default();
    const _: () = assert!(
        true,
        "std::is_signed_v<                      decltype(std::ssize(sc))>"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<                   decltype(std::ssize(sc))>::max()  > 60000"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<std::make_signed_t<decltype(std:: size(sc))>>::max() < 60000"
    );;
    unimplemented!();
    const _: () = assert!(true, "");;
    return 0_();
}
thread_local!(
    static arrA_12: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
);
pub trait short_containerImpl {
    fn size(&self) -> u16;
}
impl short_containerImpl for Ptr<short_container> {
    fn size(&self) -> u16 {
        return 60000_u16;
    }
}
pub fn __cpp2rust_init_globals() {}
