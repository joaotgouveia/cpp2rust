extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn test_0() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
}
pub fn test_1() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
}
pub fn test_2() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
}
pub fn test_3() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    return 0_();
}
pub trait MoveOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
}
pub fn __cpp2rust_init_globals() {}
