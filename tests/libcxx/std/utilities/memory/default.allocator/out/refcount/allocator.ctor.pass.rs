extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> bool {
    let a1: Value<std_allocator_char_> = Rc::new(RefCell::new(<std_allocator_char_>::default()));
    let a1_copy: Value<std_allocator_char_> = Rc::new(RefCell::new((*a1.borrow()).clone()));
    &(*a1_copy.borrow_mut());
    let a2: Value<std_allocator_long_> = Rc::new(RefCell::new(std_allocator_long_::new_1({
        a1.as_pointer()
    })));
    &(*a2.borrow_mut());
    return true;
}
pub fn test_1() -> bool {
    let a1: Value<std_allocator_int_> = Rc::new(RefCell::new(<std_allocator_int_>::default()));
    let a1_copy: Value<std_allocator_int_> = Rc::new(RefCell::new((*a1.borrow()).clone()));
    &(*a1_copy.borrow_mut());
    let a2: Value<std_allocator_long_> = Rc::new(RefCell::new(std_allocator_long_::new_2({
        a1.as_pointer()
    })));
    &(*a2.borrow_mut());
    return true;
}
pub fn test_2() -> bool {
    let a1: Value<std_allocator_void_> = Rc::new(RefCell::new(<std_allocator_void_>::default()));
    let a1_copy: Value<std_allocator_void_> = Rc::new(RefCell::new((*a1.borrow()).clone()));
    &(*a1_copy.borrow_mut());
    let a2: Value<std_allocator_long_> = Rc::new(RefCell::new(std_allocator_long_::new_3({
        a1.as_pointer()
    })));
    &(*a2.borrow_mut());
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
    ({ test_0() });
    ({ test_1() });
    ({ test_2() });
    const _: () = assert!(true, "test<char>()");;
    const _: () = assert!(true, "test<int>()");;
    const _: () = assert!(true, "test<void>()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
