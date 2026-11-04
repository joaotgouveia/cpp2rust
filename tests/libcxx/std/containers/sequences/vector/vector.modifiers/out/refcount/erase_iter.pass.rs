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
pub fn is_contiguous_container_asan_correct_1(_a0: Ptr<Vec<NonTriviallyRelocatable>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_2(_a0: Ptr<Vec<i32>>) -> bool {
    return true;
}
pub fn is_contiguous_container_asan_correct_3(_a0: Ptr<Vec<NonTriviallyRelocatable>>) -> bool {
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
impl CppMove for Tracker {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
impl CppMove for TrackedAssignment {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_4(a: Ptr<NonTriviallyRelocatable>, b: Ptr<NonTriviallyRelocatable>) -> bool {
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
        operator_eq_4(__lhs.clone(), __rhs.clone())
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
impl CppMove for NonTriviallyRelocatable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
        assert!((alloc_count_6.with(|rc| *rc.borrow()) >= dealloc_count_7.with(|rc| *rc.borrow())));
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_6: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_7: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_8: Value<bool> = Rc::new(RefCell::new(false));
);
pub fn operator_sub_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_11(_x, y)
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
impl std::cmp::Ord for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                if operator_lt_10(__lhs.read(), __rhs.read()) {
                    std::cmp::Ordering::Less
                } else if operator_lt_10(__rhs.read(), __lhs.read()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        }
    }
}
impl std::cmp::PartialOrd for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_11(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
    }
}
impl CppIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_inc_8(__it);
        });
        self
    }
}
impl CppInputIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl CppBidirectionalIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_dec_10(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_int__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(
                __it, n as _,
            );
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (operator_sub_9(__lhs.read(), __rhs.read())) as isize
        })
    }
}
impl CppMove for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_13(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_14(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_ne_15(
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
impl std::cmp::Ord for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                if operator_lt_14(__lhs.read(), __rhs.read()) {
                    std::cmp::Ordering::Less
                } else if operator_lt_14(__rhs.read(), __lhs.read()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        }
    }
}
impl std::cmp::PartialOrd for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                !(operator_ne_15(__lhs.read(), __rhs.read()))
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
impl CppIterator<i32> for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> Ptr<i32> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_deref(
                __it,
            )
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__Impl::operator_inc_9(
                __it,
            );
        });
        self
    }
}
impl CppInputIterator<i32> for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_16(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_17(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_18(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_17(_x, y)
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
impl std::cmp::PartialEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_17(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
    }
}
impl CppIterator<MoveOnly> for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> Ptr<MoveOnly> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_inc_8(
                __it,
            );
        });
        self
    }
}
impl CppInputIterator<MoveOnly> for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl CppBidirectionalIterator<MoveOnly>
    for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__
{
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_dec_10(
                __it,
            );
        });
        self
    }
}
impl CppMove for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_19(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_20(
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
        operator_eq_20(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_21(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_22(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_23(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_22(_x, y)
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
impl std::cmp::PartialEq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_22(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
impl CppAdvance for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
    }
}
impl CppIterator<TrivialMoveOnly>
    for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__
{
    fn iter_deref(&self) -> Ptr<TrivialMoveOnly> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_inc_8(__it);
        });
        self
    }
}
impl CppInputIterator<TrivialMoveOnly>
    for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__
{
}
impl CppBidirectionalIterator<TrivialMoveOnly>
    for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__
{
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__Impl::operator_dec_10(__it);
        });
        self
    }
}
impl CppMove for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_24(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_eq_25(
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
        operator_eq_25(__lhs.read(), __rhs.read())
    }
}
impl CppMove for min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_26(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_27(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_eq_28(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } == { (y.ptr_).clone() });
}
pub fn operator_ne_29(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !({
        let _x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_28(_x, y)
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
impl std::cmp::Ord
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                if operator_lt_27(__lhs.read(), __rhs.read()) {
                    std::cmp::Ordering::Less
                } else if operator_lt_27(__rhs.read(), __lhs.read()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        }
    }
}
impl std::cmp::PartialOrd
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                operator_eq_28(__lhs.read(), __rhs.read())
            })
        }
    }
}
impl std::cmp::Eq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
impl CppAdvance for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_8();
    }
    fn cpp_dec(__it: &Ptr<Self>) {
        __it.operator_dec_10();
    }
}
impl CppIterator<NonTriviallyRelocatable>
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn iter_deref(&self) -> Ptr<NonTriviallyRelocatable> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl::operator_inc_8(__it);
        });
        self
    }
}
impl CppInputIterator<NonTriviallyRelocatable>
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
impl CppBidirectionalIterator<NonTriviallyRelocatable>
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn iter_dec(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl::operator_dec_10(__it);
        });
        self
    }
}
impl CppRandomAccessIterator<NonTriviallyRelocatable>
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn iter_add_assign(&mut self, n: isize) -> Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl::operator_add_assign(__it, n as _);
        });
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        libcc2rs::with_record_ptrs(self, from, |__lhs, __rhs| {
            (operator_sub_26(__lhs.read(), __rhs.read())) as isize
        })
    }
}
impl CppMove for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_sub_30(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_).clone() - (y.ptr_).clone()) as i64 as isize);
}
pub fn operator_lt_31(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ({ (x.ptr_).clone() } < { (y.ptr_).clone() });
}
pub fn operator_ne_32(
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
impl std::cmp::Ord
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                if operator_lt_31(__lhs.read(), __rhs.read()) {
                    std::cmp::Ordering::Less
                } else if operator_lt_31(__rhs.read(), __lhs.read()) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            })
        }
    }
}
impl std::cmp::PartialOrd
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        {
            libcc2rs::with_record_ptrs(self, other, |__lhs, __rhs| {
                !(operator_ne_32(__lhs.read(), __rhs.read()))
            })
        }
    }
}
impl std::cmp::Eq
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
impl CppAdvance
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn cpp_inc(__it: &Ptr<Self>) {
        __it.operator_inc_9();
    }
}
impl CppIterator<NonTriviallyRelocatable>
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn iter_deref(&self) -> Ptr<NonTriviallyRelocatable> {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl::operator_deref(__it)
        })
    }
    fn iter_inc(&mut self) -> &mut Self {
        libcc2rs::with_record_ptr(self, |__it| {
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__Impl::operator_inc_9(__it);
        });
        self
    }
}
impl CppInputIterator<NonTriviallyRelocatable>
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
impl CppMove
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<u64>,
}
impl min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {}
impl CppMove for min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_MoveOnly_ {}
impl CppMove for min_allocator_MoveOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_TrivialMoveOnly_ {}
impl CppMove for min_allocator_TrivialMoveOnly_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_NonTriviallyRelocatable_ {}
impl CppMove for min_allocator_NonTriviallyRelocatable_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_unsigned_long_ {}
impl CppMove for min_allocator_unsigned_long_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn operator_eq_33(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_34(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_35(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_36(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_37(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_38(x: Ptr<MoveOnly>, y: Ptr<MoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_39(_a0: Ptr<MoveOnly>, _a1: Ptr<MoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
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
        operator_lt_35(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for MoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_33(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for MoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        { MoveOnly::new(None) }
    }
}
pub fn operator_eq_40(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_41(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } != { y.with(|__s| __s.data_) });
}
pub fn operator_lt_42(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
pub fn operator_le_43(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } <= { y.with(|__s| __s.data_) });
}
pub fn operator_gt_44(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } > { y.with(|__s| __s.data_) });
}
pub fn operator_ge_45(x: Ptr<TrivialMoveOnly>, y: Ptr<TrivialMoveOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } >= { y.with(|__s| __s.data_) });
}
pub fn operator_cmp_46(_a0: Ptr<TrivialMoveOnly>, _a1: Ptr<TrivialMoveOnly>) -> std::cmp::Ordering {
    {
        let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.data_)),
            &(_a1.with(|__s| __s.data_)),
        )));
        if !(({ (*cmp.borrow()) }) == std::cmp::Ordering::Equal) {
            return ({ (*cmp.borrow()) });
        }
    }
    return ({ (*libcc2rs::strong_ordering_equal().upgrade().deref()) });
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
        operator_lt_42(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for TrivialMoveOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_40(__lhs.clone(), __rhs.clone())
    }
}
impl CppMove for TrivialMoveOnly {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        { TrivialMoveOnly::new(None) }
    }
}
pub fn tests_47() {
    {
        let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        {
            let v: Value<Vec<i32>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<i32>).offset((5) as isize) },) {
                    (mut __tmp_0,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != __tmp_0 {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<i32>> =
                Rc::new(RefCell::new(match ({ unimplemented!() },) {
                    (mut __tmp_1,) => {
                        let __off = __tmp_1.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                        (__tmp_1).clone()
                    }
                }));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin());
            assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 4, 5])));
            let v: Value<Vec<i32>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<i32>).offset((5) as isize) },) {
                    (mut __tmp_2,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != __tmp_2 {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(1_isize) },) {
                    (mut __tmp_3,) => {
                        let __off = __tmp_3.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                        (__tmp_3).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin().iter_add(1_isize));
            assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
            let v: Value<Vec<i32>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<i32>).offset((5) as isize) },) {
                    (mut __tmp_4,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != __tmp_4 {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(4_isize) },) {
                    (mut __tmp_5,) => {
                        let __off = __tmp_5.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                        (__tmp_5).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().end());
            assert!(({ is_contiguous_container_asan_correct_0(v.as_pointer(),) }));
        }
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
            match ({ <RandomAccessIterator<MoveOnly>>::from(v.decay().begin()) },) {
                (mut __tmp_6,) => {
                    let __off = __tmp_6.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<MoveOnly>| __v.remove(__off));
                    (__tmp_6).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                    operator_eq_33(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_33(_x, _y.as_pointer())
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
            match ({ <RandomAccessIterator<TrivialMoveOnly>>::from(v.decay().begin()) },) {
                (mut __tmp_7,) => {
                    let __off = __tmp_7.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<TrivialMoveOnly>| __v.remove(__off));
                    (__tmp_7).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(2) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_48() {
    {
        let arr: Value<Box<[NonTriviallyRelocatable]>> = Rc::new(RefCell::new(Box::new([
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
            NonTriviallyRelocatable::new_2({ 4 }),
            NonTriviallyRelocatable::new_2({ 5 }),
        ])));
        {
            let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((5) as isize) },)
                {
                    (mut __tmp_8,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it != __tmp_8 {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                Rc::new(RefCell::new(match ({ unimplemented!() },) {
                    (mut __tmp_9,) => {
                        let __off = __tmp_9.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<NonTriviallyRelocatable>| __v.remove(__off));
                        (__tmp_9).clone()
                    }
                }));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin());
            assert!(({ is_contiguous_container_asan_correct_1(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1 }),
                    NonTriviallyRelocatable::new_2({ 3 }),
                    NonTriviallyRelocatable::new_2({ 4 }),
                    NonTriviallyRelocatable::new_2({ 5 }),
                ])));
            let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((5) as isize) },)
                {
                    (mut __tmp_10,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it != __tmp_10 {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(1_isize) },) {
                    (mut __tmp_11,) => {
                        let __off = __tmp_11.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<NonTriviallyRelocatable>| __v.remove(__off));
                        (__tmp_11).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin().iter_add(1_isize));
            assert!(({ is_contiguous_container_asan_correct_1(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1 }),
                    NonTriviallyRelocatable::new_2({ 2 }),
                    NonTriviallyRelocatable::new_2({ 3 }),
                    NonTriviallyRelocatable::new_2({ 4 }),
                ])));
            let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((5) as isize) },)
                {
                    (mut __tmp_12,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it != __tmp_12 {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(4_isize) },) {
                    (mut __tmp_13,) => {
                        let __off = __tmp_13.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<NonTriviallyRelocatable>| __v.remove(__off));
                        (__tmp_13).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().end());
            assert!(({ is_contiguous_container_asan_correct_1(v.as_pointer(),) }));
        }
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
            match ({ <RandomAccessIterator<MoveOnly>>::from(v.decay().begin()) },) {
                (mut __tmp_14,) => {
                    let __off = __tmp_14.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<MoveOnly>| __v.remove(__off));
                    (__tmp_14).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                    operator_eq_33(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_33(_x, _y.as_pointer())
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
            match ({ <RandomAccessIterator<TrivialMoveOnly>>::from(v.decay().begin()) },) {
                (mut __tmp_15,) => {
                    let __off = __tmp_15.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<TrivialMoveOnly>| __v.remove(__off));
                    (__tmp_15).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(2) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_49() {
    {
        let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4, 5])));
        {
            let v: Value<Vec<i32>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<i32>).offset((5) as isize) },) {
                    (mut __tmp_16,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != __tmp_16 {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<i32>> =
                Rc::new(RefCell::new(match ({ unimplemented!() },) {
                    (mut __tmp_17,) => {
                        let __off = __tmp_17.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                        (__tmp_17).clone()
                    }
                }));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin());
            assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 3, 4, 5])));
            let v: Value<Vec<i32>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<i32>).offset((5) as isize) },) {
                    (mut __tmp_18,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != __tmp_18 {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(1_isize) },) {
                    (mut __tmp_19,) => {
                        let __off = __tmp_19.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                        (__tmp_19).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin().iter_add(1_isize));
            assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 3, 4])));
            let v: Value<Vec<i32>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<i32>).offset((5) as isize) },) {
                    (mut __tmp_20,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<i32>);
                        while __it != __tmp_20 {
                            __v.push((__it.iter_deref().read()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(4_isize) },) {
                    (mut __tmp_21,) => {
                        let __off = __tmp_21.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<i32>| __v.remove(__off));
                        (__tmp_21).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().end());
            assert!(({ is_contiguous_container_asan_correct_2(v.as_pointer(),) }));
        }
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
            match ({ <()>::from(v.decay().begin()) },) {
                (mut __tmp_22,) => {
                    let __off = __tmp_22.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<MoveOnly>| __v.remove(__off));
                    (__tmp_22).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                    operator_eq_33(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_33(_x, _y.as_pointer())
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
            match ({ <()>::from(v.decay().begin()) },) {
                (mut __tmp_23,) => {
                    let __off = __tmp_23.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<TrivialMoveOnly>| __v.remove(__off));
                    (__tmp_23).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(2) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_50() {
    {
        let arr: Value<Box<[NonTriviallyRelocatable]>> = Rc::new(RefCell::new(Box::new([
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
            NonTriviallyRelocatable::new_2({ 4 }),
            NonTriviallyRelocatable::new_2({ 5 }),
        ])));
        {
            let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((5) as isize) },)
                {
                    (mut __tmp_24,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it != __tmp_24 {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<NonTriviallyRelocatable>> =
                Rc::new(RefCell::new(match ({ unimplemented!() },) {
                    (mut __tmp_25,) => {
                        let __off = __tmp_25.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<NonTriviallyRelocatable>| __v.remove(__off));
                        (__tmp_25).clone()
                    }
                }));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin());
            assert!(({ is_contiguous_container_asan_correct_3(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1 }),
                    NonTriviallyRelocatable::new_2({ 3 }),
                    NonTriviallyRelocatable::new_2({ 4 }),
                    NonTriviallyRelocatable::new_2({ 5 }),
                ])));
            let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((5) as isize) },)
                {
                    (mut __tmp_26,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it != __tmp_26 {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(1_isize) },) {
                    (mut __tmp_27,) => {
                        let __off = __tmp_27.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<NonTriviallyRelocatable>| __v.remove(__off));
                        (__tmp_27).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().begin().iter_add(1_isize));
            assert!(({ is_contiguous_container_asan_correct_3(v.as_pointer(),) }));
        }
        {
            let expected: Value<Box<[NonTriviallyRelocatable]>> =
                Rc::new(RefCell::new(Box::new([
                    NonTriviallyRelocatable::new_2({ 1 }),
                    NonTriviallyRelocatable::new_2({ 2 }),
                    NonTriviallyRelocatable::new_2({ 3 }),
                    NonTriviallyRelocatable::new_2({ 4 }),
                ])));
            let v: Value<Vec<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ (arr.as_pointer() as Ptr<NonTriviallyRelocatable>).offset((5) as isize) },)
                {
                    (mut __tmp_28,) => {
                        let mut __v = Vec::new();
                        let mut __it = (arr.as_pointer() as Ptr<NonTriviallyRelocatable>);
                        while __it != __tmp_28 {
                            __v.push(<NonTriviallyRelocatable as CppFrom<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    }
                },
            ));
            let it: Value<RandomAccessIterator<NonTriviallyRelocatable>> = Rc::new(RefCell::new(
                match ({ unimplemented!().iter_add(4_isize) },) {
                    (mut __tmp_29,) => {
                        let __off = __tmp_29.index();
                        v.as_pointer()
                            .with_mut(|__v: &mut Vec<NonTriviallyRelocatable>| __v.remove(__off));
                        (__tmp_29).clone()
                    }
                },
            ));
            assert!(unimplemented!());
            assert!((*it.borrow()) == v.decay().end());
            assert!(({ is_contiguous_container_asan_correct_3(v.as_pointer(),) }));
        }
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
            match ({ <()>::from(v.decay().begin()) },) {
                (mut __tmp_30,) => {
                    let __off = __tmp_30.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<MoveOnly>| __v.remove(__off));
                    (__tmp_30).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(2) })));
                    operator_eq_33(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<MoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ Some(3) })));
                    operator_eq_33(_x, _y.as_pointer())
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
            match ({ <()>::from(v.decay().begin()) },) {
                (mut __tmp_31,) => {
                    let __off = __tmp_31.index();
                    v.as_pointer()
                        .with_mut(|__v: &mut Vec<TrivialMoveOnly>| __v.remove(__off));
                    (__tmp_31).clone()
                }
            };
            assert!(({ ((*v.borrow()).len() == 2_usize) }));
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(0_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(2) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
            assert!(
                ({
                    let _x: Ptr<TrivialMoveOnly> = v.decay().offset(1_usize as isize);
                    let _y: Value<TrivialMoveOnly> =
                        Rc::new(RefCell::new(TrivialMoveOnly::new({ Some(3) })));
                    operator_eq_40(_x, _y.as_pointer())
                })
            );
        }
    }
}
pub fn tests_51() -> bool {
    ({ tests_47() });
    ({ tests_48() });
    ({ tests_49() });
    ({ tests_50() });
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
    ({ tests_51() });
    const _: () = assert!(true, "tests()");;
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
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>;
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
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
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
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable>;
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
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__>;
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
    fn operator_arrow(&self) -> Ptr<NonTriviallyRelocatable> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
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
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__>;
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
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
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
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
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
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
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
    fn operator_arrow(&self) -> Ptr<i32>;
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__>;
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
    fn operator_arrow(&self) -> Ptr<i32> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn operator_inc_8(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_inc();
        });
        return (*self).clone();
    }
    fn operator_dec_10(&self) -> Ptr<min_pointer_int__std_integral_constant_unsigned_long__0__> {
        field!((*self), ptr_).with_mut(|__v| {
            __v.prefix_dec();
        });
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
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
}
