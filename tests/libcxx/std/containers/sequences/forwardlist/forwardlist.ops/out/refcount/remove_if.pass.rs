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
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_4: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
pub fn operator_eq_5(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_6(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_void__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_5(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_5(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(16)]
pub struct unary_counting_predicate_bool__ptr__int___int_ {
    #[offset(0)]
    #[byte_size(8)]
    p_: FnPtr<fn(()) -> ()>,
    #[offset(8)]
    count_: (),
}
impl unary_counting_predicate_bool__ptr__int___int_ {
    pub fn new(mut p: FnPtr<fn(()) -> ()>) -> Self {
        Self {
            p_: (p).clone(),
            count_: 0_(),
        }
    }
}
pub fn do_remove_if_7(l: Ptr<()>, pred: (), mut expected: ()) {
    let pred: Value<()> = Rc::new(RefCell::new(pred));
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn do_remove_if_8(l: Ptr<()>, pred: (), mut expected: ()) {
    let pred: Value<()> = Rc::new(RefCell::new(pred));
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn do_remove_if_9(l: Ptr<()>, pred: (), mut expected: ()) {
    let pred: Value<()> = Rc::new(RefCell::new(pred));
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub fn g_10(mut i: ()) -> () {
    return (i < 3_());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct PredLWG526 {
    #[offset(0)]
    pub i_: (),
}
impl PredLWG526 {
    pub fn new(mut i: ()) -> Self {
        Self { i_: i }
    }
}
pub fn test_11() -> () {
    {
        let mut t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_7(c1.as_pointer(), unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_7(c1.as_pointer(), unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: [(); 3] = [5_(), 5_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_7(c1.as_pointer(), unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_7(c1.as_pointer(), unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_7(c1.as_pointer(), unimplemented!(), 1_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            1_(),
            2_(),
            1_(),
            3_(),
            5_(),
            8_(),
            11_(),
        ])));
        let mut a2: [(); 5] = [2_(), 3_(), 5_(), 8_(), 11_()];
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _l: Ptr<()> = c.as_pointer();
            let _pred: () = unimplemented!();
            do_remove_if_8(_l, _pred, 2_())
        });
        let mut i: () = 0_();
        'loop_: while (i < 5_()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_9(c1.as_pointer(), unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_9(c1.as_pointer(), unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: [(); 3] = [5_(), 5_(), 5_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_9(c1.as_pointer(), unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_9(c1.as_pointer(), unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let mut t2: [(); 3] = [5_(), 5_(), 5_()];
        let c1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let c2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(()) -> ()>::new(g_10)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        ({ do_remove_if_9(c1.as_pointer(), unimplemented!(), 1_()) });
        unimplemented!();
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
    unimplemented!();
    return 0_();
}
pub trait PredLWG526Impl {
    fn destructor(&self);
    fn operator_call(&self, p: Ptr<PredLWG526>) -> ();
    fn operator_eq(&self, i: ()) -> ();
}
impl PredLWG526Impl for Ptr<PredLWG526> {
    fn destructor(&self) {
        field!((*self), i_).write(-32767_());
    }
    fn operator_call(&self, p: Ptr<PredLWG526>) -> () {
        return ({ p.with(|__s| __s.i_) } == { (*self).with(|__s| __s.i_) });
    }
    fn operator_eq(&self, mut i: ()) -> () {
        return (i == (*self).with(|__s| __s.i_));
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait unary_counting_predicate_bool__ptr__int___int_Impl {
    fn destructor(&self);
    fn operator_call(&self, a: Ptr<()>) -> ();
    fn count(&self) -> ();
    fn reset(&self) {
        unimplemented!()
    }
}
impl unary_counting_predicate_bool__ptr__int___int_Impl
    for Ptr<unary_counting_predicate_bool__ptr__int___int_>
{
    fn destructor(&self) {}
    fn operator_call(&self, a: Ptr<()>) -> () {
        field!((*self), count_).with_mut(|__v| __v.prefix_inc());
        return ({ (*self).with(|__s| __s.p_.clone()).call((a.read())) });
    }
    fn count(&self) -> () {
        return (*self).with(|__s| __s.count_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
