extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(a: Ptr<test_less_int_>, b: Ptr<test_less_int_>) -> () {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: (),
}
impl test_less_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0_() }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl std::cmp::PartialEq for test_less_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(test_less_int_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_less_int_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
impl Default for test_less_int_ {
    fn default() -> Self {
        { test_less_int_::new_1() }
    }
}
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct tracking_allocator_int_ {
    #[offset(0)]
    #[byte_size(8)]
    allocs_: Ptr<()>,
}
impl tracking_allocator_int_ {
    pub fn new(allocs: Ptr<()>) -> Self {
        Self {
            allocs_: (allocs).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoOp {}
pub fn test_alloc_5(
    lhs_alloc: Option<Ptr<()>>,
    rhs_alloc: Option<Ptr<()>>,
    check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<Value<()>> = None;
    let lhs_alloc: Ptr<()> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<()>> = None;
    let rhs_alloc: Ptr<()> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [4_(), 5_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut i: () = 0_();
            'loop_: while (i != 50_()) {
                unimplemented!();
                i.prefix_inc();
            }
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            let mut i: () = 0_();
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: () = (unimplemented!().read());
                unimplemented!();
            }
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_alloc_6(
    lhs_alloc: Option<Ptr<min_allocator_int_>>,
    rhs_alloc: Option<Ptr<min_allocator_int_>>,
    check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<Value<min_allocator_int_>> = None;
    let lhs_alloc: Ptr<min_allocator_int_> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(<min_allocator_int_>::default())))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<min_allocator_int_>> = None;
    let rhs_alloc: Ptr<min_allocator_int_> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(<min_allocator_int_>::default())))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [4_(), 5_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut i: () = 0_();
            'loop_: while (i != 50_()) {
                unimplemented!();
                i.prefix_inc();
            }
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            let mut i: () = 0_();
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: () = (unimplemented!().read());
                unimplemented!();
            }
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_alloc_7(
    lhs_alloc: Ptr<tracking_allocator_int_>,
    rhs_alloc: Ptr<tracking_allocator_int_>,
    check_alloc_invariant: AssertEmpty_8,
) {
    let check_alloc_invariant: Value<AssertEmpty_8> = Rc::new(RefCell::new(check_alloc_invariant));
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_8Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_8Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        ({ AssertEmpty_8Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [4_(), 5_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_8Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_8Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_8Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut i: () = 0_();
            'loop_: while (i != 50_()) {
                unimplemented!();
                i.prefix_inc();
            }
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            let mut i: () = 0_();
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: () = (unimplemented!().read());
                unimplemented!();
            }
        }
        ({ AssertEmpty_8Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_9() -> () {
    ({ test_alloc_5(None, None, None) });
    ({ test_alloc_6(None, None, None) });
    {
        let lhs_allocs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let rhs_allocs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _lhs_alloc: Value<tracking_allocator_int_> =
                Rc::new(RefCell::new(tracking_allocator_int_::new({
                    lhs_allocs.as_pointer()
                })));
            let _rhs_alloc: Value<tracking_allocator_int_> =
                Rc::new(RefCell::new(tracking_allocator_int_::new({
                    rhs_allocs.as_pointer()
                })));
            let _check_alloc_invariant: AssertEmpty_8 =
                AssertEmpty_8::new({ lhs_allocs.as_pointer() }, { rhs_allocs.as_pointer() });
            test_alloc_7(
                _lhs_alloc.as_pointer(),
                _rhs_alloc.as_pointer(),
                _check_alloc_invariant,
            )
        });
    }
    {
        let mut arr: [(); 3] = [1_(), 2_(), 2_()];
        let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct AssertEmpty_8 {
    #[offset(0)]
    #[byte_size(8)]
    lhs_allocs_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    rhs_allocs_: Ptr<()>,
}
impl AssertEmpty_8 {
    pub fn new(lhs_allocs: Ptr<()>, rhs_allocs: Ptr<()>) -> Self {
        Self {
            lhs_allocs_: (lhs_allocs).clone(),
            rhs_allocs_: (rhs_allocs).clone(),
        }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_9() });
    return 0_();
}
pub trait AssertEmpty_8Impl {
    fn operator_call(&self);
}
impl AssertEmpty_8Impl for Ptr<AssertEmpty_8> {
    fn operator_call(&self) {
        unimplemented!();
        unimplemented!();
    }
}
pub trait NoOpImpl {
    fn operator_call(&self);
}
impl NoOpImpl for Ptr<NoOp> {
    fn operator_call(&self) {}
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
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> ();
}
impl test_less_int_Impl for Ptr<test_less_int_> {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> () {
        return ({ (a.read()) } < { (b.read()) });
    }
}
pub trait tracking_allocator_int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl tracking_allocator_int_Impl for Ptr<tracking_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
