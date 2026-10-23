extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_aggregate_0() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_1() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_2() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_3() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_4() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_5() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_6() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_7() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
pub fn check_aggregate_8() {
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 0> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 1> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 2> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 3> >::value");;
    const _: () = assert!(true, "std::is_aggregate<std::array<T, 4> >::value");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Trivial {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub j: (),
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(8)]
pub struct NonTrivial {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    pub j: (),
}
impl NonTrivial {
    pub fn copy_from(_a0: Ptr<NonTrivial>) -> Self {
        Self {
            i: unimplemented!(),
            j: unimplemented!(),
        }
    }
}
impl Clone for NonTrivial {
    fn clone(&self) -> Self {
        let __src: Value<NonTrivial> = Rc::new(RefCell::new(NonTrivial {
            i: self.i.clone(),
            j: self.j.clone(),
        }));
        NonTrivial::copy_from(__src.as_pointer())
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
    ({ check_aggregate_0() });
    ({ check_aggregate_1() });
    ({ check_aggregate_2() });
    ({ check_aggregate_3() });
    ({ check_aggregate_4() });
    ({ check_aggregate_5() });
    ({ check_aggregate_6() });
    ({ check_aggregate_7() });
    ({ check_aggregate_8() });
    return 0_();
}
pub trait NonTrivialImpl {
    fn copy_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial>;
}
impl NonTrivialImpl for Ptr<NonTrivial> {
    fn copy_assign(&self, _a0: Ptr<NonTrivial>) -> Ptr<NonTrivial> {
        field!((*self), i).write({ _a0.with(|__s| __s.i) });
        field!((*self), j).write({ _a0.with(|__s| __s.j) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
