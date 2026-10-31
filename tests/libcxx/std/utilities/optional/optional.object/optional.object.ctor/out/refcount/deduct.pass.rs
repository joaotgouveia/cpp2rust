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
pub struct test_macros_detail_is_same_std_optional_int___std_optional_int__ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_A___std_optional_A__ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_const_int_ptr___std_optional_const_int_ptr__ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_int_ptr___std_optional_int_ptr__ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_optional_char___std_optional_char__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct A {}
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
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(((unimplemented!().read()) == 5));
    }
    {
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
        let source: Ptr<i32> = __tmp_0.as_pointer();
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(((unimplemented!().read()) == 5));
    }
    {
        let mut source: Ptr<i32> = Ptr::<i32>::null();
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!((unimplemented!().read()).is_null());
    }
    {
        let mut source: [i32; 3] = [1, 2, 3];
        let opt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(((elem!((unimplemented!().read()), 0).read()) == 1));
    }
    {
        let source: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let opt: Value<()> = Rc::new(RefCell::new((*source.borrow()).clone()));
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        assert!((((unimplemented!().read()) as i32) == ((unimplemented!().read()) as i32)));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
