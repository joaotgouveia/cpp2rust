extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_1(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: i32,
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_i32 };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for DefaultOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for DefaultOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_4.with(|rc| *rc.borrow()) >= dealloc_count_5.with(|rc| *rc.borrow())));
        return ((alloc_count_4.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_5.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_6.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_3.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_4.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_4: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_5: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_6: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_7(LHS: Ptr<detail_TypeID>, RHS: Ptr<detail_TypeID>) -> bool {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_8(LHS: Ptr<detail_TypeID>, RHS: Ptr<detail_TypeID>) -> bool {
    return ({ LHS.with(|__s| __s.m_id.clone()) } != { RHS.with(|__s| __s.m_id.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct detail_TypeID {
    #[offset(0)]
    #[byte_size(8)]
    m_id: Ptr<i32>,
}
impl detail_TypeID {
    fn new(mut xid: Ptr<i32>) -> Self {
        Self {
            m_id: (xid).clone(),
        }
    }
}
impl CppEq for detail_TypeID {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for detail_TypeID {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    static dummy_addr_9: Value<i32> = Rc::new(RefCell::new(42));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_TypeInfo_detail_ArgumentListID_const_int_ref__ {}
impl detail_TypeInfo_detail_ArgumentListID_const_int_ref__ {
    pub fn GetID() -> Ptr<detail_TypeID> {
        return id_10.with(|v| v.as_pointer());
    }
}
impl CppMove for detail_TypeInfo_detail_ArgumentListID_const_int_ref__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn makeTypeID_11() -> Ptr<detail_TypeID> {
    return ({ detail_TypeInfo_detail_ArgumentListID_const_int_ref__::GetID() });
}
pub fn makeArgumentID_12() -> Ptr<detail_TypeID> {
    return ({ makeTypeID_11() });
}
thread_local!(
    static id_10: Value<detail_TypeID> = Rc::new(RefCell::new(detail_TypeID::new({
        (dummy_addr_9.with(|v| v.as_pointer()))
    })));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct AllocatorConstructController {
    #[offset(0)]
    #[byte_size(8)]
    pub m_expected_args: Ptr<detail_TypeID>,
    #[offset(8)]
    pub m_allow_constructions: bool,
    #[offset(9)]
    pub m_allow_unchecked: bool,
    #[offset(12)]
    pub m_expected_count: i32,
}
impl AllocatorConstructController {
    fn new() -> Self {
        let __this: Value<AllocatorConstructController> = Rc::new(RefCell::new(Self {
            m_expected_args: Ptr::<detail_TypeID>::null(),
            m_allow_constructions: false,
            m_allow_unchecked: false,
            m_expected_count: 0_i32,
        }));
        let this: Ptr<AllocatorConstructController> = __this.as_pointer();
        ({ AllocatorConstructControllerImpl::reset(&this) });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for AllocatorConstructController {
    fn default() -> Self {
        { AllocatorConstructController::new() }
    }
}
pub fn getConstructController_13() -> Ptr<AllocatorConstructController> {
    return (c_14.with(|v| v.as_pointer()));
}
thread_local!(
    static c_14: Value<AllocatorConstructController> =
        Rc::new(RefCell::new(AllocatorConstructController::new()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ContainerTestAllocator_int__int_ {
    #[offset(0)]
    pub construct_called: i32,
    #[offset(4)]
    pub destroy_called: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub controller: Ptr<AllocatorConstructController>,
}
impl ContainerTestAllocator_int__int_ {}
impl CppMove for ContainerTestAllocator_int__int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ContainerTestAllocator_float__int_ {
    #[offset(0)]
    pub construct_called: i32,
    #[offset(4)]
    pub destroy_called: i32,
    #[offset(8)]
    #[byte_size(8)]
    pub controller: Ptr<AllocatorConstructController>,
}
impl ContainerTestAllocator_float__int_ {}
impl CppMove for ContainerTestAllocator_float__int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
const _: () = assert!(true, "std::is_same<A1T::rebind_traits<float>, A2T>::value");
const _: () = assert!(true, "std::is_same<A2T::rebind_traits<int>, A1T>::value");
pub fn test_15() -> bool {
    {
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
        let c: Value<std_forward_list_int__std_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1(
                { ({ begin_16(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        ({
            let ___val: Value<i32> = Rc::new(RefCell::new(10));
            (*c.borrow()).resize_52(3_u64, ___val.as_pointer())
        });
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_17(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        ({
            let ___val: Value<i32> = Rc::new(RefCell::new(10));
            (*c.borrow()).resize_52(6_u64, ___val.as_pointer())
        });
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_17(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 3_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 4_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 5_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        ({
            let ___val: Value<i32> = Rc::new(RefCell::new(12));
            (*c.borrow()).resize_52(6_u64, ___val.as_pointer())
        });
        assert!(
            ({
                (({
                    let ___first: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).begin_25() });
                    let ___last: std__Fwd_list_iterator_int_ = ({ (*c.borrow()).end_27() });
                    distance_17(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 3_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 4_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 5_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
    }
    {
        let t: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4])));
        let c: Value<std_forward_list_int__min_allocator_int__> = Rc::new(RefCell::new(
            std_forward_list_int__min_allocator_int__::new_1(
                { ({ begin_16(t.as_pointer()) }) },
                { (t.as_pointer() as Ptr<i32>).to_end() },
                None,
            ),
        ));
        ({
            let ___val: Value<i32> = Rc::new(RefCell::new(10));
            (*c.borrow()).resize_52(3_u64, ___val.as_pointer())
        });
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_18(___first, ___last)
                }) == 3_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        ({
            let ___val: Value<i32> = Rc::new(RefCell::new(10));
            (*c.borrow()).resize_52(6_u64, ___val.as_pointer())
        });
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_18(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 3_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 4_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 5_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        ({
            let ___val: Value<i32> = Rc::new(RefCell::new(12));
            (*c.borrow()).resize_52(6_u64, ___val.as_pointer())
        });
        assert!(
            ({
                (({
                    let ___first: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . begin_25 ( ) } )   ;
                    let ___last: std___fwdlist__Iterator_false__min_pointer_int__std_integral_constant_unsigned_long__0___   = ( { (*c.borrow()) . end_27 ( ) } )   ;
                    distance_18(___first, ___last)
                }) == 6_isize)
            })
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 0_isize as isize)
                .iter_deref()
                .read())
                == 0)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 1_isize as isize)
                .iter_deref()
                .read())
                == 1)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 2_isize as isize)
                .iter_deref()
                .read())
                == 2)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 3_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 4_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
        assert!(
            ((libcc2rs::next_refcount(({ ({ (*c.borrow()).begin_25() }) }), 5_isize as isize)
                .iter_deref()
                .read())
                == 10)
        );
    }
    {
        let mut cc: Ptr<AllocatorConstructController> = ({ getConstructController_13() });
        ({ AllocatorConstructControllerImpl::reset(&cc) });
        {
            let c: Value<std_forward_list_int__ContainerTestAllocator_int__int__> = Rc::new(
                RefCell::new(<std_forward_list_int__ContainerTestAllocator_int__int__>::default()),
            );
            ({ AllocatorConstructControllerImpl::expect_5_const_int_ref(&cc, 6) });
            ({
                let ___val: Value<i32> = Rc::new(RefCell::new(42));
                (*c.borrow()).resize_52(6_u64, ___val.as_pointer())
            });
            assert!(!({ AllocatorConstructControllerImpl::unchecked(&cc,) }));
        }
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
    assert!(({ test_15() }));
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait AllocatorConstructControllerImpl {
    fn clear(&self);
    fn check(&self, tid: Ptr<detail_TypeID>) -> bool;
    fn unchecked(&self) -> bool;
    fn expect(&self, tid: Ptr<detail_TypeID>);
    fn isInAllocatorConstruct(&self) -> bool;
    fn inAllocatorConstruct(&self, value: Option<bool>);
    fn allowUnchecked(&self, value: Option<bool>);
    fn reset(&self);
    fn copy_assign(
        &self,
        _a0: Ptr<AllocatorConstructController>,
    ) -> Ptr<AllocatorConstructController> {
        unimplemented!()
    }
    fn expect_5_const_int_ref(&self, times: i32);
    fn check_6_const_int_ref(&self) -> bool;
}
impl AllocatorConstructControllerImpl for Ptr<AllocatorConstructController> {
    fn clear(&self) {
        field!((*self), m_expected_args).write(Ptr::<detail_TypeID>::null());
        field!((*self), m_expected_count).write(-1_i32);
    }
    fn check(&self, tid: Ptr<detail_TypeID>) -> bool {
        if !(!((*self).with(|__s| __s.m_expected_args.clone())).is_null()) {
            assert!((*self).with(|__s| __s.m_allow_unchecked));
            return (*self).with(|__s| __s.m_allow_unchecked);
        }
        let res: Value<bool> = Rc::new(RefCell::new(
            ({
                let _LHS: Ptr<detail_TypeID> = (*self).with(|__s| __s.m_expected_args.clone());
                let _RHS: Ptr<detail_TypeID> = (tid).clone();
                operator_eq_7(_LHS, _RHS)
            }),
        ));
        if ((*self).with(|__s| __s.m_expected_count) == -1_i32)
            || (field!((*self), m_expected_count).with_mut(|__v| __v.prefix_dec().clone())
                == -1_i32)
        {
            field!((*self), m_expected_args).write(Ptr::<detail_TypeID>::null());
        }
        return (*res.borrow());
    }
    fn unchecked(&self) -> bool {
        return !(((*self).with(|__s| __s.m_expected_args.clone())).is_null());
    }
    fn expect(&self, tid: Ptr<detail_TypeID>) {
        assert!(!({ AllocatorConstructControllerImpl::unchecked(self,) }));
        field!((*self), m_expected_args).write((tid).clone());
    }
    fn isInAllocatorConstruct(&self) -> bool {
        return (*self).with(|__s| __s.m_allow_constructions);
    }
    fn inAllocatorConstruct(&self, value: Option<bool>) {
        let mut value: bool = value.unwrap_or_else(|| true);
        field!((*self), m_allow_constructions).write(value);
    }
    fn allowUnchecked(&self, value: Option<bool>) {
        let mut value: bool = value.unwrap_or_else(|| true);
        field!((*self), m_allow_unchecked).write(value);
    }
    fn reset(&self) {
        field!((*self), m_allow_constructions).write(false);
        field!((*self), m_expected_args).write(Ptr::<detail_TypeID>::null());
        field!((*self), m_allow_unchecked).write(false);
        field!((*self), m_expected_count).write(-1_i32);
    }
    fn expect_5_const_int_ref(&self, mut times: i32) {
        assert!(!({ AllocatorConstructControllerImpl::unchecked(self,) }));
        assert!((times > 0));
        field!((*self), m_expected_count).write((times - 1));
        field!((*self), m_expected_args).write(({ makeArgumentID_12() }));
    }
    fn check_6_const_int_ref(&self) -> bool {
        return ({ AllocatorConstructControllerImpl::check(self, ({ makeArgumentID_12() })) });
    }
}
pub trait ContainerTestAllocator_float__int_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<f32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<f32>, mut _a1: usize) {
        unimplemented!()
    }
}
impl ContainerTestAllocator_float__int_Impl for Ptr<ContainerTestAllocator_float__int_> {}
pub trait ContainerTestAllocator_int__int_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<i32>, mut _a1: usize) {
        unimplemented!()
    }
}
impl ContainerTestAllocator_int__int_Impl for Ptr<ContainerTestAllocator_int__int_> {}
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
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
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
    let _ = dummy_addr_9.with(|_| ());
}
