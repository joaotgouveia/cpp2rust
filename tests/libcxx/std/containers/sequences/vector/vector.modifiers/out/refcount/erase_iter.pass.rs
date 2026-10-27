extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_contiguous_container_asan_correct_0(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_1(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_2(_a0: Ptr<()>) -> () {
    return true;
}
pub fn is_contiguous_container_asan_correct_3(_a0: Ptr<()>) -> () {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Tracker {
    #[offset(0)]
    pub copy_assignments: (),
    #[offset(4)]
    pub move_assignments: (),
}
impl Default for Tracker {
    fn default() -> Self {
        {
            Tracker {
                copy_assignments: 0_(),
                move_assignments: 0_(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(8)]
pub struct TrackedAssignment {
    #[offset(0)]
    #[byte_size(8)]
    pub tracker_: Ptr<Tracker>,
}
impl TrackedAssignment {
    pub fn new(mut tracker: Ptr<Tracker>) -> Self {
        Self {
            tracker_: (tracker).clone(),
        }
    }
}
pub fn operator_eq_4(a: Ptr<NonTriviallyRelocatable>, b: Ptr<NonTriviallyRelocatable>) -> () {
    return ({ a.with(|__s| __s.value_) } == { b.with(|__s| __s.value_) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(4)]
pub struct NonTriviallyRelocatable {
    #[offset(0)]
    pub value_: (),
}
impl NonTriviallyRelocatable {
    pub fn new_1() -> Self {
        Self { value_: 0_() }
    }
    pub fn new_2(mut v: ()) -> Self {
        Self { value_: v }
    }
    pub fn copy_from(other: Ptr<NonTriviallyRelocatable>) -> Self {
        Self {
            value_: other.with(|__s| __s.value_),
        }
    }
    pub fn move_from(other: Ptr<NonTriviallyRelocatable>) -> Self {
        Self {
            value_: other.with(|__s| __s.value_),
        }
    }
}
impl std::cmp::PartialEq for NonTriviallyRelocatable {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_4(
                Rc::new(RefCell::new(NonTriviallyRelocatable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NonTriviallyRelocatable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NonTriviallyRelocatable {}
impl Clone for NonTriviallyRelocatable {
    fn clone(&self) -> Self {
        let __src: Value<NonTriviallyRelocatable> =
            Rc::new(RefCell::new(NonTriviallyRelocatable {
                value_: self.value_.clone(),
            }));
        NonTriviallyRelocatable::copy_from(__src.as_pointer())
    }
}
impl Default for NonTriviallyRelocatable {
    fn default() -> Self {
        { NonTriviallyRelocatable::new_1() }
    }
}
const _: () = assert!(
    true,
    "!std::__libcpp_is_trivially_relocatable<NonTriviallyRelocatable>::value"
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_7: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_8: Value<()> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_10(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<()>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<()>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_12(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_ne_13(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_14(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_15(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_16(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_15(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MoveOnly>,
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<MoveOnly>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<MoveOnly>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_17(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_ne_18(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MoveOnly>,
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
pub fn operator_sub_19(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_20(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_21(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_20(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<TrivialMoveOnly>,
}
impl min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<TrivialMoveOnly>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<TrivialMoveOnly>::null(),
        }
    }
}
impl std::cmp::PartialEq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_20(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_22(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_ne_23(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<TrivialMoveOnly>,
}
impl min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(
        mut p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
pub fn operator_sub_24(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_eq_25(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_26(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return !({
        let _x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_25(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<NonTriviallyRelocatable>,
}
impl min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<NonTriviallyRelocatable>) -> Self {
        Self { ptr_: (p).clone() }
    }
    pub fn new_2(mut _a0: ()) -> Self {
        Self {
            ptr_: Ptr::<NonTriviallyRelocatable>::null(),
        }
    }
}
impl std::cmp::PartialEq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        { operator_eq_25(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
pub fn operator_sub_27(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as () as ());
}
pub fn operator_ne_28(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<NonTriviallyRelocatable>,
}
impl min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(
        mut p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>,
}
impl min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_TrivialMoveOnly_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_NonTriviallyRelocatable_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_const_int__std_integral_constant_unsigned_long__0___ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___
{}
pub fn operator_eq_29(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_30(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_31(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_32(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_33(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_34(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_35(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: (),
}
impl MoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0_());
        __this
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_35(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_29(
                Rc::new(RefCell::new(MoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(MoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_36(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_37(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_38(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_39(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_40(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_41(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> () {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_42(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: (),
}
impl TrivialMoveOnly {
    pub fn new(data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| 1_());
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_42(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_36(
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(TrivialMoveOnly {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
pub fn tests_43() {
    {
        let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 3_(), 4_(), 5_()])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn tests_44() {
    {
        let arr: Value<Box<[NonTriviallyRelocatable]>> = Rc::new(RefCell::new(Box::new([
            NonTriviallyRelocatable::new_2({ 1_() }),
            NonTriviallyRelocatable::new_2({ 2_() }),
            NonTriviallyRelocatable::new_2({ 3_() }),
            NonTriviallyRelocatable::new_2({ 4_() }),
            NonTriviallyRelocatable::new_2({ 5_() }),
        ])));
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1_() }),
                    NonTriviallyRelocatable::new_2({ 3_() }),
                    NonTriviallyRelocatable::new_2({ 4_() }),
                    NonTriviallyRelocatable::new_2({ 5_() }),
                ])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1_() }),
                    NonTriviallyRelocatable::new_2({ 2_() }),
                    NonTriviallyRelocatable::new_2({ 3_() }),
                    NonTriviallyRelocatable::new_2({ 4_() }),
                ])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn tests_45() {
    {
        let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 3_(), 4_(), 5_()])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn tests_46() {
    {
        let arr: Value<Box<[NonTriviallyRelocatable]>> = Rc::new(RefCell::new(Box::new([
            NonTriviallyRelocatable::new_2({ 1_() }),
            NonTriviallyRelocatable::new_2({ 2_() }),
            NonTriviallyRelocatable::new_2({ 3_() }),
            NonTriviallyRelocatable::new_2({ 4_() }),
            NonTriviallyRelocatable::new_2({ 5_() }),
        ])));
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1_() }),
                    NonTriviallyRelocatable::new_2({ 3_() }),
                    NonTriviallyRelocatable::new_2({ 4_() }),
                    NonTriviallyRelocatable::new_2({ 5_() }),
                ])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1_() }),
                    NonTriviallyRelocatable::new_2({ 2_() }),
                    NonTriviallyRelocatable::new_2({ 3_() }),
                    NonTriviallyRelocatable::new_2({ 4_() }),
                ])));
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn tests_47() -> () {
    ({ tests_43() });
    ({ tests_44() });
    ({ tests_45() });
    ({ tests_46() });
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
    ({ tests_47() });
    const _: () = assert!(true, "tests()");;
    {
        let tracker: Value<Tracker> = Rc::new(RefCell::new(<Tracker>::default()));
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i != 5_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0_());
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly {
        return MoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait NonTriviallyRelocatableImpl {
    fn copy_assign(&self, other: Ptr<NonTriviallyRelocatable>) -> Ptr<NonTriviallyRelocatable>;
    fn move_assign(&self, other: Ptr<NonTriviallyRelocatable>) -> Ptr<NonTriviallyRelocatable>;
}
impl NonTriviallyRelocatableImpl for Ptr<NonTriviallyRelocatable> {
    fn copy_assign(&self, other: Ptr<NonTriviallyRelocatable>) -> Ptr<NonTriviallyRelocatable> {
        field!((*self), value_).write({ other.with(|__s| __s.value_) });
        return (*self).clone();
    }
    fn move_assign(&self, other: Ptr<NonTriviallyRelocatable>) -> Ptr<NonTriviallyRelocatable> {
        field!((*self), value_).write({ other.with(|__s| __s.value_) });
        return (*self).clone();
    }
}
pub trait TrackedAssignmentImpl {
    fn copy_assign(&self, _a0: Ptr<TrackedAssignment>) -> Ptr<TrackedAssignment>;
    fn move_assign(&self, _a0: Ptr<TrackedAssignment>) -> Ptr<TrackedAssignment>;
}
impl TrackedAssignmentImpl for Ptr<TrackedAssignment> {
    fn copy_assign(&self, _a0: Ptr<TrackedAssignment>) -> Ptr<TrackedAssignment> {
        field!((*self).with(|__s| __s.tracker_.clone()), copy_assignments)
            .with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<TrackedAssignment>) -> Ptr<TrackedAssignment> {
        field!((*self).with(|__s| __s.tracker_.clone()), move_assignments)
            .with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
}
pub trait TrivialMoveOnlyImpl {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly>;
    fn get(&self) -> ();
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } + { x.with(|__s| __s.data_) }))
        });
    }
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({
            Some(({ (*self).with(|__s| __s.data_) } * { x.with(|__s| __s.data_) }))
        });
    }
}
pub trait min_allocator_MoveOnly_Impl {
    fn allocate(&self, n: ()) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_MoveOnly_Impl for Ptr<min_allocator_MoveOnly_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_NonTriviallyRelocatable_Impl {
    fn allocate(
        &self,
        n: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_NonTriviallyRelocatable_Impl for Ptr<min_allocator_NonTriviallyRelocatable_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_TrivialMoveOnly_Impl {
    fn allocate(
        &self,
        n: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
        n: (),
    );
}
impl min_allocator_TrivialMoveOnly_Impl for Ptr<min_allocator_TrivialMoveOnly_> {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: ());
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___Impl for Ptr<min_allocator_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___> {
}
pub trait min_allocator_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___Impl
{
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___Impl for Ptr<min_allocator_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0___> {
}
pub trait min_allocator_min_pointer_const_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_const_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_const_int__std_integral_constant_unsigned_long__0___>
{
}
pub trait min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl {
    fn allocate ( &self , mut n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___Impl
    for Ptr<min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___>
{
}
pub trait min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> ();
    fn operator_deref(&self) -> Ptr<MoveOnly>;
    fn operator_arrow(&self) -> Ptr<MoveOnly>;
    fn operator_inc_8(&self)
    -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_add(&self, n: ())
    -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(&self, n: ())
    -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, n: ()) -> Ptr<MoveOnly>;
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>
{
    fn to_(&self) -> () {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<MoveOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_sub_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_index(&self, mut n: ()) -> Ptr<MoveOnly> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<NonTriviallyRelocatable>;
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<NonTriviallyRelocatable> {
        unimplemented!()
    }
}
impl min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<NonTriviallyRelocatable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        > = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> ();
    fn operator_deref(&self) -> Ptr<TrivialMoveOnly>;
    fn operator_arrow(&self) -> Ptr<TrivialMoveOnly>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        n: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        n: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, n: ()) -> Ptr<TrivialMoveOnly>;
}
impl min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>
{
    fn to_(&self) -> () {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<TrivialMoveOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<TrivialMoveOnly> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
    fn operator_index(&self, mut n: ()) -> Ptr<TrivialMoveOnly> {
        return (*self).with(|__s| __s.ptr_.clone()).offset((n) as isize);
    }
}
pub trait min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<NonTriviallyRelocatable> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<NonTriviallyRelocatable> {
        unimplemented!()
    }
}
impl min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
}
pub trait min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<TrivialMoveOnly> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<TrivialMoveOnly> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<TrivialMoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_10(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_11(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_12(
        &self,
        mut _a0: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()>;
    fn operator_arrow(&self) -> Ptr<()>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
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
    fn operator_deref(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_arrow(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(&self, mut n: ()) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl
{
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_9 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : ()  , ) -> Ptr< min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : ()  , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__> {
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
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
}
