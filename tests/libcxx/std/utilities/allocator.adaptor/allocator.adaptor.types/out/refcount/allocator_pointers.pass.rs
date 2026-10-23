extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_pointer_0() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub fn test_pointer_1() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub fn test_pointer_2() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub fn test_void_pointer_3() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub fn test_void_pointer_4() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub fn test_void_pointer_5() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Foo {
    #[offset(0)]
    pub x: (),
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
    ({ test_pointer_0() });
    ({ test_pointer_1() });
    ({ test_pointer_2() });
    ({ test_void_pointer_3() });
    ({ test_void_pointer_4() });
    ({ test_void_pointer_5() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
