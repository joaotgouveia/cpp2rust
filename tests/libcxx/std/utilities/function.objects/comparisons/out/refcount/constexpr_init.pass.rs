extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_context_0() -> bool {
    let eq: Value<std_equal_to_int_> = Rc::new(RefCell::new(<std_equal_to_int_>::default()));
    (&(*eq.borrow_mut()));
    let neq: Value<std_not_equal_to_int_> =
        Rc::new(RefCell::new(<std_not_equal_to_int_>::default()));
    (&(*neq.borrow_mut()));
    let l: Value<std_less_int_> = Rc::new(RefCell::new(<std_less_int_>::default()));
    (&(*l.borrow_mut()));
    let le: Value<std_less_equal_int_> = Rc::new(RefCell::new(<std_less_equal_int_>::default()));
    (&(*le.borrow_mut()));
    let g: Value<std_greater_int_> = Rc::new(RefCell::new(<std_greater_int_>::default()));
    (&(*g.borrow_mut()));
    let ge: Value<std_greater_equal_int_> =
        Rc::new(RefCell::new(<std_greater_equal_int_>::default()));
    (&(*ge.borrow_mut()));
    return true;
}
pub fn test_constexpr_context_1() -> bool {
    let eq: Value<std_equal_to_void_> = Rc::new(RefCell::new(<std_equal_to_void_>::default()));
    (&(*eq.borrow_mut()));
    let neq: Value<std_not_equal_to_void_> =
        Rc::new(RefCell::new(<std_not_equal_to_void_>::default()));
    (&(*neq.borrow_mut()));
    let l: Value<std_less_void_> = Rc::new(RefCell::new(<std_less_void_>::default()));
    (&(*l.borrow_mut()));
    let le: Value<std_less_equal_void_> = Rc::new(RefCell::new(<std_less_equal_void_>::default()));
    (&(*le.borrow_mut()));
    let g: Value<std_greater_void_> = Rc::new(RefCell::new(<std_greater_void_>::default()));
    (&(*g.borrow_mut()));
    let ge: Value<std_greater_equal_void_> =
        Rc::new(RefCell::new(<std_greater_equal_void_>::default()));
    (&(*ge.borrow_mut()));
    return true;
}
const _: () = assert!(true, "test_constexpr_context<int>()");
const _: () = assert!(true, "test_constexpr_context<void>()");
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
    return 0;
}
pub fn __cpp2rust_init_globals() {}
