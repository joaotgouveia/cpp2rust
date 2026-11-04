extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct functor1 {}
impl CppMove for functor1 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct convertible_to_int_ref {
    #[offset(0)]
    pub val: i32,
}
impl CppMove for convertible_to_int_ref {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for convertible_to_int_ref {
    fn default() -> Self {
        { convertible_to_int_ref { val: 0 } }
    }
}
pub fn test_0(t: Ptr<FnPtr<fn()>>) {
    let r: Value<std_reference_wrapper_void__ptr____> =
        Rc::new(RefCell::new(std_reference_wrapper_void__ptr____::new({
            (t).clone()
        })));
    let t2: Value<FnPtr<fn()>> = Rc::new(RefCell::new((t.read())));
    let r2: Value<std_reference_wrapper_void__ptr____> =
        Rc::new(RefCell::new(std_reference_wrapper_void__ptr____::new({
            t2.as_pointer()
        })));
    (*r2.borrow_mut()) = (*r.borrow()).clone();
    ({ cpp2rust_assert_fail_1(({ ({ (*r2.borrow()).get() }) } == { (t).clone() })) });
}
pub fn test_2(t: Ptr<functor1>) {
    let r: Value<std_reference_wrapper_functor1_> =
        Rc::new(RefCell::new(std_reference_wrapper_functor1_::new({
            (t).clone()
        })));
    let t2: Value<functor1> = Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
    let r2: Value<std_reference_wrapper_functor1_> =
        Rc::new(RefCell::new(std_reference_wrapper_functor1_::new({
            t2.as_pointer()
        })));
    (*r2.borrow_mut()) = (*r.borrow()).clone();
    ({ cpp2rust_assert_fail_1(({ ({ (*r2.borrow()).get() }) } == { (t).clone() })) });
}
pub fn test_3(t: Ptr<i32>) {
    let r: Value<std_reference_wrapper_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_int_::new({
            (t).clone()
        })));
    let t2: Value<i32> = Rc::new(RefCell::new((t.read())));
    let r2: Value<std_reference_wrapper_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_int_::new({
            t2.as_pointer()
        })));
    (*r2.borrow_mut()) = (*r.borrow()).clone();
    ({ cpp2rust_assert_fail_1(({ ({ (*r2.borrow()).get() }) } == { (t).clone() })) });
}
pub fn test_4(t: Ptr<i32>) {
    let r: Value<std_reference_wrapper_const_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_const_int_::new({
            (t).clone()
        })));
    let t2: Value<i32> = Rc::new(RefCell::new((t.read())));
    let r2: Value<std_reference_wrapper_const_int_> =
        Rc::new(RefCell::new(std_reference_wrapper_const_int_::new({
            t2.as_pointer()
        })));
    (*r2.borrow_mut()) = (*r.borrow()).clone();
    ({ cpp2rust_assert_fail_1(({ ({ (*r2.borrow()).get() }) } == { (t).clone() })) });
}
pub fn test_5(t: Ptr<convertible_to_int_ref>) {
    let r: Value<std_reference_wrapper_convertible_to_int_ref_> = Rc::new(RefCell::new(
        std_reference_wrapper_convertible_to_int_ref_::new({ (t).clone() }),
    ));
    let t2: Value<convertible_to_int_ref> = Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
    let r2: Value<std_reference_wrapper_convertible_to_int_ref_> = Rc::new(RefCell::new(
        std_reference_wrapper_convertible_to_int_ref_::new({ t2.as_pointer() }),
    ));
    (*r2.borrow_mut()) = (*r.borrow()).clone();
    ({ cpp2rust_assert_fail_1(({ ({ (*r2.borrow()).get() }) } == { (t).clone() })) });
}
pub fn test_6(t: Ptr<convertible_to_int_ref>) {
    let r: Value<std_reference_wrapper_const_convertible_to_int_ref_> = Rc::new(RefCell::new(
        std_reference_wrapper_const_convertible_to_int_ref_::new({ (t).clone() }),
    ));
    let t2: Value<convertible_to_int_ref> = Rc::new(RefCell::new((*t.upgrade().deref()).clone()));
    let r2: Value<std_reference_wrapper_const_convertible_to_int_ref_> = Rc::new(RefCell::new(
        std_reference_wrapper_const_convertible_to_int_ref_::new({ t2.as_pointer() }),
    ));
    (*r2.borrow_mut()) = (*r.borrow()).clone();
    ({ cpp2rust_assert_fail_1(({ ({ (*r2.borrow()).get() }) } == { (t).clone() })) });
}
pub fn f_7() {}
pub fn g_8() {}
pub fn test_function_9() {
    let r: Value<std_reference_wrapper_void____> =
        Rc::new(RefCell::new(std_reference_wrapper_void____::new({
            FnPtr::<fn()>::new(f_7)
        })));
    let r2: Value<std_reference_wrapper_void____> =
        Rc::new(RefCell::new(std_reference_wrapper_void____::new({
            FnPtr::<fn()>::new(g_8)
        })));
    (*r2.borrow_mut()) = (*r.borrow()).clone();
    ({ cpp2rust_assert_fail_1((({ (*r2.borrow()).get() }) == (FnPtr::<fn()>::new(f_7)))) });
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
    let fp: Value<FnPtr<fn()>> = Rc::new(RefCell::new(FnPtr::<fn()>::new(f_7)));
    ({ test_0(fp.as_pointer()) });
    ({ test_function_9() });
    let f1: Value<functor1> = Rc::new(RefCell::new(<functor1>::default()));
    ({ test_2(f1.as_pointer()) });
    let i: Value<i32> = Rc::new(RefCell::new(0));
    ({ test_3(i.as_pointer()) });
    let j: Value<i32> = Rc::new(RefCell::new(0));
    ({ test_4(j.as_pointer()) });
    let convi: Value<convertible_to_int_ref> =
        Rc::new(RefCell::new(<convertible_to_int_ref>::default()));
    ({ test_5(convi.as_pointer()) });
    let convic: Value<convertible_to_int_ref> =
        Rc::new(RefCell::new(<convertible_to_int_ref>::default()));
    ({ test_6(convic.as_pointer()) });
    {
        const _: () = assert!(true, "(std::is_assignable<Ref&, int&>::value)");;
        const _: () = assert!(true, "(!std::is_assignable<Ref&, int>::value)");;
        const _: () = assert!(true, "(!std::is_assignable<Ref&, int&&>::value)");;
    }
    return 0;
}
pub trait convertible_to_int_refImpl {
    fn to_pmuti32(&self) -> Ptr<i32>;
    fn to_pconsti32(&self) -> Ptr<i32>;
}
impl convertible_to_int_refImpl for Ptr<convertible_to_int_ref> {
    fn to_pmuti32(&self) -> Ptr<i32> {
        return field_ptr!((*self), val);
    }
    fn to_pconsti32(&self) -> Ptr<i32> {
        return field_ptr!((*self), val);
    }
}
pub fn __cpp2rust_init_globals() {}
