extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct ConstructFromInt {
    #[offset(0)]
    pub value: i32,
}
impl ConstructFromInt {
    pub fn new(mut v: i32) -> Self {
        Self { value: v }
    }
}
pub fn test_default_template_arg_0() -> bool {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: ConstructFromInt = unimplemented!();
        assert!((x.value == 10));
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: ConstructFromInt = unimplemented!();
        assert!((x.value == 5));
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: ConstructFromInt = unimplemented!();
        assert!((x.value == 10));
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: ConstructFromInt = unimplemented!();
        assert!((x.value == 5));
    }
    return true;
}
pub fn test_1() -> bool {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: i32 = unimplemented!();
        assert!((x == 10));
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: i32 = unimplemented!();
        assert!((x == 5));
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: i32 = unimplemented!();
        assert!((x == 10));
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut x: i32 = unimplemented!();
        assert!((x == 5));
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    ({ test_default_template_arg_0() });
    const _: () = assert!(true, "test_default_template_arg()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
