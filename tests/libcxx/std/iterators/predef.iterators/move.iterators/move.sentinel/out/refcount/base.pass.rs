extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int__int_ {}
impl CppMove for test_macros_detail_is_same_int__int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_2() -> bool {
    {
        let m: Value<std_move_sentinel_int_> =
            Rc::new(RefCell::new(std_move_sentinel_int_::new_1({ 42 })));
        let cm: Ptr<std_move_sentinel_int_> = m.as_pointer();
        assert!(({ (({ (*m.borrow()).base() }) == 42) }));
        assert!(({ (({ (*cm.upgrade().deref()).base() }) == 42) }));
        assert!(({ (({ (*m.borrow()).base() }) == 42) }));
        assert!(({ (({ (*cm.upgrade().deref()).base() }) == 42) }));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
        let m: Value<std_move_sentinel_const_int_ptr_> =
            Rc::new(RefCell::new(std_move_sentinel_const_int_ptr_::new_1({
                (a.as_pointer() as Ptr<i32>)
            })));
        let cm: Ptr<std_move_sentinel_const_int_ptr_> = m.as_pointer();
        assert!(({ (({ (*m.borrow()).base() }) == (a.as_pointer() as Ptr::<i32>)) }));
        assert!(
            ({ ({ ({ (*cm.upgrade().deref()).base() }) } == { (a.as_pointer() as Ptr::<i32>) }) })
        );
        assert!(({ (({ (*m.borrow()).base() }) == (a.as_pointer() as Ptr::<i32>)) }));
        assert!(
            ({ ({ ({ (*cm.upgrade().deref()).base() }) } == { (a.as_pointer() as Ptr::<i32>) }) })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
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
    ({ test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
