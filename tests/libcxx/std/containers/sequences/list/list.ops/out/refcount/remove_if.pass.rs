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
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())));
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(FnPtr<fn(i32  ,) -> bool  >  , new, value)]
#[byte_size(16)]
pub struct unary_counting_predicate_bool__ptr__int___int_ {
    #[offset(0)]
    #[byte_size(8)]
    p_: FnPtr<fn(i32) -> bool>,
    #[offset(8)]
    count_: usize,
}
impl unary_counting_predicate_bool__ptr__int___int_ {
    pub fn new(mut p: FnPtr<fn(i32) -> bool>) -> Self {
        Self {
            p_: (p).clone(),
            count_: 0_usize,
        }
    }
}
pub fn even_5(mut i: i32) -> bool {
    return ((i % 2) == 0);
}
pub fn g_6(mut i: i32) -> bool {
    return (i < 3);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Destructor, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct PredLWG526 {
    #[offset(0)]
    pub i_: i32,
}
impl PredLWG526 {
    pub fn new(mut i: i32) -> Self {
        Self { i_: i }
    }
}
pub fn test_7() -> bool {
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 4])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(i32) -> bool>::new(g_6)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        const _: () = assert!(true, "");;
        assert!((unimplemented!() == 2_()));
        assert!(unimplemented!());
        assert!(
            (({ unary_counting_predicate_bool__ptr__int___int_Impl::count(&cp.as_pointer(),) })
                == 4_usize)
        );
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(i32) -> bool>::new(even_5)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        assert!((unimplemented!() == 2_()));
        assert!(unimplemented!());
        assert!(
            (({ unary_counting_predicate_bool__ptr__int___int_Impl::count(&cp.as_pointer(),) })
                == 4_usize)
        );
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 1, 3, 5, 8, 11])));
        let mut a2: [i32; 5] = [2, 3, 5, 8, 11];
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        assert!((unimplemented!() == 5_()));
        let mut i: usize = 0_usize;
        'loop_: while (i < (unimplemented!() as usize)) {
            assert!(({ PredLWG526Impl::operator_eq(&unimplemented!(), a2[(i) as usize],) }));
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let a1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
        let a2: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([3, 4])));
        let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let cp: Value<unary_counting_predicate_bool__ptr__int___int_> = Rc::new(RefCell::new(
            unary_counting_predicate_bool__ptr__int___int_::new({
                FnPtr::<fn(i32) -> bool>::new(g_6)
            }),
        ));
        let _dtor_cp = ScopedDestructor::new(&cp, |__p| __p.destructor());
        assert!((unimplemented!() == 2_()));
        assert!((unimplemented!()));
        assert!(
            (({ unary_counting_predicate_bool__ptr__int___int_Impl::count(&cp.as_pointer(),) })
                == 4_usize)
        );
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
    assert!(({ test_7() }));
    return 0;
}
pub trait PredLWG526Impl {
    fn destructor(&self);
    fn operator_call(&self, p: Ptr<PredLWG526>) -> bool;
    fn operator_eq(&self, i: i32) -> bool;
}
impl PredLWG526Impl for Ptr<PredLWG526> {
    fn destructor(&self) {
        field!((*self), i_).write(-32767_i32);
    }
    fn operator_call(&self, p: Ptr<PredLWG526>) -> bool {
        return ({ p.with(|__s| __s.i_) } == { (*self).with(|__s| __s.i_) });
    }
    fn operator_eq(&self, mut i: i32) -> bool {
        return (i == (*self).with(|__s| __s.i_));
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait unary_counting_predicate_bool__ptr__int___int_Impl {
    fn destructor(&self);
    fn operator_call(&self, a: Ptr<i32>) -> bool;
    fn count(&self) -> usize;
    fn reset(&self) {
        unimplemented!()
    }
}
impl unary_counting_predicate_bool__ptr__int___int_Impl
    for Ptr<unary_counting_predicate_bool__ptr__int___int_>
{
    fn destructor(&self) {}
    fn operator_call(&self, a: Ptr<i32>) -> bool {
        field!((*self), count_).with_mut(|__v| __v.prefix_inc());
        return ({ (*self).with(|__s| __s.p_.clone()).call((a.read())) });
    }
    fn count(&self) -> usize {
        return (*self).with(|__s| __s.count_);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
