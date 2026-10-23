extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_1(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: (),
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_() };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl std::cmp::Ord for DefaultOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_2(
                Rc::new(RefCell::new(DefaultOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DefaultOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(
                Rc::new(RefCell::new(DefaultOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DefaultOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for DefaultOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(DefaultOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(DefaultOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for DefaultOnly {}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_4.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_5.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_6.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_3.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_4.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_6: Value<()> = Rc::new(RefCell::new(false));
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
pub fn operator_eq_7(LHS: Ptr<detail_TypeID>, RHS: Ptr<detail_TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } == { RHS.with(|__s| __s.m_id.clone()) });
}
pub fn operator_ne_8(LHS: Ptr<detail_TypeID>, RHS: Ptr<detail_TypeID>) -> () {
    return ({ LHS.with(|__s| __s.m_id.clone()) } != { RHS.with(|__s| __s.m_id.clone()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct detail_TypeID {
    #[offset(0)]
    #[byte_size(8)]
    m_id: Ptr<()>,
}
impl detail_TypeID {
    fn new(mut xid: Ptr<()>) -> Self {
        Self {
            m_id: (xid).clone(),
        }
    }
}
impl std::cmp::PartialEq for detail_TypeID {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_7(
                Rc::new(RefCell::new(detail_TypeID {
                    m_id: self.m_id.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(detail_TypeID {
                    m_id: other.m_id.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for detail_TypeID {}
thread_local!(
    static dummy_addr_9: Value<()> = Rc::new(RefCell::new(42_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct detail_TypeInfo_detail_ArgumentListID_const_int_ref__ {}
impl detail_TypeInfo_detail_ArgumentListID_const_int_ref__ {
    pub fn GetID() -> Ptr<detail_TypeID> {
        return id_10.with(|v| v.as_pointer());
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
    pub m_allow_constructions: (),
    #[offset(9)]
    pub m_allow_unchecked: (),
    #[offset(12)]
    pub m_expected_count: (),
}
impl AllocatorConstructController {
    fn new() -> Self {
        let __this: Value<AllocatorConstructController> = Rc::new(RefCell::new(Self {
            m_expected_args: Ptr::<detail_TypeID>::null(),
            m_allow_constructions: unimplemented!(),
            m_allow_unchecked: unimplemented!(),
            m_expected_count: unimplemented!(),
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
    pub construct_called: (),
    #[offset(4)]
    pub destroy_called: (),
    #[offset(8)]
    #[byte_size(8)]
    pub controller: Ptr<AllocatorConstructController>,
}
impl ContainerTestAllocator_int__int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct ContainerTestAllocator_float__int_ {
    #[offset(0)]
    pub construct_called: (),
    #[offset(4)]
    pub destroy_called: (),
    #[offset(8)]
    #[byte_size(8)]
    pub controller: Ptr<AllocatorConstructController>,
}
impl ContainerTestAllocator_float__int_ {}
const _: () = assert!(true, "std::is_same<A1T::rebind_traits<float>, A2T>::value");
const _: () = assert!(true, "std::is_same<A2T::rebind_traits<int>, A1T>::value");
pub fn test_15() -> () {
    {
        let mut t: [(); 5] = [0_(), 1_(), 2_(), 3_(), 4_()];
        let c: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__std_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
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
    {
        let mut t: [(); 5] = [0_(), 1_(), 2_(), 3_(), 4_()];
        let c: Value<()> = Rc::new(RefCell::new(
            std_forward_list_int__min_allocator_int__::new_1({ unimplemented!() }, {
                unimplemented!()
            }),
        ));
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
    {
        let mut cc: Ptr<AllocatorConstructController> = ({ getConstructController_13() });
        ({ AllocatorConstructControllerImpl::reset(&cc) });
        {
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ AllocatorConstructControllerImpl::expect_5_const_int_ref(&cc, 6_()) });
            unimplemented!();
            unimplemented!();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    unimplemented!();
    return 0_();
}
pub trait AllocatorConstructControllerImpl {
    fn clear(&self);
    fn check(&self, tid: Ptr<detail_TypeID>) -> ();
    fn unchecked(&self) -> ();
    fn expect(&self, tid: Ptr<detail_TypeID>);
    fn isInAllocatorConstruct(&self) -> ();
    fn inAllocatorConstruct(&self, value: Option<()>);
    fn allowUnchecked(&self, value: Option<()>);
    fn reset(&self);
    fn copy_assign(
        &self,
        _a0: Ptr<AllocatorConstructController>,
    ) -> Ptr<AllocatorConstructController> {
        unimplemented!()
    }
    fn expect_5_const_int_ref(&self, times: ());
    fn check_6_const_int_ref(&self) -> ();
}
impl AllocatorConstructControllerImpl for Ptr<AllocatorConstructController> {
    fn clear(&self) {
        field!((*self), m_expected_args).write(Ptr::<detail_TypeID>::null());
        field!((*self), m_expected_count).write(-1_());
    }
    fn check(&self, tid: Ptr<detail_TypeID>) -> () {
        if !(!((*self).with(|__s| __s.m_expected_args.clone())).is_null()) {
            unimplemented!();
            return (*self).with(|__s| __s.m_allow_unchecked);
        }
        let res: Value<()> = Rc::new(RefCell::new(
            ({
                let _LHS: Ptr<detail_TypeID> = (*self).with(|__s| __s.m_expected_args.clone());
                let _RHS: Ptr<detail_TypeID> = (tid).clone();
                operator_eq_7(_LHS, _RHS)
            }),
        ));
        if ((*self).with(|__s| __s.m_expected_count) == -1_())
            || (field!((*self), m_expected_count).with_mut(|__v| __v.prefix_dec()) == -1_())
        {
            field!((*self), m_expected_args).write(Ptr::<detail_TypeID>::null());
        }
        return (*res.borrow());
    }
    fn unchecked(&self) -> () {
        return !(((*self).with(|__s| __s.m_expected_args.clone())).is_null());
    }
    fn expect(&self, tid: Ptr<detail_TypeID>) {
        unimplemented!();
        field!((*self), m_expected_args).write((tid).clone());
    }
    fn isInAllocatorConstruct(&self) -> () {
        return (*self).with(|__s| __s.m_allow_constructions);
    }
    fn inAllocatorConstruct(&self, value: Option<()>) {
        let mut value: () = value.unwrap_or_else(|| true);
        field!((*self), m_allow_constructions).write(value);
    }
    fn allowUnchecked(&self, value: Option<()>) {
        let mut value: () = value.unwrap_or_else(|| true);
        field!((*self), m_allow_unchecked).write(value);
    }
    fn reset(&self) {
        field!((*self), m_allow_constructions).write(false);
        field!((*self), m_expected_args).write(Ptr::<detail_TypeID>::null());
        field!((*self), m_allow_unchecked).write(false);
        field!((*self), m_expected_count).write(-1_());
    }
    fn expect_5_const_int_ref(&self, mut times: ()) {
        unimplemented!();
        unimplemented!();
        field!((*self), m_expected_count).write((times - 1_()));
        field!((*self), m_expected_args).write(({ makeArgumentID_12() }));
    }
    fn check_6_const_int_ref(&self) -> () {
        return ({ AllocatorConstructControllerImpl::check(self, ({ makeArgumentID_12() })) });
    }
}
pub trait ContainerTestAllocator_float__int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: ()) {
        unimplemented!()
    }
}
impl ContainerTestAllocator_float__int_Impl for Ptr<ContainerTestAllocator_float__int_> {}
pub trait ContainerTestAllocator_int__int_Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut _a1: ()) {
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
        field!((*self), data_).write(0_());
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
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
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
    let _ = dummy_addr_9.with(|_| ());
}
