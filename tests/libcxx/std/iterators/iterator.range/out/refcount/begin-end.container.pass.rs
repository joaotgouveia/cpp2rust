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
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
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
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std__List_iterator_int___std__List_iterator_int__ {}
impl CppMove for test_macros_detail_is_same_std__List_iterator_int___std__List_iterator_int__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std__List_const_iterator_int___std__List_const_iterator_int__
{}
impl CppMove
    for test_macros_detail_is_same_std__List_const_iterator_int___std__List_const_iterator_int__
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_std__List_iterator_int____std_reverse_iterator_std__List_iterator_int___
{}
impl CppMove for test_macros_detail_is_same_std_reverse_iterator_std__List_iterator_int____std_reverse_iterator_std__List_iterator_int___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_reverse_iterator_std__List_const_iterator_int____std_reverse_iterator_std__List_const_iterator_int___
{}
impl CppMove for test_macros_detail_is_same_std_reverse_iterator_std__List_const_iterator_int____std_reverse_iterator_std__List_const_iterator_int___ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_6 = u32;
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int____
{}
impl CppMove for test_macros_detail_is_same___gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int____ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
pub type anon_7 = u32;
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____
{}
impl CppMove for test_macros_detail_is_same___gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int_______gnu_cxx___normal_iterator_const_int_ptr__std_vector_int__std_allocator_int____ { fn cpp_move(src: Ptr<Self>) -> Self { src.read() } }
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
    ({ test_8() });
    ({ test_9() });
    ({ test_10() });
    const _: () = assert!(true, "test<std::array<int, 3>>()");;
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
        let ca: Ptr<i32> = a.as_pointer();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut b: Ptr<i32> = ({ cbegin_12(a_11.with(|v| v.as_pointer())) });
        assert!(({ (b).clone() } == { (a_11.with(|v| v.as_pointer()) as Ptr::<i32>) }));
        let mut e: Ptr<i32> = ({ cend_13(a_11.with(|v| v.as_pointer())) });
        assert!(
            ({ (e).clone() } == {
                (a_11.with(|v| v.as_pointer()) as Ptr<i32>).offset((3) as isize)
            })
        );
    }
    return 0;
}
thread_local!(
    static a_11: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
);
pub fn __cpp2rust_init_globals() {}
