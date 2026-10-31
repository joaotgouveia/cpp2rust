extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn is_contiguous_container_asan_correct_0(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_1(_a0: Ptr<Vec<Vec<i32>>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_2(_a0: Ptr<Vec<NonTriviallyRelocatable>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_3(_a0: Ptr<Vec<Vec<NonTriviallyRelocatable>>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_4(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_5(_a0: Ptr<Vec<Vec<i32>>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_6(_a0: Ptr<Vec<NonTriviallyRelocatable>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_7(_a0: Ptr<Vec<Vec<NonTriviallyRelocatable>>>) -> bool {
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Tracker {
    #[offset(0)]
    pub copy_assignments: i32,
    #[offset(4)]
    pub move_assignments: i32,
}
impl Default for Tracker {
    fn default() -> Self {
        {
            Tracker {
                copy_assignments: 0,
                move_assignments: 0,
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
pub fn operator_eq_8(a: Ptr<NonTriviallyRelocatable>, b: Ptr<NonTriviallyRelocatable>) -> bool {
    return ({ a.with(|__s| __s.value_) } == { b.with(|__s| __s.value_) });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign)]
#[byte_size(4)]
pub struct NonTriviallyRelocatable {
    #[offset(0)]
    pub value_: i32,
}
impl NonTriviallyRelocatable {
    pub fn new_1() -> Self {
        Self { value_: 0 }
    }
    pub fn new_2(mut v: i32) -> Self {
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
impl CppEq for NonTriviallyRelocatable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
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
const _: () = assert!(true, "");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!(
            (alloc_count_10.with(|rc| *rc.borrow()) >= dealloc_count_11.with(|rc| *rc.borrow()))
        );
        return ((alloc_count_10.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_11.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_12.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_9.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_10.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_11.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_9: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_10: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_11: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_12: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_15(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_16(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_15(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<i32>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppLess for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_14(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_15(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_17(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_18(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_19(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_20(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
#[byte_size(8)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<i32>,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
impl CppLess for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_18(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_19(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub fn operator_sub_21(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_22(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_23(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_22(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Vec<i32>>,
}
impl min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ {
    fn new_1(mut p: Ptr<Vec<i32>>) -> Self {
        Self { ptr_: (p).clone() }
    }
}
impl CppEq
    for min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_22(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance
    for min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
pub fn operator_sub_24(
    mut x : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
#[byte_size(8)]
pub struct min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Vec<i32>>,
}
impl
    min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    pub fn new_1(
        mut p : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        Self {
            ptr_: (p.ptr_).clone(),
        }
    }
}
pub fn operator_sub_25(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_26(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_27(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_28(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_27(_x, y)
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
}
impl CppLess for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_26(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_27(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
pub fn operator_sub_29(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_30(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
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
impl CppEq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_30(__lhs.read(), __rhs.read())
    }
}
pub fn operator_sub_31(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_32(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_33(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_34(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_33(_x, y)
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
}
impl CppLess for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_32(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_33(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
pub fn operator_sub_35(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_36(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
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
impl CppEq for min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_36(__lhs.read(), __rhs.read())
    }
}
pub fn operator_sub_37(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_38(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_39(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_40(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_39(_x, y)
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
}
impl CppLess for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_38(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_39(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
}
pub fn operator_sub_41(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_42(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_43(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_44(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } != { (y.ptr_).clone() });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(
    min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
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
impl CppLess
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_42(__lhs.read(), __rhs.read())
    }
}
impl CppEq for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_43(__lhs.read(), __rhs.read())
    }
}
impl CppAdvance
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
pub fn operator_sub_45(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_46(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_47(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__   = x   ;
        operator_eq_46(_x, y)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Vec<NonTriviallyRelocatable>>,
}
impl min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { fn new_1 ( mut p : Ptr< Vec<NonTriviallyRelocatable  >  >  , ) -> Self { Self { ptr_ : (p ).clone() , }  } }
impl CppEq for min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool { operator_eq_46(__lhs.read(), __rhs.read()) } }
impl CppAdvance for min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { fn cpp_inc(__it: &Ptr<Self>) { __it.operator_inc_8(); } }
pub fn operator_sub_48(
    mut x : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  , new_1, value)]
#[byte_size(8)]
pub struct min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__
{
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<Vec<NonTriviallyRelocatable>>,
}
impl min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { pub fn new_1 ( mut p : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  , ) -> Self { Self { ptr_ : (p . ptr_  ).clone() , }  } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<u64>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_vector_int__min_allocator_int___ {}
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
pub struct min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
pub fn operator_eq_49(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_50(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_51(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_52(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_53(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_54(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_55(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    data_: i32,
}
impl MoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(x: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            data_: x.with(|__s| __s.data_),
        };
        field!(x, data_).write(0);
        __this
    }
}
impl CppLess for MoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_51(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_49(__lhs.clone(), __rhs.clone())
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_56(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_57(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_58(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_59(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_60(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_61(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_62(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
            return (*cmp.borrow());
        }
    }
    return std::cmp::Ordering::Equal;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, CppFrom, MoveAssign)]
#[cpp_from(i32, new, some_value)]
#[byte_size(4)]
pub struct TrivialMoveOnly {
    #[offset(0)]
    data_: i32,
}
impl TrivialMoveOnly {
    pub fn new(data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| 1);
        Self { data_: data }
    }
    pub fn move_from(_a0: Ptr<TrivialMoveOnly>) -> Self {
        Self {
            data_: { (*_a0.upgrade().deref()).data_ },
        }
    }
}
impl CppLess for TrivialMoveOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_58(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_56(__lhs.clone(), __rhs.clone())
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
pub fn tests_63() {
    {
        let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
        {
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!((*v.borrow()).is_empty());
                assert!((*i.borrow()) == v.decay().end());
                assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let last: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
            }
        }
        {
            {
                {
                    let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<i32>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<i32>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<i32>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(((*v.borrow()).len() == 0_usize));
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
                }
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(((*v.borrow()).len() == 2_usize));
                assert!(
                    ((v.decay().offset(0_usize as isize).read()) == (*arr.borrow())[(0) as usize])
                );
                assert!(
                    ((v.decay().offset(1_usize as isize).read()) == (*arr.borrow())[(2) as usize])
                );
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
            }
        }
    }
    {
        let outer: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(vec![
            (0..(1_usize) as usize)
                .map(|_| 0_i32)
                .collect::<Vec<_>>();
            2_usize as usize
        ]));
        unimplemented!();
        assert!(((*outer.borrow()).len() == 2_usize));
        assert!(((*outer.decay().offset(0_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(((*outer.decay().offset(1_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(({ is_contiguous_container_asan_correct_1(outer.as_pointer(),) }));
        assert!(
            ({ is_contiguous_container_asan_correct_0(outer.decay().offset(0_usize as isize),) })
        );
        assert!(
            ({ is_contiguous_container_asan_correct_0(outer.decay().offset(1_usize as isize),) })
        );
    }
    {
        {
            let v: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = MoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_49(_x, _y.as_pointer())
                })
            );
        }
        {
            let v: Value<Vec<TrivialMoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_56(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_64() {
    {
        let arr: Value<Box<[NonTriviallyRelocatable]>> = Rc::new(RefCell::new(Box::new([
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
        ])));
        {
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(Vec::new()));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!((*v.borrow()).is_empty());
                assert!((*i.borrow()) == v.decay().end());
                assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let last: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
            }
        }
        {
            {
                {
                    let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it
                            != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>)
                                .offset((3) as isize)
                        {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it
                            != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>)
                                .offset((3) as isize)
                        {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it
                            != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>)
                                .offset((3) as isize)
                        {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(((*v.borrow()).len() == 0_usize));
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
                }
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(((*v.borrow()).len() == 2_usize));
                assert!(
                    ({
                        let _a: Ptr<NonTriviallyRelocatable> = v.decay().offset(0_usize as isize);
                        operator_eq_8(
                            _a,
                            (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset(0),
                        )
                    })
                );
                assert!(
                    ({
                        let _a: Ptr<NonTriviallyRelocatable> = v.decay().offset(1_usize as isize);
                        operator_eq_8(
                            _a,
                            (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset(2),
                        )
                    })
                );
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
            }
        }
    }
    {
        let outer: Value<Vec<Vec<NonTriviallyRelocatable>>> =
            Rc::new(RefCell::new(vec![
                (0..(1_usize) as usize)
                    .map(|_| <NonTriviallyRelocatable>::default())
                    .collect::<Vec<_>>();
                2_usize as usize
            ]));
        unimplemented!();
        assert!(((*outer.borrow()).len() == 2_usize));
        assert!(((*outer.decay().offset(0_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(((*outer.decay().offset(1_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(({ is_contiguous_container_asan_correct_3(outer.as_pointer(),) }));
        assert!(
            ({ is_contiguous_container_asan_correct_2(outer.decay().offset(0_usize as isize),) })
        );
        assert!(
            ({ is_contiguous_container_asan_correct_2(outer.decay().offset(1_usize as isize),) })
        );
    }
    {
        {
            let v: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = MoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_49(_x, _y.as_pointer())
                })
            );
        }
        {
            let v: Value<Vec<TrivialMoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_56(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_65() {
    {
        let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3])));
        {
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!((*v.borrow()).is_empty());
                assert!((*i.borrow()) == v.decay().end());
                assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let last: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<i32>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
            }
        }
        {
            {
                {
                    let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<i32>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<i32>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<i32>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(((*v.borrow()).len() == 0_usize));
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
                }
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(((*v.borrow()).len() == 2_usize));
                assert!(
                    ((v.decay().offset(0_usize as isize).read()) == (*arr.borrow())[(0) as usize])
                );
                assert!(
                    ((v.decay().offset(1_usize as isize).read()) == (*arr.borrow())[(2) as usize])
                );
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<i32>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<i32>);
                    while __it != (arr.as_pointer() as Ptr<i32>).offset((3) as isize) {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_4(v.as_pointer(),) }));
            }
        }
    }
    {
        let outer: Value<Vec<Vec<i32>>> = Rc::new(RefCell::new(vec![
            (0..(1_usize) as usize)
                .map(|_| 0_i32)
                .collect::<Vec<_>>();
            2_usize as usize
        ]));
        unimplemented!();
        assert!(((*outer.borrow()).len() == 2_usize));
        assert!(((*outer.decay().offset(0_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(((*outer.decay().offset(1_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(({ is_contiguous_container_asan_correct_5(outer.as_pointer(),) }));
        assert!(
            ({ is_contiguous_container_asan_correct_4(outer.decay().offset(0_usize as isize),) })
        );
        assert!(
            ({ is_contiguous_container_asan_correct_4(outer.decay().offset(1_usize as isize),) })
        );
    }
    {
        {
            let v: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = MoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_49(_x, _y.as_pointer())
                })
            );
        }
        {
            let v: Value<Vec<TrivialMoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_56(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_66() {
    {
        let arr: Value<Box<[NonTriviallyRelocatable]>> = Rc::new(RefCell::new(Box::new([
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
        ])));
        {
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(Vec::new()));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!((*v.borrow()).is_empty());
                assert!((*i.borrow()) == v.decay().end());
                assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let last: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!().iter_add(1_isize)));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let first: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let last: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == (*last.borrow()));
                assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
            }
        }
        {
            {
                {
                    let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it
                            != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>)
                                .offset((3) as isize)
                        {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it
                            != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>)
                                .offset((3) as isize)
                        {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(unimplemented!());
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
                }
                {
                    let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it
                            != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>)
                                .offset((3) as isize)
                        {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }));
                    let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                        Rc::new(RefCell::new(unimplemented!()));
                    assert!(((*v.borrow()).len() == 0_usize));
                    assert!((*i.borrow()) == v.decay().begin());
                    assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
                }
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(((*v.borrow()).len() == 2_usize));
                assert!(
                    ({
                        let _a: Ptr<NonTriviallyRelocatable> = v.decay().offset(0_usize as isize);
                        operator_eq_8(
                            _a,
                            (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset(0),
                        )
                    })
                );
                assert!(
                    ({
                        let _a: Ptr<NonTriviallyRelocatable> = v.decay().offset(1_usize as isize);
                        operator_eq_8(
                            _a,
                            (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset(2),
                        )
                    })
                );
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
            }
            {
                let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new({
                    let mut __v = Vec::new();
                    let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                    while __it
                        != (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((3) as isize)
                    {
                        __v.push(<NonTriviallyRelocatable as CppFrom<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                }));
                let i: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                    Rc::new(RefCell::new(unimplemented!()));
                assert!(unimplemented!());
                assert!((*i.borrow()) == v.decay().begin().iter_add(1_isize));
                assert!(({ is_contiguous_container_asan_correct_6(v.as_pointer(),) }));
            }
        }
    }
    {
        let outer: Value<Vec<Vec<NonTriviallyRelocatable>>> =
            Rc::new(RefCell::new(vec![
                (0..(1_usize) as usize)
                    .map(|_| <NonTriviallyRelocatable>::default())
                    .collect::<Vec<_>>();
                2_usize as usize
            ]));
        unimplemented!();
        assert!(((*outer.borrow()).len() == 2_usize));
        assert!(((*outer.decay().offset(0_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(((*outer.decay().offset(1_usize as isize).upgrade().deref()).len() == 1_usize));
        assert!(({ is_contiguous_container_asan_correct_7(outer.as_pointer(),) }));
        assert!(
            ({ is_contiguous_container_asan_correct_6(outer.decay().offset(0_usize as isize),) })
        );
        assert!(
            ({ is_contiguous_container_asan_correct_6(outer.decay().offset(1_usize as isize),) })
        );
    }
    {
        {
            let v: Value<Vec<MoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = MoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_49(_x, _y.as_pointer())
                })
            );
        }
        {
            let v: Value<Vec<TrivialMoveOnly>> = Rc::new(RefCell::new(Vec::new()));
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                (*v.borrow_mut()).push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                (*v.borrow_mut()).push(__init)
            };
            unimplemented!();
            assert!(((*v.borrow()).len() == 1_usize));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_56(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_67() -> bool {
    ({ tests_63() });
    ({ tests_64() });
    ({ tests_65() });
    ({ tests_66() });
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
    ({ tests_67() });
    const _: () = assert!(true, "tests()");;
    {
        let v: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new(Vec::new()));
        let mut short_string: Vec<i8> = {
            let mut __bytes = Ptr::<i8>::from_string_literal(b"short").to_c_bytes();
            __bytes.push(0_i8);
            __bytes
        };
        let mut long_string: Vec<i8> = vec![('x' as i8); (256_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i8))
            .collect();
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            {
                let a0_clone = if ((i % 2) == 0) {
                    (short_string).clone()
                } else {
                    (long_string).clone()
                }
                .clone();
                (*v.borrow_mut()).push(a0_clone)
            };
            i.prefix_inc();
        }
        let original: Value<Vec<Vec<i8>>> = Rc::new(RefCell::new((*v.borrow()).clone()));
        let it: Value<RandomAccessIterator<Vec<i8>>> = Rc::new(RefCell::new(unimplemented!()));
        assert!(((*v.borrow()).len() == 4_usize));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!((*it.borrow()) == v.decay().begin().iter_add(2_isize));
    }
    return 0;
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<MoveOnly>) -> MoveOnly;
    fn operator_mul(&self, x: Ptr<MoveOnly>) -> MoveOnly;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, x: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        field!((*self), data_).write({ x.with(|__s| __s.data_) });
        field!(x, data_).write(0);
        return (*self).clone();
    }
    fn get(&self) -> i32 {
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
    fn get(&self) -> i32;
    fn operator_add(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
    fn operator_mul(&self, x: Ptr<TrivialMoveOnly>) -> TrivialMoveOnly;
}
impl TrivialMoveOnlyImpl for Ptr<TrivialMoveOnly> {
    fn move_assign(&self, _a0: Ptr<TrivialMoveOnly>) -> Ptr<TrivialMoveOnly> {
        field!((*self), data_).write({ { (*_a0.upgrade().deref()).data_ } });
        return (*self).clone();
    }
    fn get(&self) -> i32 {
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
    fn allocate(&self, n: usize) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_MoveOnly_Impl for Ptr<min_allocator_MoveOnly_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_NonTriviallyRelocatable_Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_NonTriviallyRelocatable_Impl for Ptr<min_allocator_NonTriviallyRelocatable_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_TrivialMoveOnly_Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_TrivialMoveOnly_Impl for Ptr<min_allocator_TrivialMoveOnly_> {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
    fn deallocate(&self, p: min_pointer_int__std_integral_constant_unsigned_long__0__, n: usize);
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___Impl
{
    fn allocate ( &self , n : usize  , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  ;
    fn deallocate(
        &self,
        p : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___Impl
    for Ptr<
        min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___,
    >
{
    fn allocate ( &self , mut n : usize  , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        return min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_std_vector_int__min_allocator_int___Impl {
    fn allocate(
        &self,
        n: usize,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__;
    fn deallocate(
        &self,
        p : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
        n: usize,
    );
}
impl min_allocator_std_vector_int__min_allocator_int___Impl
    for Ptr<min_allocator_std_vector_int__min_allocator_int___>
{
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        return min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()   } , )    ;
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub trait min_allocator_unsigned_long_Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_unsigned_long_Impl for Ptr<min_allocator_unsigned_long_> {}
pub trait min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<MoveOnly>;
    fn operator_arrow(&self) -> Ptr<MoveOnly>;
    fn operator_inc_8(&self)
    -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, mut n: isize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>
{
    fn to_bool(&self) -> bool {
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
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
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
        mut n: isize,
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
}
pub trait min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<NonTriviallyRelocatable>;
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<NonTriviallyRelocatable> {
        unimplemented!()
    }
}
impl min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
{
    fn to_bool(&self) -> bool {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<NonTriviallyRelocatable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
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
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<TrivialMoveOnly>;
    fn operator_arrow(&self) -> Ptr<TrivialMoveOnly>;
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        n: isize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__;
    fn operator_index(&self, mut n: isize) -> Ptr<TrivialMoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>
{
    fn to_bool(&self) -> bool {
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
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() - n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
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
        mut n: isize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl :: operator_sub_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
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
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<NonTriviallyRelocatable>;
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<NonTriviallyRelocatable> {
        unimplemented!()
    }
}
impl min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<NonTriviallyRelocatable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>
    {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        > = Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
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
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<TrivialMoveOnly> {
        unimplemented!()
    }
}
impl min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_10(
        &self,
        mut _a0: i32,
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
        mut _a0: i32,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<i32> {
        unimplemented!()
    }
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__>
{
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_9(
        &self,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_const_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let tmp: Value<min_pointer_const_int__std_integral_constant_unsigned_long__0__> =
            Rc::new(RefCell::new((*(*self).upgrade().deref()).clone()));
        ({
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                &tmp.as_pointer(),
                n,
            )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Vec<NonTriviallyRelocatable>> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<Vec<NonTriviallyRelocatable>> {
        unimplemented!()
    }    fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_10 ( &self , mut _a0 : i32  , ) -> min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_12 ( &self , mut _a0 : i32  , ) -> min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : isize  , ) -> min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<NonTriviallyRelocatable>> {
        unimplemented!()
    }
}
impl min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<Vec<i32>> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<Vec<i32>> {
        unimplemented!()
    }    fn operator_inc_9 ( &self , ) -> Ptr< min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_inc_10 ( &self , mut _a0 : i32  , ) -> min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_11 ( &self , ) -> Ptr< min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_12 ( &self , mut _a0 : i32  , ) -> min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_add ( &self , mut n : isize  , ) -> min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<i32>> {
        unimplemented!()
    }
}
impl min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__> {
}
pub trait min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_const_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_int__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<i32>;
    fn operator_arrow(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
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
        n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(&self, n: isize) -> min_pointer_int__std_integral_constant_unsigned_long__0__;
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
    fn to_bool(&self) -> bool {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
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
pub trait min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<Vec<NonTriviallyRelocatable>>;
    fn operator_arrow(&self) -> Ptr<Vec<NonTriviallyRelocatable>> {
        unimplemented!()
    }
    fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  > ;
    fn operator_post_inc_9 ( &self , mut _a0 : i32  , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }    fn operator_dec_10 ( &self , ) -> Ptr< min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }    fn operator_post_dec_11 ( &self , mut _a0 : i32  , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_add_assign ( &self , n : isize  , ) -> Ptr< min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  > ;
    fn operator_sub_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  >{
        unimplemented!()
    }
    fn operator_add ( &self , n : isize  , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  ;
    fn operator_sub ( &self , mut n : isize  , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<NonTriviallyRelocatable>> {
        unimplemented!()
    }
}
impl min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__Impl for Ptr<min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__> {
fn to_bool ( &self , ) -> bool  { return ! ( ( (* self ) .with(|__s| __s . ptr_ .clone()) ).is_null() )   ;
 } fn operator_deref ( &self , ) -> Ptr< Vec<NonTriviallyRelocatable  >  > { return (* self ) .with(|__s| __s . ptr_ .clone())    ;
 } fn operator_inc_8 ( &self , ) -> Ptr< min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  > { field!((* self ) , ptr_) .with_mut(|__v| __v. prefix_inc ()) ;
 return ((* self )  ).clone()  ;
 } fn operator_add_assign ( &self , mut n : isize  , ) -> Ptr< min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  > { { let _ptr = field!((* self ) , ptr_); _ptr.write(_ptr.read() + n ) } ;
 return ((* self )  ).clone()  ;
 } fn operator_add ( &self , mut n : isize  , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  { let tmp : Value<min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
  ;
 ;
 ( { min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , ) } )  ;
 return ((*tmp.borrow()) ).clone()   ;
 } }
pub trait min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__Impl
{
    fn to_bool(&self) -> bool;
    fn operator_deref(&self) -> Ptr<Vec<i32>>;
    fn operator_arrow(&self) -> Ptr<Vec<i32>> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<
        min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    >;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<
        min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    > {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        n: isize,
    ) -> Ptr<
        min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    >;
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<
        min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    > {
        unimplemented!()
    }
    fn operator_add(
        &self,
        n: isize,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__;
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<Vec<i32>> {
        unimplemented!()
    }
}
impl min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__Impl
    for Ptr<
        min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    >
{
    fn to_bool(&self) -> bool {
        return !(((*self).with(|__s| __s.ptr_.clone())).is_null());
    }
    fn operator_deref(&self) -> Ptr<Vec<i32>> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<
        min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    > {
        field!((*self), ptr_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<
        min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    > {
        {
            let _ptr = field!((*self), ptr_);
            _ptr.write(_ptr.read() + n)
        };
        return (*self).clone();
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        let tmp : Value<min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ > = Rc::new(RefCell::new((( * (* self ) .upgrade().deref() )  ).clone() )) ;
        ({
            min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__Impl :: operator_add_assign ( &tmp .as_pointer()  , n  , )
        });
        return (*tmp.borrow()).clone();
    }
}
pub trait min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<u64> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<u64> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<u64> {
        unimplemented!()
    }
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__>
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_9.with(|_| ());
    let _ = alloc_count_10.with(|_| ());
    let _ = dealloc_count_11.with(|_| ());
    let _ = disable_default_constructor_12.with(|_| ());
}
