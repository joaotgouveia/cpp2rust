extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_container_0(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_container_1(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_container_2(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_container_3(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_const_container_4(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_const_container_5(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_const_container_6(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_const_container_7(c: Ptr<()>) {
    unimplemented!();
}
pub fn test_const_container_8(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_container_9(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub fn test_const_array_10(array: Ptr<()>) {
    const _: () = assert!(true, "");;
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
    ({ test_container_0(v.as_pointer()) });
    ({ test_container_1(l.as_pointer()) });
    ({ test_container_2(a.as_pointer()) });
    ({ test_container_9(il.as_pointer()) });
    ({ test_const_container_4(v.as_pointer()) });
    ({ test_const_container_5(l.as_pointer()) });
    ({ test_const_container_6(a.as_pointer()) });
    ({ test_const_container_8(il.as_pointer()) });
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_container_3(sv.as_pointer()) });
    ({ test_const_container_7(sv.as_pointer()) });
    ({ test_const_array_10(arrA_11.with(|v| v.as_pointer())) });
    return 0_();
}
thread_local!(
    static arrA_11: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
);
pub fn __cpp2rust_init_globals() {}
