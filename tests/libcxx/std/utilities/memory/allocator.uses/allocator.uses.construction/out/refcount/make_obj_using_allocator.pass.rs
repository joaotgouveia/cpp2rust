extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_0: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_1: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_2(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_2(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct test_allocator_int_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_int_ {
    pub fn new_1(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_int_>) -> Self {
        let __this: Value<test_allocator_int_> = Rc::new(RefCell::new(Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }));
        let this: Ptr<test_allocator_int_> = __this.as_pointer();
        unimplemented!();
        if !((this.with(|__s| __s.stats_.clone())).is_null()) {
            field!(this.with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_inc());
            field!(this.with(|__s| __s.stats_.clone()), copied).with_mut(|__v| __v.prefix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_int_> = Rc::new(RefCell::new(test_allocator_int_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_int_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        {
            test_allocator_int_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
pub type RefType = ();
pub const RefType_LValue: RefType = 0;
pub const RefType_ConstLValue: RefType = 1;
pub const RefType_RValue: RefType = 2;
pub const RefType_ConstRValue: RefType = 3;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(48)]
pub struct UsesAllocArgT {
    #[offset(0)]
    pub allocator_constructed_: (),
    #[offset(8)]
    #[byte_size(16)]
    pub a_: test_allocator_int_,
    #[offset(24)]
    #[byte_size(8)]
    pub alloc_: Ptr<test_allocator_int_>,
    #[offset(32)]
    #[byte_size(8)]
    pub val_ptr_: Ptr<()>,
    #[offset(40)]
    #[byte_size(4)]
    pub ref_type_: RefType,
}
impl UsesAllocArgT {
    pub fn new_1(_a0: (), alloc: Ptr<test_allocator_int_>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: Ptr::<()>::null(),
            ref_type_: <RefType>::default(),
        }
    }
    pub fn new_2(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_LValue,
        }
    }
    pub fn new_3(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstLValue,
        }
    }
    pub fn new_4(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_RValue,
        }
    }
    pub fn new_5(_a0: (), alloc: Ptr<test_allocator_int_>, val: Ptr<()>) -> Self {
        let _a0: Value<()> = Rc::new(RefCell::new(_a0));
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstRValue,
        }
    }
}
impl Default for UsesAllocArgT {
    fn default() -> Self {
        {
            UsesAllocArgT {
                allocator_constructed_: false,
                a_: <test_allocator_int_>::default(),
                alloc_: field_ptr!((*self), a_),
                val_ptr_: Ptr::<()>::null(),
                ref_type_: <RefType>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(48)]
pub struct UsesAllocLast {
    #[offset(0)]
    pub allocator_constructed_: (),
    #[offset(8)]
    #[byte_size(16)]
    pub a_: test_allocator_int_,
    #[offset(24)]
    #[byte_size(8)]
    pub alloc_: Ptr<test_allocator_int_>,
    #[offset(32)]
    #[byte_size(8)]
    pub val_ptr_: Ptr<()>,
    #[offset(40)]
    #[byte_size(4)]
    pub ref_type_: RefType,
}
impl UsesAllocLast {
    pub fn new_1(alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: Ptr::<()>::null(),
            ref_type_: <RefType>::default(),
        }
    }
    pub fn new_2(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_LValue,
        }
    }
    pub fn new_3(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstLValue,
        }
    }
    pub fn new_4(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_RValue,
        }
    }
    pub fn new_5(val: Ptr<()>, alloc: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
            a_: <test_allocator_int_>::default(),
            alloc_: (alloc).clone(),
            val_ptr_: (val).clone(),
            ref_type_: RefType_ConstRValue,
        }
    }
}
impl Default for UsesAllocLast {
    fn default() -> Self {
        {
            UsesAllocLast {
                allocator_constructed_: false,
                a_: <test_allocator_int_>::default(),
                alloc_: field_ptr!((*self), a_),
                val_ptr_: Ptr::<()>::null(),
                ref_type_: <RefType>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct NotAllocatorAware {
    #[offset(0)]
    pub allocator_constructed_: (),
}
impl NotAllocatorAware {
    pub fn new_1(_a0: Ptr<test_allocator_int_>) -> Self {
        Self {
            allocator_constructed_: true,
        }
    }
    pub fn new_2(_a0: Ptr<test_allocator_int_>, mut _a1: ()) -> Self {
        Self {
            allocator_constructed_: true,
        }
    }
}
impl Default for NotAllocatorAware {
    fn default() -> Self {
        {
            NotAllocatorAware {
                allocator_constructed_: false,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ConvertibleToPair {}
pub fn test_4() -> () {
    let a: Value<test_allocator_int_> =
        Rc::new(RefCell::new(test_allocator_int_::new_1({ 12_() })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    {
        let ret: Value<UsesAllocArgT> = Rc::new(RefCell::new(unimplemented!()));
        let _dtor_ret = ScopedDestructor::new(&ret, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<UsesAllocLast> = Rc::new(RefCell::new(unimplemented!()));
        let _dtor_ret = ScopedDestructor::new(&ret, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: NotAllocatorAware = unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        let mut ctp: ConvertibleToPair = <ConvertibleToPair>::default();
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let ctp: Value<ConvertibleToPair> = Rc::new(RefCell::new(<ConvertibleToPair>::default()));
        let ret: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    ({ test_4() });
    const _: () = assert!(true, "test()");;
    0
}
pub trait ConvertibleToPairImpl {
    fn to_(&self) -> ();
}
impl ConvertibleToPairImpl for Ptr<ConvertibleToPair> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait UsesAllocArgTImpl {
    fn destructor(&self);
}
impl UsesAllocArgTImpl for Ptr<UsesAllocArgT> {
    fn destructor(&self) {
        field_ptr!(self, a_).destructor();
    }
}
pub trait UsesAllocLastImpl {
    fn destructor(&self);
}
impl UsesAllocLastImpl for Ptr<UsesAllocLast> {
    fn destructor(&self) {
        field_ptr!(self, a_).destructor();
    }
}
pub trait test_allocator_int_Impl {
    fn destructor(&self);
    fn address_11(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn address_12(&self, x: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn allocate(&self, mut n: (), mut _a1: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut p: Ptr<()>, mut s: ()) {
        unimplemented!()
    }
    fn max_size(&self) -> () {
        unimplemented!()
    }
    fn destroy(&self, mut p: Ptr<()>) {
        unimplemented!()
    }
    fn get_data(&self) -> () {
        unimplemented!()
    }
    fn get_id(&self) -> () {
        unimplemented!()
    }
}
impl test_allocator_int_Impl for Ptr<test_allocator_int_> {
    fn destructor(&self) {
        unimplemented!();
        unimplemented!();
        if !(((*self).with(|__s| __s.stats_.clone())).is_null()) {
            field!((*self).with(|__s| __s.stats_.clone()), count).with_mut(|__v| __v.prefix_dec());
        }
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_0.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_0.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_0.with(|_| ());
    let _ = moved_value_1.with(|_| ());
}
