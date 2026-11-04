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
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
impl CppMove for test_macros_detail_is_same_int_ref__int_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
impl CppMove for test_macros_detail_is_same_const_int_ref__const_int_ref_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
impl CppMove for test_macros_detail_is_same_int_ptr__int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
impl CppMove for test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        let v: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a.as_pointer() as Ptr<i32>) },
            { 5_usize },
        )));
        let cv: Ptr<std_valarray_int_> = v.as_pointer();
        const _: () = assert!(
            true,
            "std::is_base_of<std::random_access_iterator_tag, std::iterator_traits<It>::iterator_category>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_base_of<std::random_access_iterator_tag, std::iterator_traits<CIt>::iterator_category>::value"
        );;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!((({ begin_4(v.as_pointer(),) }) == (v.as_pointer().offset(0_usize))));
        assert!(({ ({ begin_5((cv).clone(),) }) } == { (cv.offset(0_usize)) }));
        ({ begin_4(v.as_pointer()) }).write(10);
        assert!(({ ({ (*v.borrow())[0_usize] } == 10) }));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(
            ((libcc2rs::prev_refcount(
                ({ libcc2rs::end_refcount(v.as_pointer(),) }),
                1_isize as isize
            )) == (v.as_pointer().offset(4_usize)))
        );
        assert!(
            ({
                (libcc2rs::prev_refcount(
                    ({ libcc2rs::end_refcount((cv).clone()) }),
                    1_isize as isize,
                ))
            } == { (cv.offset(4_usize)) })
        );
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        let v: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a.as_pointer() as Ptr<i32>) },
            { 5_usize },
        )));
        let mut sum: i32 = 0;
        'loop_: for i in IterRange::new(
            ({ begin_4(v.as_pointer()) }),
            ({ libcc2rs::end_refcount(v.as_pointer()) }),
        ) {
            let i: Ptr<i32> = (i).clone();
            sum += { (i.read()) };
        }
        assert!((sum == 15));
    }
    {
        let a: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        let cv: Value<std_valarray_int_> = Rc::new(RefCell::new(std_valarray_int_::new_1(
            { (a.as_pointer() as Ptr<i32>) },
            { 5_usize },
        )));
        let mut sum: i32 = 0;
        'loop_: for i in IterRange::new(
            ({ begin_5(cv.as_pointer()) }),
            ({ libcc2rs::end_refcount(cv.as_pointer()) }),
        ) {
            let i: Ptr<i32> = (i).clone();
            sum += { (i.read()) };
        }
        assert!((sum == 15));
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {}
