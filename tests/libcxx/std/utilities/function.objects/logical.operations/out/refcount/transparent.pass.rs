extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static value_0: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_logical_and_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_and_int__ {}
thread_local!(
    pub static value_1: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_logical_and_std_string___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_and_std_string__ {}
thread_local!(
    pub static value_2: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_and_void__ {}
thread_local!(
    pub static value_3: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_logical_or_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_or_int__ {}
thread_local!(
    pub static value_4: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_logical_or_std_string___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_or_std_string__ {}
thread_local!(
    pub static value_5: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_or_void__ {}
thread_local!(
    pub static value_6: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_logical_not_int___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_not_int__ {}
thread_local!(
    pub static value_7: Value<()> = Rc::new(RefCell::new((2usize == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
struct is_transparent_std_logical_not_std_string___two {
    #[offset(0)]
    pub lx: (),
    #[offset(1)]
    pub lxx: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_not_std_string__ {}
thread_local!(
    pub static value_8: Value<()> = Rc::new(RefCell::new((::std::mem::size_of::<()>() == 1_())));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct is_transparent_std_logical_not_void__ {}
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
    const _: () = assert!(true, "!is_transparent<std::logical_and<int>>::value");;
    const _: () = assert!(
        true,
        "!is_transparent<std::logical_and<std::string>>::value"
    );;
    const _: () = assert!(true, "is_transparent<std::logical_and<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::logical_and<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::logical_or<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::logical_or<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::logical_or<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::logical_or<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::logical_not<int>>::value");;
    const _: () = assert!(
        true,
        "!is_transparent<std::logical_not<std::string>>::value"
    );;
    const _: () = assert!(true, "is_transparent<std::logical_not<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::logical_not<>>::value");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = value_0.with(|_| ());
    let _ = value_1.with(|_| ());
    let _ = value_2.with(|_| ());
    let _ = value_3.with(|_| ());
    let _ = value_4.with(|_| ());
    let _ = value_5.with(|_| ());
    let _ = value_6.with(|_| ());
    let _ = value_7.with(|_| ());
    let _ = value_8.with(|_| ());
}
