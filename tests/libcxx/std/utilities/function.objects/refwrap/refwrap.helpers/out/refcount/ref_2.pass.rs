extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(FnPtr<fn(i32  ,) -> bool  >  , new, value)]
#[byte_size(16)]
pub struct unary_counting_predicate_bool__ptr__int___int_ {
    #[offset(0)]
    #[byte_size(8)]
    p_: FnPtr<fn(i32) -> bool>,
    #[offset(8)]
    count_: u64,
}
impl unary_counting_predicate_bool__ptr__int___int_ {
    pub fn new(mut p: FnPtr<fn(i32) -> bool>) -> Self {
        Self {
            p_: (p).clone(),
            count_: 0_u64,
        }
    }
}
impl Callable1<Ptr<i32>, bool> for unary_counting_predicate_bool__ptr__int___int_ {
    fn call(&self, a1: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            unary_counting_predicate_bool__ptr__int___int_Impl::operator_call(__it, a1)
        })
    }
}
impl CppMove for unary_counting_predicate_bool__ptr__int___int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn is5_0(mut i: i32) -> bool {
    return (i == 5);
}
pub fn call_pred_1(pred: unary_counting_predicate_bool__ptr__int___int_) -> bool {
    let pred: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(pred));
    let _dtor_pred = ScopedDestructor::new(&pred, |__p| __p.destructor());
    return ({
        let _a: Value<i32> = Rc::new(RefCell::new(5));
        unary_counting_predicate_bool__ptr__int___int_Impl::operator_call(
            &pred.as_pointer(),
            _a.as_pointer(),
        )
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct adl_A {}
impl CppMove for adl_A {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn ref_2(mut _a0: adl_A) {}
pub fn test_3() -> bool {
    {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        let r1: Value<std_reference_wrapper_int_> =
            Rc::new(RefCell::new(({ ref_4(i.as_pointer()) })));
        let r2: Value<std_reference_wrapper_int_> =
            Rc::new(RefCell::new(({ ref_5((*r1.borrow()).clone()) })));
        ({ cpp2rust_assert_fail_6((({ (*r2.borrow()).get() }) == (i.as_pointer()))) });
    }
    {
        let a: Value<adl_A> = Rc::new(RefCell::new(<adl_A>::default()));
        let a1: Value<std_reference_wrapper_adl_A_> =
            Rc::new(RefCell::new(({ ref_7(a.as_pointer()) })));
        let a2: Value<std_reference_wrapper_adl_A_> =
            Rc::new(RefCell::new(({ ref_8((*a1.borrow()).clone()) })));
        ({ cpp2rust_assert_fail_6((({ (*a2.borrow()).get() }) == (a.as_pointer()))) });
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
    ({ test_3() });
    const _: () = assert!(true, "test()");;
    {
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(i32) -> bool>::new(is5_0)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({
            cpp2rust_assert_fail_6(
                !({
                    let _a: Value<i32> = Rc::new(RefCell::new(6));
                    unary_counting_predicate_bool__ptr__int___int_Impl::operator_call(
                        &cp.as_pointer(),
                        _a.as_pointer(),
                    )
                }),
            )
        });
        ({
            cpp2rust_assert_fail_6(
                (({ unary_counting_predicate_bool__ptr__int___int_Impl::count(&cp.as_pointer()) })
                    == 1_u64),
            )
        });
        ({ cpp2rust_assert_fail_6(({ call_pred_1((*cp.borrow()).clone()) })) });
        ({
            cpp2rust_assert_fail_6(
                (({ unary_counting_predicate_bool__ptr__int___int_Impl::count(&cp.as_pointer()) })
                    == 1_u64),
            )
        });
        ({ cpp2rust_assert_fail_6(({ call_pred_9(({ ref_10(cp.as_pointer()) })) })) });
        ({
            cpp2rust_assert_fail_6(
                (({ unary_counting_predicate_bool__ptr__int___int_Impl::count(&cp.as_pointer()) })
                    == 2_u64),
            )
        });
    }
    return 0;
}
pub trait unary_counting_predicate_bool__ptr__int___int_Impl {
    fn destructor(&self);
    fn operator_call(&self, a: Ptr<i32>) -> bool;
    fn count(&self) -> u64;
    fn reset(&self) {
        unimplemented!()
    }
}
impl unary_counting_predicate_bool__ptr__int___int_Impl
    for Ptr<unary_counting_predicate_bool__ptr__int___int_>
{
    fn destructor(&self) {}
    fn operator_call(&self, a: Ptr<i32>) -> bool {
        field!((*self), count_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return ({ (*self).with(|__s| __s.p_.clone()).call((a.read())) });
    }
    fn count(&self) -> u64 {
        return (*self).with(|__s| __s.count_);
    }
}
pub fn __cpp2rust_init_globals() {}
