extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg)]
pub struct __gnu_test_func_callback {
    _M_size: (),
    _M_tests: [Option<unsafe fn()>; 15],
}
impl __gnu_test_func_callback {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            _M_size: 0_(),
            _M_tests: [None; 15],
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return self._M_size;
    }
    pub unsafe fn tests(&self) -> *const Option<unsafe fn()> {
        return self._M_tests.as_ptr();
    }
    pub unsafe fn push_back(&mut self, mut test: Option<unsafe fn()>) {
        self._M_tests[(self._M_size) as usize] = test;
        self._M_size.prefix_inc();
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        unsafe { __gnu_test_func_callback::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const __gnu_test_object_counter) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_test_object_counter,
    ) -> *mut __gnu_test_object_counter {
        return &mut (*(self as *mut __gnu_test_object_counter));
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_object_counter::copy_from(self as *const __gnu_test_object_counter) }
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        unsafe { __gnu_test_object_counter::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__1))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)))
        {
            unimplemented!();
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__3))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)))
        {
            unimplemented!();
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)) = 0_();
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_copy_tracker {
    id_: (),
    throw_on_copy_: (),
}
impl __gnu_test_copy_tracker {
    pub unsafe fn new(mut id: Option<()>, mut throw_on_copy: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__6)).postfix_dec()
        });
        let mut throw_on_copy: () = throw_on_copy.unwrap_or_else(|| unsafe { false });
        let mut this = Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const __gnu_test_copy_tracker) -> Self {
        let mut this = Self {
            id_: (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }),
            throw_on_copy_: (*rhs).throw_on_copy_,
        };
        if this.throw_on_copy_ {
            (unsafe {
                __gnu_test_copy_constructor::throw_on(
                    (unsafe { __gnu_test_copy_constructor::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_copy_constructor::mark_call() });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rhs: *const __gnu_test_copy_tracker,
    ) -> *mut __gnu_test_copy_tracker {
        self.id_ = (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) });
        if (*rhs).throw_on_copy_ {
            (unsafe {
                __gnu_test_assignment_operator::throw_on(
                    (unsafe { __gnu_test_assignment_operator::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_assignment_operator::mark_call() });
        return &mut (*(self as *mut __gnu_test_copy_tracker));
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { __gnu_test_destructor::mark_call() });
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn reset() {
        (unsafe { __gnu_test_copy_constructor::reset() });
        (unsafe { __gnu_test_assignment_operator::reset() });
        (unsafe { __gnu_test_destructor::reset() });
    }
}
impl std::cmp::Ord for __gnu_test_copy_tracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_7(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(
                other as *const __gnu_test_copy_tracker,
                self as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_copy_tracker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_copy_tracker {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_8(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copy_tracker {}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_copy_tracker::copy_from(self as *const __gnu_test_copy_tracker) }
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        unsafe { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub unsafe fn operator_eq_8(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_7(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        < (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_semaphore {
    sem_set_: (),
    pid_: (),
}
impl __gnu_test_semaphore {}
pub static mut arr_9: std::cell::LazyCell<[(); 15]> =
    std::cell::LazyCell::new(|| unsafe { std::mem::transmute(*b"efgh\0abcd\0ijkl\0") });
pub static mut s1_10: std::cell::LazyCell<*const ()> = std::cell::LazyCell::new(|| unsafe {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut arr_9)).as_ptr()
});
pub static mut s2_11: std::cell::LazyCell<*const ()> = std::cell::LazyCell::new(|| unsafe {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut arr_9))
        .as_ptr()
        .offset((5_()) as isize)
});
pub static mut s3_12: std::cell::LazyCell<*const ()> = std::cell::LazyCell::new(|| unsafe {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut arr_9))
        .as_ptr()
        .offset((6_()) as isize)
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrNone {
    pub str: *const (),
}
impl CStrNone {
    pub unsafe fn to_pconst(&self) -> *const () {
        return self.str;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMem_ReturnOrd_std_strong_ordering__ {
    pub str: *const (),
}
impl CStrMem_ReturnOrd_std_strong_ordering__ {
    pub unsafe fn operator_cmp_2(&self, rhs: *const CStrMem_ReturnOrd_std_strong_ordering__) -> () {
        return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
    }
    pub unsafe fn operator_cmp_3(&self, mut rhs: *const ()) -> () {
        return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
    }
}
impl std::cmp::Ord for CStrMem_ReturnOrd_std_strong_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            CStrMem_ReturnOrd_std_strong_ordering__::operator_cmp_2(
                self,
                other as *const CStrMem_ReturnOrd_std_strong_ordering__,
            )
        }
    }
}
impl std::cmp::PartialOrd for CStrMem_ReturnOrd_std_strong_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrMem_ReturnOrd_std_strong_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CStrMem_ReturnOrd_std_strong_ordering__::operator_cmp_2(
                self,
                other as *const CStrMem_ReturnOrd_std_strong_ordering__,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for CStrMem_ReturnOrd_std_strong_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMem_ReturnOrd_std_weak_ordering__ {
    pub str: *const (),
}
impl CStrMem_ReturnOrd_std_weak_ordering__ {
    pub unsafe fn operator_cmp_2(&self, rhs: *const CStrMem_ReturnOrd_std_weak_ordering__) -> () {
        return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
    }
    pub unsafe fn operator_cmp_3(&self, mut rhs: *const ()) -> () {
        return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
    }
}
impl std::cmp::Ord for CStrMem_ReturnOrd_std_weak_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            CStrMem_ReturnOrd_std_weak_ordering__::operator_cmp_2(
                self,
                other as *const CStrMem_ReturnOrd_std_weak_ordering__,
            )
        }
    }
}
impl std::cmp::PartialOrd for CStrMem_ReturnOrd_std_weak_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrMem_ReturnOrd_std_weak_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CStrMem_ReturnOrd_std_weak_ordering__::operator_cmp_2(
                self,
                other as *const CStrMem_ReturnOrd_std_weak_ordering__,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for CStrMem_ReturnOrd_std_weak_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMem_ReturnOrd_std_partial_ordering__ {
    pub str: *const (),
}
impl CStrMem_ReturnOrd_std_partial_ordering__ {
    pub unsafe fn operator_cmp_2(
        &self,
        rhs: *const CStrMem_ReturnOrd_std_partial_ordering__,
    ) -> () {
        return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
    }
    pub unsafe fn operator_cmp_3(&self, mut rhs: *const ()) -> () {
        return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
    }
}
impl std::cmp::Ord for CStrMem_ReturnOrd_std_partial_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            CStrMem_ReturnOrd_std_partial_ordering__::operator_cmp_2(
                self,
                other as *const CStrMem_ReturnOrd_std_partial_ordering__,
            )
        }
    }
}
impl std::cmp::PartialOrd for CStrMem_ReturnOrd_std_partial_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrMem_ReturnOrd_std_partial_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CStrMem_ReturnOrd_std_partial_ordering__::operator_cmp_2(
                self,
                other as *const CStrMem_ReturnOrd_std_partial_ordering__,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for CStrMem_ReturnOrd_std_partial_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMem_ReturnInt_ {
    pub str: *const (),
}
impl CStrMem_ReturnInt_ {
    pub unsafe fn operator_cmp_2(&self, rhs: *const CStrMem_ReturnInt_) -> () {
        return (unsafe { ReturnInt::create(unimplemented!()) });
    }
    pub unsafe fn operator_cmp_3(&self, mut rhs: *const ()) -> () {
        return (unsafe { ReturnInt::create(unimplemented!()) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMem_CustomOrd_ {
    pub str: *const (),
}
impl CStrMem_CustomOrd_ {
    pub unsafe fn operator_cmp_2(&self, rhs: *const CStrMem_CustomOrd_) -> CustomOrd {
        return (unsafe { CustomOrd::create(unimplemented!()) });
    }
    pub unsafe fn operator_cmp_3(&self, mut rhs: *const ()) -> CustomOrd {
        return (unsafe { CustomOrd::create(unimplemented!()) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMem_ReturnVoid_ {
    pub str: *const (),
}
impl CStrMem_ReturnVoid_ {
    pub unsafe fn operator_cmp_2(&self, rhs: *const CStrMem_ReturnVoid_) {
        (unsafe { ReturnVoid::create(unimplemented!()) });
        return;
    }
    pub unsafe fn operator_cmp_3(&self, mut rhs: *const ()) {
        (unsafe { ReturnVoid::create(unimplemented!()) });
        return;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMem_NoOperators_ {
    pub str: *const (),
}
impl CStrMem_NoOperators_ {
    pub unsafe fn operator_cmp_2(&self, rhs: *const CStrMem_NoOperators_) -> NoOperators {
        return (unsafe { NoOperators::create(unimplemented!()) });
    }
    pub unsafe fn operator_cmp_3(&self, mut rhs: *const ()) -> NoOperators {
        return (unsafe { NoOperators::create(unimplemented!()) });
    }
}
pub unsafe fn operator_cmp_13(
    mut lhs: CStrFriend_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrFriend_ReturnOrd_std_strong_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_14(
    mut lhs: CStrFriend_ReturnOrd_std_strong_ordering__,
    mut rhs: *const (),
) -> () {
    return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFriend_ReturnOrd_std_strong_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrFriend_ReturnOrd_std_strong_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_13(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrFriend_ReturnOrd_std_strong_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrFriend_ReturnOrd_std_strong_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_13(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrFriend_ReturnOrd_std_strong_ordering__ {}
pub unsafe fn operator_cmp_15(
    mut lhs: CStrFriend_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrFriend_ReturnOrd_std_weak_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_16(
    mut lhs: CStrFriend_ReturnOrd_std_weak_ordering__,
    mut rhs: *const (),
) -> () {
    return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFriend_ReturnOrd_std_weak_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrFriend_ReturnOrd_std_weak_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrFriend_ReturnOrd_std_weak_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrFriend_ReturnOrd_std_weak_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_15(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrFriend_ReturnOrd_std_weak_ordering__ {}
pub unsafe fn operator_cmp_17(
    mut lhs: CStrFriend_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrFriend_ReturnOrd_std_partial_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_18(
    mut lhs: CStrFriend_ReturnOrd_std_partial_ordering__,
    mut rhs: *const (),
) -> () {
    return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFriend_ReturnOrd_std_partial_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrFriend_ReturnOrd_std_partial_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_17(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrFriend_ReturnOrd_std_partial_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrFriend_ReturnOrd_std_partial_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_17(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrFriend_ReturnOrd_std_partial_ordering__ {}
pub unsafe fn operator_cmp_19(
    mut lhs: CStrFriend_ReturnInt_,
    mut rhs: CStrFriend_ReturnInt_,
) -> () {
    return (unsafe { ReturnInt::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_20(mut lhs: CStrFriend_ReturnInt_, mut rhs: *const ()) -> () {
    return (unsafe { ReturnInt::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFriend_ReturnInt_ {
    pub str: *const (),
}
pub unsafe fn operator_cmp_21(
    mut lhs: CStrFriend_CustomOrd_,
    mut rhs: CStrFriend_CustomOrd_,
) -> CustomOrd {
    return (unsafe { CustomOrd::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_22(mut lhs: CStrFriend_CustomOrd_, mut rhs: *const ()) -> CustomOrd {
    return (unsafe { CustomOrd::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFriend_CustomOrd_ {
    pub str: *const (),
}
pub unsafe fn operator_cmp_23(mut lhs: CStrFriend_ReturnVoid_, mut rhs: CStrFriend_ReturnVoid_) {
    (unsafe { ReturnVoid::create(unimplemented!()) });
    return;
}
pub unsafe fn operator_cmp_24(mut lhs: CStrFriend_ReturnVoid_, mut rhs: *const ()) {
    (unsafe { ReturnVoid::create(unimplemented!()) });
    return;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFriend_ReturnVoid_ {
    pub str: *const (),
}
pub unsafe fn operator_cmp_25(
    mut lhs: CStrFriend_NoOperators_,
    mut rhs: CStrFriend_NoOperators_,
) -> NoOperators {
    return (unsafe { NoOperators::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_26(mut lhs: CStrFriend_NoOperators_, mut rhs: *const ()) -> NoOperators {
    return (unsafe { NoOperators::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFriend_NoOperators_ {
    pub str: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFree_ReturnOrd_std_strong_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrFree_ReturnOrd_std_strong_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrFree_ReturnOrd_std_strong_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrFree_ReturnOrd_std_strong_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_27(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrFree_ReturnOrd_std_strong_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFree_ReturnOrd_std_weak_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrFree_ReturnOrd_std_weak_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_28(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrFree_ReturnOrd_std_weak_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrFree_ReturnOrd_std_weak_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_28(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrFree_ReturnOrd_std_weak_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFree_ReturnOrd_std_partial_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrFree_ReturnOrd_std_partial_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_29(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrFree_ReturnOrd_std_partial_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrFree_ReturnOrd_std_partial_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_29(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrFree_ReturnOrd_std_partial_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFree_ReturnInt_ {
    pub str: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFree_CustomOrd_ {
    pub str: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFree_ReturnVoid_ {
    pub str: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrFree_NoOperators_ {
    pub str: *const (),
}
pub unsafe fn operator_cmp_27(
    mut lhs: CStrFree_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_strong_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_28(
    mut lhs: CStrFree_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_weak_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_29(
    mut lhs: CStrFree_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_partial_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_30(mut lhs: CStrFree_ReturnInt_, mut rhs: CStrFree_ReturnInt_) -> () {
    return (unsafe { ReturnInt::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_31(
    mut lhs: CStrFree_CustomOrd_,
    mut rhs: CStrFree_CustomOrd_,
) -> CustomOrd {
    return (unsafe { CustomOrd::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_32(mut lhs: CStrFree_ReturnVoid_, mut rhs: CStrFree_ReturnVoid_) {
    (unsafe { ReturnVoid::create(unimplemented!()) });
    return;
}
pub unsafe fn operator_cmp_33(
    mut lhs: CStrFree_NoOperators_,
    mut rhs: CStrFree_NoOperators_,
) -> NoOperators {
    return (unsafe { NoOperators::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_34(
    mut lhs: CStrFree_ReturnOrd_std_strong_ordering__,
    mut rhs: *const (),
) -> () {
    return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_35(
    mut lhs: CStrFree_ReturnOrd_std_weak_ordering__,
    mut rhs: *const (),
) -> () {
    return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_36(
    mut lhs: CStrFree_ReturnOrd_std_partial_ordering__,
    mut rhs: *const (),
) -> () {
    return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_37(mut lhs: CStrFree_ReturnInt_, mut rhs: *const ()) -> () {
    return (unsafe { ReturnInt::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_38(mut lhs: CStrFree_CustomOrd_, mut rhs: *const ()) -> CustomOrd {
    return (unsafe { CustomOrd::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_39(mut lhs: CStrFree_ReturnVoid_, mut rhs: *const ()) {
    (unsafe { ReturnVoid::create(unimplemented!()) });
    return;
}
pub unsafe fn operator_cmp_40(mut lhs: CStrFree_NoOperators_, mut rhs: *const ()) -> NoOperators {
    return (unsafe { NoOperators::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMixed_ReturnOrd_std_strong_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrMixed_ReturnOrd_std_strong_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_41(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrMixed_ReturnOrd_std_strong_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrMixed_ReturnOrd_std_strong_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_41(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrMixed_ReturnOrd_std_strong_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMixed_ReturnOrd_std_weak_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrMixed_ReturnOrd_std_weak_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_42(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrMixed_ReturnOrd_std_weak_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrMixed_ReturnOrd_std_weak_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_42(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrMixed_ReturnOrd_std_weak_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMixed_ReturnOrd_std_partial_ordering__ {
    pub str: *const (),
}
impl std::cmp::Ord for CStrMixed_ReturnOrd_std_partial_ordering__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_43(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for CStrMixed_ReturnOrd_std_partial_ordering__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CStrMixed_ReturnOrd_std_partial_ordering__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_43(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for CStrMixed_ReturnOrd_std_partial_ordering__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMixed_ReturnInt_ {
    pub str: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMixed_CustomOrd_ {
    pub str: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMixed_ReturnVoid_ {
    pub str: *const (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CStrMixed_NoOperators_ {
    pub str: *const (),
}
pub unsafe fn operator_cmp_41(
    mut lhs: CStrMixed_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrMixed_ReturnOrd_std_strong_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_42(
    mut lhs: CStrMixed_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrMixed_ReturnOrd_std_weak_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_43(
    mut lhs: CStrMixed_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrMixed_ReturnOrd_std_partial_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_44(mut lhs: CStrMixed_ReturnInt_, mut rhs: CStrMixed_ReturnInt_) -> () {
    return (unsafe { ReturnInt::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_45(
    mut lhs: CStrMixed_CustomOrd_,
    mut rhs: CStrMixed_CustomOrd_,
) -> CustomOrd {
    return (unsafe { CustomOrd::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_46(
    mut lhs: CStrMixed_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_strong_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_47(
    mut lhs: CStrMixed_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_weak_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_48(
    mut lhs: CStrMixed_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_partial_ordering__,
) -> () {
    return (unsafe { ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_49(mut lhs: CStrMixed_ReturnInt_, mut rhs: CStrFree_ReturnInt_) -> () {
    return (unsafe { ReturnInt::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_50(
    mut lhs: CStrMixed_CustomOrd_,
    mut rhs: CStrFree_CustomOrd_,
) -> CustomOrd {
    return (unsafe { CustomOrd::create(unimplemented!()) });
}
pub unsafe fn operator_cmp_51(mut lhs: CStrMixed_ReturnVoid_, mut rhs: CStrFree_ReturnVoid_) {
    (unsafe { ReturnVoid::create(unimplemented!()) });
    return;
}
pub unsafe fn operator_cmp_52(
    mut lhs: CStrMixed_NoOperators_,
    mut rhs: CStrFree_NoOperators_,
) -> NoOperators {
    return (unsafe { NoOperators::create(unimplemented!()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnVoid {}
impl ReturnVoid {
    pub unsafe fn create(mut _a0: ()) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoOperators {}
impl NoOperators {
    pub unsafe fn create(mut _a0: ()) -> NoOperators {
        return <NoOperators>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnOrd_std_strong_ordering_ {}
impl ReturnOrd_std_strong_ordering_ {
    pub unsafe fn create(mut cmp: ()) -> () {
        return std::cmp::Ord::cmp(&(cmp), &(0_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnOrd_std_weak_ordering_ {}
impl ReturnOrd_std_weak_ordering_ {
    pub unsafe fn create(mut cmp: ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnOrd_std_partial_ordering_ {}
impl ReturnOrd_std_partial_ordering_ {
    pub unsafe fn create(mut cmp: ()) -> () {
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ReturnInt {}
impl ReturnInt {
    pub unsafe fn create(mut cmp: ()) -> () {
        return cmp;
    }
}
pub unsafe fn operator_lt_53(mut c: CustomOrd, mut _a1: ()) -> () {
    return ((c.v) < (0_()));
}
pub unsafe fn operator_lt_54(mut _a0: (), mut c: CustomOrd) -> () {
    return ((0_()) < (c.v));
}
pub unsafe fn operator_gt_55(mut c: CustomOrd, mut _a1: ()) -> () {
    return ((c.v) > (0_()));
}
pub unsafe fn operator_gt_56(mut _a0: (), mut c: CustomOrd) -> () {
    return ((0_()) > (c.v));
}
pub unsafe fn operator_le_57(mut c: CustomOrd, mut _a1: ()) -> () {
    return ((c.v) <= (0_()));
}
pub unsafe fn operator_le_58(mut _a0: (), mut c: CustomOrd) -> () {
    return ((0_()) <= (c.v));
}
pub unsafe fn operator_ge_59(mut c: CustomOrd, mut _a1: ()) -> () {
    return ((c.v) >= (0_()));
}
pub unsafe fn operator_ge_60(mut _a0: (), mut c: CustomOrd) -> () {
    return ((0_()) >= (c.v));
}
pub unsafe fn operator_cmp_61(mut c: CustomOrd, mut _a1: ()) -> CustomOrd {
    return c;
}
pub unsafe fn operator_cmp_62(mut _a0: (), mut c: CustomOrd) -> CustomOrd {
    return CustomOrd::new_1({ -c.v });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CustomOrd {
    v: (),
}
impl CustomOrd {
    pub unsafe fn create(mut cmp: ()) -> CustomOrd {
        return CustomOrd::new_1({ cmp });
    }
    pub unsafe fn new_1(mut cmp: ()) -> Self {
        let mut this = Self { v: cmp };
        this
    }
}
impl Default for CustomOrd {
    fn default() -> Self {
        unsafe { CustomOrd { v: 0_() } }
    }
}
pub unsafe fn test_relational_63(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_64(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_65(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_66(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_67(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_68(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_69(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_70(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_71(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_72(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_73(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_74(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_75(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_76(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_77(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_78(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_79(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_80(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_81(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_82(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_83(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_84(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_85(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_86(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_87(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_88(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_89(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_90(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_91(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_92(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_93(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_94(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_95(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_96(mut use_overloaded: ()) {
    let mut cs1: CStrNone = CStrNone {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrNone = CStrNone {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) < (unsafe { CStrNone::to_pconst(&cs2) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) > (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) < (unsafe { CStrNone::to_pconst(&cs2) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) > (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) < (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) > (unsafe { CStrNone::to_pconst(&cs2) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) < (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) > (unsafe { CStrNone::to_pconst(&cs2) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_97(mut use_overloaded: ()) {
    let mut cs1: CStrNone = CStrNone {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) < (cs2))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) > (cs2)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) < (cs2))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) > (cs2)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) < (cs2)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) > (cs2))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrNone::to_pconst(&cs1) }) < (cs2)) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrNone::to_pconst(&cs1) }) > (cs2))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_98(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrNone = CStrNone {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((cs1) < (unsafe { CStrNone::to_pconst(&cs2) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((cs1) > (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((cs1) < (unsafe { CStrNone::to_pconst(&cs2) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((cs1) > (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((cs1) < (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((cs1) > (unsafe { CStrNone::to_pconst(&cs2) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((cs1) < (unsafe { CStrNone::to_pconst(&cs2) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((cs1) > (unsafe { CStrNone::to_pconst(&cs2) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_99(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnInt_ = CStrMixed_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrMixed_ReturnInt_ = cs1;
                operator_cmp_49(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_100(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMixed_ReturnInt_ = CStrMixed_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                < (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                > (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                < (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                > (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                < (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                > (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                < (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                > (unsafe {
                    let _lhs: CStrMixed_ReturnInt_ = cs2;
                    operator_cmp_49(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_101(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnInt_ = CStrMem_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMem_ReturnInt_ = CStrMem_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) < (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) > (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) < (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) > (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) < (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) > (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) < (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_2(&cs1, &cs2) }) > (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_102(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnInt_ = CStrMem_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) < (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) > (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) < (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) > (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) < (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) > (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) < (0_())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs1, cs2) }) > (0_()))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_103(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_ReturnInt_ = CStrMem_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_()) < (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_()) > (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_()) < (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_()) > (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_()) < (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_()) > (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_()) < (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_()) > (unsafe { CStrMem_ReturnInt_::operator_cmp_3(&cs2, cs1) }))) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_104(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_19(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_105(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFriend_ReturnInt_ = cs1;
                operator_cmp_20(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_106(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                < (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                > (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                < (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                > (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                < (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                > (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                < (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                > (unsafe {
                    let _lhs: CStrFriend_ReturnInt_ = cs2;
                    operator_cmp_20(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_107(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_30(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_108(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) < (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) > (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) < (0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((unsafe {
                let _lhs: CStrFree_ReturnInt_ = cs1;
                operator_cmp_37(_lhs, cs2)
            }) > (0_())))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_109(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                < (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                > (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                < (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                > (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                < (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                > (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !((0_())
                < (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!((0_())
                > (unsafe {
                    let _lhs: CStrFree_ReturnInt_ = cs2;
                    operator_cmp_37(_lhs, cs1)
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_110(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_CustomOrd_ = CStrMixed_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrMixed_CustomOrd_ = cs1;
                    operator_cmp_50(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_111(mut use_overloaded: ()) {
    let mut cs1: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMixed_CustomOrd_ = CStrMixed_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrMixed_CustomOrd_ = cs2;
                        operator_cmp_50(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_112(mut use_overloaded: ()) {
    let mut cs1: CStrMem_CustomOrd_ = CStrMem_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMem_CustomOrd_ = CStrMem_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_2(&cs1, &cs2) });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_113(mut use_overloaded: ()) {
    let mut cs1: CStrMem_CustomOrd_ = CStrMem_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs1, cs2) });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_114(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_CustomOrd_ = CStrMem_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe { CStrMem_CustomOrd_::operator_cmp_3(&cs2, cs1) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_115(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_21(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_116(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFriend_CustomOrd_ = cs1;
                    operator_cmp_22(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_117(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFriend_CustomOrd_ = cs2;
                        operator_cmp_22(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_118(mut use_overloaded: ()) {
    let mut cs1: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_31(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_119(mut use_overloaded: ()) {
    let mut cs1: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                let _c: CustomOrd = (unsafe {
                    let _lhs: CStrFree_CustomOrd_ = cs1;
                    operator_cmp_38(_lhs, cs2)
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_120(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unsafe {
                operator_lt_54(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unsafe {
                operator_gt_56(
                    unimplemented!(),
                    (unsafe {
                        let _lhs: CStrFree_CustomOrd_ = cs2;
                        operator_cmp_38(_lhs, cs1)
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_relational_type_121(mut use_overloaded: ()) {
    (unsafe { test_relational_96(use_overloaded) });
    (unsafe { test_relational_97(use_overloaded) });
    (unsafe { test_relational_98(use_overloaded) });
}
pub unsafe fn test_relational_type_122(mut use_overloaded: ()) {
    (unsafe { test_relational_65(use_overloaded) });
    (unsafe { test_relational_66(use_overloaded) });
    (unsafe { test_relational_67(use_overloaded) });
}
pub unsafe fn test_relational_type_123(mut use_overloaded: ()) {
    (unsafe { test_relational_68(use_overloaded) });
    (unsafe { test_relational_69(use_overloaded) });
    (unsafe { test_relational_70(use_overloaded) });
}
pub unsafe fn test_relational_type_124(mut use_overloaded: ()) {
    (unsafe { test_relational_71(use_overloaded) });
    (unsafe { test_relational_72(use_overloaded) });
    (unsafe { test_relational_73(use_overloaded) });
}
pub unsafe fn test_relational_type_125(mut use_overloaded: ()) {
    (unsafe { test_relational_76(use_overloaded) });
    (unsafe { test_relational_77(use_overloaded) });
    (unsafe { test_relational_78(use_overloaded) });
}
pub unsafe fn test_relational_type_126(mut use_overloaded: ()) {
    (unsafe { test_relational_79(use_overloaded) });
    (unsafe { test_relational_80(use_overloaded) });
    (unsafe { test_relational_81(use_overloaded) });
}
pub unsafe fn test_relational_type_127(mut use_overloaded: ()) {
    (unsafe { test_relational_82(use_overloaded) });
    (unsafe { test_relational_83(use_overloaded) });
    (unsafe { test_relational_84(use_overloaded) });
}
pub unsafe fn test_relational_type_128(mut use_overloaded: ()) {
    (unsafe { test_relational_87(use_overloaded) });
    (unsafe { test_relational_88(use_overloaded) });
    (unsafe { test_relational_89(use_overloaded) });
}
pub unsafe fn test_relational_type_129(mut use_overloaded: ()) {
    (unsafe { test_relational_90(use_overloaded) });
    (unsafe { test_relational_91(use_overloaded) });
    (unsafe { test_relational_92(use_overloaded) });
}
pub unsafe fn test_relational_type_130(mut use_overloaded: ()) {
    (unsafe { test_relational_93(use_overloaded) });
    (unsafe { test_relational_94(use_overloaded) });
    (unsafe { test_relational_95(use_overloaded) });
}
pub unsafe fn test_relational_type_131(mut use_overloaded: ()) {
    (unsafe { test_relational_101(use_overloaded) });
    (unsafe { test_relational_102(use_overloaded) });
    (unsafe { test_relational_103(use_overloaded) });
}
pub unsafe fn test_relational_type_132(mut use_overloaded: ()) {
    (unsafe { test_relational_104(use_overloaded) });
    (unsafe { test_relational_105(use_overloaded) });
    (unsafe { test_relational_106(use_overloaded) });
}
pub unsafe fn test_relational_type_133(mut use_overloaded: ()) {
    (unsafe { test_relational_107(use_overloaded) });
    (unsafe { test_relational_108(use_overloaded) });
    (unsafe { test_relational_109(use_overloaded) });
}
pub unsafe fn test_relational_type_134(mut use_overloaded: ()) {
    (unsafe { test_relational_112(use_overloaded) });
    (unsafe { test_relational_113(use_overloaded) });
    (unsafe { test_relational_114(use_overloaded) });
}
pub unsafe fn test_relational_type_135(mut use_overloaded: ()) {
    (unsafe { test_relational_115(use_overloaded) });
    (unsafe { test_relational_116(use_overloaded) });
    (unsafe { test_relational_117(use_overloaded) });
}
pub unsafe fn test_relational_type_136(mut use_overloaded: ()) {
    (unsafe { test_relational_118(use_overloaded) });
    (unsafe { test_relational_119(use_overloaded) });
    (unsafe { test_relational_120(use_overloaded) });
}
pub unsafe fn test_relational_return_137() {
    (unsafe { test_relational_type_131(true) });
    (unsafe { test_relational_type_132(true) });
    (unsafe { test_relational_type_133(true) });
    (unsafe { test_relational_99(true) });
    (unsafe { test_relational_100(true) });
}
pub unsafe fn test_relational_return_138() {
    (unsafe { test_relational_type_134(true) });
    (unsafe { test_relational_type_135(true) });
    (unsafe { test_relational_type_136(true) });
    (unsafe { test_relational_110(true) });
    (unsafe { test_relational_111(true) });
}
pub unsafe fn test_relational_return_139() {
    (unsafe { test_relational_type_122(true) });
    (unsafe { test_relational_type_123(true) });
    (unsafe { test_relational_type_124(true) });
    (unsafe { test_relational_63(true) });
    (unsafe { test_relational_64(true) });
}
pub unsafe fn test_relational_return_140() {
    (unsafe { test_relational_type_125(true) });
    (unsafe { test_relational_type_126(true) });
    (unsafe { test_relational_type_127(true) });
    (unsafe { test_relational_74(true) });
    (unsafe { test_relational_75(true) });
}
pub unsafe fn test_relational_return_141() {
    (unsafe { test_relational_type_128(true) });
    (unsafe { test_relational_type_129(true) });
    (unsafe { test_relational_type_130(true) });
    (unsafe { test_relational_85(true) });
    (unsafe { test_relational_86(true) });
}
pub unsafe fn test_spaceship_142(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_143(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_144(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_145(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_146(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_ReturnOrd_std_strong_ordering__ =
        CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_147(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_148(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_149(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_150(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_151(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_152(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_153(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_154(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_155(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_156(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_157(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_ReturnOrd_std_weak_ordering__ = CStrMem_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_158(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_159(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_160(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_161(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_162(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
    };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_163(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_164(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_165(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_166(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_167(mut use_overloaded: ()) {
    let mut cs1: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_168(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrMem_ReturnOrd_std_partial_ordering__ =
        CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_169(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_170(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_171(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_172(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_173(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10)),
        };
    let mut cs2: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_174(mut use_overloaded: ()) {
    let mut cs1: *const () = (*std::cell::LazyCell::force_mut(&mut *&raw mut s1_10));
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*std::cell::LazyCell::force_mut(&mut *&raw mut s2_11)),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub unsafe fn test_spaceship_type_175(mut use_overloaded: ()) {
    (unsafe { test_spaceship_144(use_overloaded) });
    (unsafe { test_spaceship_145(use_overloaded) });
    (unsafe { test_spaceship_146(use_overloaded) });
}
pub unsafe fn test_spaceship_type_176(mut use_overloaded: ()) {
    (unsafe { test_spaceship_147(use_overloaded) });
    (unsafe { test_spaceship_148(use_overloaded) });
    (unsafe { test_spaceship_149(use_overloaded) });
}
pub unsafe fn test_spaceship_type_177(mut use_overloaded: ()) {
    (unsafe { test_spaceship_150(use_overloaded) });
    (unsafe { test_spaceship_151(use_overloaded) });
    (unsafe { test_spaceship_152(use_overloaded) });
}
pub unsafe fn test_spaceship_type_178(mut use_overloaded: ()) {
    (unsafe { test_spaceship_155(use_overloaded) });
    (unsafe { test_spaceship_156(use_overloaded) });
    (unsafe { test_spaceship_157(use_overloaded) });
}
pub unsafe fn test_spaceship_type_179(mut use_overloaded: ()) {
    (unsafe { test_spaceship_158(use_overloaded) });
    (unsafe { test_spaceship_159(use_overloaded) });
    (unsafe { test_spaceship_160(use_overloaded) });
}
pub unsafe fn test_spaceship_type_180(mut use_overloaded: ()) {
    (unsafe { test_spaceship_161(use_overloaded) });
    (unsafe { test_spaceship_162(use_overloaded) });
    (unsafe { test_spaceship_163(use_overloaded) });
}
pub unsafe fn test_spaceship_type_181(mut use_overloaded: ()) {
    (unsafe { test_spaceship_166(use_overloaded) });
    (unsafe { test_spaceship_167(use_overloaded) });
    (unsafe { test_spaceship_168(use_overloaded) });
}
pub unsafe fn test_spaceship_type_182(mut use_overloaded: ()) {
    (unsafe { test_spaceship_169(use_overloaded) });
    (unsafe { test_spaceship_170(use_overloaded) });
    (unsafe { test_spaceship_171(use_overloaded) });
}
pub unsafe fn test_spaceship_type_183(mut use_overloaded: ()) {
    (unsafe { test_spaceship_172(use_overloaded) });
    (unsafe { test_spaceship_173(use_overloaded) });
    (unsafe { test_spaceship_174(use_overloaded) });
}
pub unsafe fn test_std_ordering_184() {
    (unsafe { test_relational_return_139() });
    (unsafe { test_spaceship_type_175(true) });
    (unsafe { test_spaceship_type_176(true) });
    (unsafe { test_spaceship_type_177(true) });
    (unsafe { test_spaceship_142(true) });
    (unsafe { test_spaceship_143(true) });
}
pub unsafe fn test_std_ordering_185() {
    (unsafe { test_relational_return_140() });
    (unsafe { test_spaceship_type_178(true) });
    (unsafe { test_spaceship_type_179(true) });
    (unsafe { test_spaceship_type_180(true) });
    (unsafe { test_spaceship_153(true) });
    (unsafe { test_spaceship_154(true) });
}
pub unsafe fn test_std_ordering_186() {
    (unsafe { test_relational_return_141() });
    (unsafe { test_spaceship_type_181(true) });
    (unsafe { test_spaceship_type_182(true) });
    (unsafe { test_spaceship_type_183(true) });
    (unsafe { test_spaceship_164(true) });
    (unsafe { test_spaceship_165(true) });
}
pub unsafe fn test_no_relational_187() {
    let mut c1: CStrMixed_ReturnVoid_ = <CStrMixed_ReturnVoid_>::default();
    let mut c2: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_188() {
    let mut c1: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    let mut c2: CStrMixed_ReturnVoid_ = <CStrMixed_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_189() {
    let mut c1: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    let mut c2: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_190() {
    let mut c1: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    let mut c2: *const () = std::ptr::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_191() {
    let mut c1: *const () = std::ptr::null();
    let mut c2: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_192() {
    let mut c1: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    let mut c2: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_193() {
    let mut c1: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    let mut c2: *const () = std::ptr::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_194() {
    let mut c1: *const () = std::ptr::null();
    let mut c2: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_195() {
    let mut c1: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    let mut c2: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_196() {
    let mut c1: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    let mut c2: *const () = std::ptr::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_197() {
    let mut c1: *const () = std::ptr::null();
    let mut c2: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_198() {
    let mut c1: CStrMixed_NoOperators_ = <CStrMixed_NoOperators_>::default();
    let mut c2: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_199() {
    let mut c1: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    let mut c2: CStrMixed_NoOperators_ = <CStrMixed_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_200() {
    let mut c1: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    let mut c2: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_201() {
    let mut c1: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    let mut c2: *const () = std::ptr::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_202() {
    let mut c1: *const () = std::ptr::null();
    let mut c2: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_203() {
    let mut c1: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    let mut c2: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_204() {
    let mut c1: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    let mut c2: *const () = std::ptr::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_205() {
    let mut c1: *const () = std::ptr::null();
    let mut c2: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_206() {
    let mut c1: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    let mut c2: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_207() {
    let mut c1: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    let mut c2: *const () = std::ptr::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_208() {
    let mut c1: *const () = std::ptr::null();
    let mut c2: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub unsafe fn test_no_relational_type_209() {
    (unsafe { test_no_relational_189() });
    (unsafe { test_no_relational_190() });
    (unsafe { test_no_relational_191() });
}
pub unsafe fn test_no_relational_type_210() {
    (unsafe { test_no_relational_192() });
    (unsafe { test_no_relational_193() });
    (unsafe { test_no_relational_194() });
}
pub unsafe fn test_no_relational_type_211() {
    (unsafe { test_no_relational_195() });
    (unsafe { test_no_relational_196() });
    (unsafe { test_no_relational_197() });
}
pub unsafe fn test_no_relational_type_212() {
    (unsafe { test_no_relational_200() });
    (unsafe { test_no_relational_201() });
    (unsafe { test_no_relational_202() });
}
pub unsafe fn test_no_relational_type_213() {
    (unsafe { test_no_relational_203() });
    (unsafe { test_no_relational_204() });
    (unsafe { test_no_relational_205() });
}
pub unsafe fn test_no_relational_type_214() {
    (unsafe { test_no_relational_206() });
    (unsafe { test_no_relational_207() });
    (unsafe { test_no_relational_208() });
}
pub unsafe fn test_no_relational_return_215() {
    (unsafe { test_no_relational_type_209() });
    (unsafe { test_no_relational_type_210() });
    (unsafe { test_no_relational_type_211() });
    (unsafe { test_no_relational_187() });
    (unsafe { test_no_relational_188() });
}
pub unsafe fn test_no_relational_return_216() {
    (unsafe { test_no_relational_type_212() });
    (unsafe { test_no_relational_type_213() });
    (unsafe { test_no_relational_type_214() });
    (unsafe { test_no_relational_198() });
    (unsafe { test_no_relational_199() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_std_ordering_184() });
    (unsafe { test_std_ordering_185() });
    (unsafe { test_std_ordering_186() });
    (unsafe { test_relational_type_121(false) });
    (unsafe { test_relational_return_137() });
    (unsafe { test_relational_return_138() });
    (unsafe { test_no_relational_return_215() });
    (unsafe { test_no_relational_return_216() });
    0
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: unimplemented!(),
        };
        this.pid_ = unimplemented!();
        this.sem_set_ = unimplemented!();
        if ((this.sem_set_) == (-1_())) {
            unimplemented!();
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
        this
    }
}
pub unsafe fn set_memory_limits_217(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) * (1048576_()));
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn set_file_limit_218(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_219(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = unimplemented!();
    if !(demangled).is_null() {
        s = (demangled).cast_const();
    } else {
        switch!(match status {
            __v if __v == 0_() => {
                s = c"error code = 0: success".as_ptr();
                break;
            }
            __v if __v == -1_() => {
                s = c"error code = -1: memory allocation failure".as_ptr();
                break;
            }
            __v if __v == -2_() => {
                s = c"error code = -2: invalid mangled name".as_ptr();
                break;
            }
            __v if __v == -3_() => {
                s = c"error code = -3: invalid arguments".as_ptr();
                break;
            }
            _ => {
                s = c"error code unknown - who knows what happened".as_ptr();
            }
        });
    }
    let mut w: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_220(
    mut name: *const (),
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const () = (unimplemented!()).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: () = unimplemented!();
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn run_tests_wrapped_env_221(
    mut name: *const (),
    mut env: *const (),
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut oldENV: *const () = (unimplemented!()).cast_const();
    if !(unimplemented!() != 0) {
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        unimplemented!();
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: (),
    pub buf: *mut (),
    pub array: *mut (),
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl __gnu_test_semaphore {}
impl __gnu_test_semaphore {
    pub unsafe fn destructor(&mut self) {
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((self.pid_) == (unimplemented!())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
pub unsafe fn test_tm_222(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> () {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_223)).clone();
}
static mut tmp_223: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const arr_9);
    std::cell::LazyCell::force(&*&raw const s1_10);
    std::cell::LazyCell::force(&*&raw const s2_11);
    std::cell::LazyCell::force(&*&raw const s3_12);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
