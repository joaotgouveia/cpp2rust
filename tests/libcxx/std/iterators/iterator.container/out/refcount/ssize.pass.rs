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
pub struct test_macros_detail_is_same_long__long_ {}
impl CppMove for test_macros_detail_is_same_long__long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct short_container {}
impl CppMove for short_container {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_container_1(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(({ ({ ({ ssize_2((c).clone(),) }) } == { ((*c.upgrade().deref()).len() as i64) }) }));
}
pub fn test_container_3(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(({ ({ ({ ssize_4((c).clone(),) }) } == { ((*c.upgrade().deref()).len() as i64) }) }));
}
pub fn test_const_container_5(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(({ ({ ({ ssize_2((c).clone(),) }) } == { ((*c.upgrade().deref()).len() as i64) }) }));
}
pub fn test_const_container_6(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(({ ({ ({ ssize_4((c).clone(),) }) } == { ((*c.upgrade().deref()).len() as i64) }) }));
}
pub fn test_const_container_7(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(({ ({ ({ ssize_8((c).clone(),) }) } == { ((*c.upgrade().deref()).len() as i64) }) }));
}
pub fn test_container_9(c: Ptr<Vec<i32>>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(c))>");;
    assert!(({ ({ ({ ssize_8((c).clone(),) }) } == { ((*c.upgrade().deref()).len() as i64) }) }));
}
pub fn test_const_array_10(array: Ptr<i32>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(array))>");;
    assert!(((({ ssize_11((array).clone(),) }) as u64) == 3_u64));
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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    let l: Value<std_list_int__std_allocator_int__> =
        Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(2));
        (*l.borrow()).push_back_52(___x.as_pointer())
    });
    let a: Value<Vec<i32>> = Rc::new(RefCell::new(
        std::array::from_fn::<_, 1, _>(|_| 0_i32).to_vec(),
    ));
    a.decay().offset(0_usize as isize).write(3);
    let il: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([4])));
    ({ test_container_1(v.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_container_12(l.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_container_3(a.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_container_9(il.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_const_container_5(v.as_pointer()) });
    ({ test_const_container_13(l.as_pointer()) });
    ({ test_const_container_6(a.as_pointer()) });
    ({ test_const_container_7(il.as_pointer()) });
    let sv: Value<std_basic_string_view_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_string_view_char__std_char_traits_char__::new_1({
            Ptr::<i8>::from_string_literal(b"ABC")
        }),
    ));
    ({ test_container_14(sv.as_pointer()) });
    const _: () = assert!(true, "");;
    ({ test_const_container_15(sv.as_pointer()) });
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_signed_v<decltype(std::ssize(arrA))>");;
    ({ test_const_array_10(arrA_16.with(|v| v.as_pointer())) });
    let sc: Value<short_container> = Rc::new(RefCell::new(<short_container>::default()));
    const _: () = assert!(
        true,
        "std::is_signed_v<                      decltype(std::ssize(sc))>"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<                   decltype(std::ssize(sc))>::max()  > 60000"
    );;
    const _: () = assert!(
        true,
        "std::numeric_limits<std::make_signed_t<decltype(std:: size(sc))>>::max() < 60000"
    );;
    assert!((({ ssize_17(sc.as_pointer(),) }) == 60000_i64));
    const _: () = assert!(true, "");;
    return 0;
}
thread_local!(
    static arrA_16: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
);
pub trait short_containerImpl {
    fn size(&self) -> u16;
}
impl short_containerImpl for Ptr<short_container> {
    fn size(&self) -> u16 {
        return 60000_u16;
    }
}
pub fn __cpp2rust_init_globals() {}
