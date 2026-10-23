extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn swap_0(a: Ptr<S>, b: Ptr<S>) {
    {
        a.with(|__s| __s.calls.clone())
            .with_mut(|__v| *__v = *__v + 1_())
    };
    {
        b.with(|__s| __s.calls.clone())
            .with_mut(|__v| *__v = *__v + 1_())
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub calls: Ptr<()>,
}
pub fn swap_1(a: Ptr<CS>, b: Ptr<CS>) {
    {
        a.with(|__s| __s.calls.clone())
            .with_mut(|__v| *__v = *__v + 1_())
    };
    {
        b.with(|__s| __s.calls.clone())
            .with_mut(|__v| *__v = *__v + 1_())
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CS {
    #[offset(0)]
    #[byte_size(8)]
    pub calls: Ptr<()>,
}
const _: () = assert!(true, "std::is_swappable_v<std::tuple<>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<S>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<CS>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<S&>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<CS, S>>");
const _: () = assert!(true, "std::is_swappable_v<std::tuple<CS, S&>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<>>");
const _: () = assert!(true, "!std::is_swappable_v<const std::tuple<S>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<CS>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<S&>>");
const _: () = assert!(true, "!std::is_swappable_v<const std::tuple<CS, S>>");
const _: () = assert!(true, "std::is_swappable_v<const std::tuple<CS, S&>>");
pub fn test_2() -> () {
    let cs_calls: Value<()> = Rc::new(RefCell::new(0_()));
    let s_calls: Value<()> = Rc::new(RefCell::new(0_()));
    let s1: Value<S> = Rc::new(RefCell::new(S {
        calls: (s_calls.as_pointer()),
    }));
    let s2: Value<S> = Rc::new(RefCell::new(S {
        calls: (s_calls.as_pointer()),
    }));
    let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let t2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ swap_3(t1.as_pointer(), t2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return true;
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
