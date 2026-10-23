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
        let ignore_v: Ptr<std___ignore_type> = ignore_2.with(|v| v.as_pointer());
    }
    {
        let res: Ptr<std___ignore_type> = (ignore_2.with(|rc| *rc.borrow_mut() = 42_()));
        const _: () = assert!(true, "noexcept(res = (std::ignore = 42))");;
        unimplemented!();
    }
    {
        let mut s: S_3 = S_3 { bf: 2_() };
        let res: Ptr<std___ignore_type> = (ignore_2.with(|rc| *rc.borrow_mut() = s.bf));
        unimplemented!();
    }
    {
        let copy: Value<std___ignore_type> = Rc::new(RefCell::new(
            (*ignore_2.with(Value::clone).borrow()).clone(),
        ));
        let moved: Value<std___ignore_type> = Rc::new(RefCell::new((*copy.borrow_mut()).clone()));
    }
    {
        let copy: Value<std___ignore_type> = Rc::new(RefCell::new(
            (*ignore_2.with(Value::clone).borrow()).clone(),
        ));
        (*copy.borrow_mut()) = (*ignore_2.with(Value::clone).borrow()).clone();
        let moved: Value<std___ignore_type> = Rc::new(RefCell::new(
            (*ignore_2.with(Value::clone).borrow()).clone(),
        ));
        (*moved.borrow_mut()) = (*copy.borrow()).clone();
    }
    {
        ignore_2.with(|rc| *rc.borrow_mut() = ({ test_nodiscard_0() }));
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_3 {
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
