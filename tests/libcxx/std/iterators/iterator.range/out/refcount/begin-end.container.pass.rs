extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_int_ptr___std_reverse_iterator_int_ptr__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_const_int_ptr___std_reverse_iterator_const_int_ptr__
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std__List_iterator_int___std__List_iterator_int__ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std__List_const_iterator_int___std__List_const_iterator_int__
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_std__List_iterator_int____std_reverse_iterator_std__List_iterator_int___
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_std__List_const_iterator_int____std_reverse_iterator_std__List_const_iterator_int___
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int____
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____
{}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int______std_reverse_iterator___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int_____
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int______std_reverse_iterator___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_____
{}
pub fn test_12() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cc: Ptr<()> = c.as_pointer();
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "noexcept(std::cbegin(cc)) == noexcept(std::begin(cc))"
        );;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(cc)) == noexcept(std::end(cc))");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cbegin(c)) == noexcept(std::begin(cc))");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(c)) == noexcept(std::end(cc))");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    return true;
}
pub fn test_13() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cc: Ptr<()> = c.as_pointer();
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "noexcept(std::cbegin(cc)) == noexcept(std::begin(cc))"
        );;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(cc)) == noexcept(std::end(cc))");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cbegin(c)) == noexcept(std::begin(cc))");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(c)) == noexcept(std::end(cc))");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    return true;
}
pub fn test_14() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let cc: Ptr<()> = c.as_pointer();
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "noexcept(std::cbegin(cc)) == noexcept(std::begin(cc))"
        );;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(cc)) == noexcept(std::end(cc))");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cbegin(c)) == noexcept(std::begin(cc))");;
        unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(std::cend(c)) == noexcept(std::end(cc))");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
        const _: () = assert!(true, "");;
        unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_12() });
    ({ test_13() });
    ({ test_14() });
    const _: () = assert!(true, "test<std::array<int, 3>>()");;
    {
        let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
        let ca: Ptr<()> = a.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut b: Ptr<()> = unimplemented!();
        unimplemented!();
        let mut e: Ptr<()> = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
thread_local!(
    static a_15: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
);
pub fn __cpp2rust_init_globals() {}
