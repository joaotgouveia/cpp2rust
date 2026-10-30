extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(
    true,
    "std::is_trivially_copyable<decltype(std::ignore)>::value"
);
const _: () = assert!(
    true,
    "std::is_trivially_default_constructible<decltype(std::ignore)>::value"
);
pub fn test_nodiscard_0() -> () {
    return 8294_();
}
pub fn test_1() -> () {
    {
        let ignore_v: Ptr<()> = unimplemented!();
    }
    {
        let res: Ptr<()> = (unimplemented!());
        const _: () = assert!(true, "noexcept(res = (std::ignore = 42))");;
        unimplemented!();
    }
    {
        let mut s: S_2 = S_2 { bf: 2_() };
        let res: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
    {
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let moved: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    }
    {
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let moved: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_2 {
    #[offset(0)]
    pub bf: (),
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
