extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
pub unsafe fn operator_ne_4(
    x: *const cpp17_input_iterator_int_ptr__int_ptr_,
    y: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_ne_5(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub unsafe fn operator_eq_6(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_7(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_8(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_7(_i, n)
    });
}
pub unsafe fn iter_swap_9(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_10(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_11(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_12(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_12(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct EmplaceConstructibleMoveableAndAssignable_int_ {
    pub copied: (),
    pub assigned: (),
    pub value: (),
}
impl EmplaceConstructibleMoveableAndAssignable_int_ {
    pub unsafe fn new(mut xvalue: ()) -> Self {
        let mut this = Self {
            copied: 0_(),
            assigned: 0_(),
            value: xvalue,
        };
        this
    }
    pub unsafe fn move_from(Other: *mut EmplaceConstructibleMoveableAndAssignable_int_) -> Self {
        let mut this = Self {
            copied: (((*Other).copied) + (1_())),
            assigned: 0_(),
            value: ((*Other).value as ()),
        };
        this
    }
    pub unsafe fn operator_assign_4(
        &mut self,
        mut xvalue: (),
    ) -> *mut EmplaceConstructibleMoveableAndAssignable_int_ {
        self.value = (xvalue as ());
        self.assigned.prefix_inc();
        return &mut (*(self as *mut EmplaceConstructibleMoveableAndAssignable_int_));
    }
}
impl Default for EmplaceConstructibleMoveableAndAssignable_int_ {
    fn default() -> Self {
        unsafe {
            EmplaceConstructibleMoveableAndAssignable_int_ {
                copied: 0_(),
                assigned: 0_(),
                value: unimplemented!(),
            }
        }
    }
}
pub unsafe fn operator_eq_13(LHS: *const detail_TypeID, RHS: *const detail_TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_14(LHS: *const detail_TypeID, RHS: *const detail_TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_TypeID {
    m_id: *const (),
}
impl detail_TypeID {
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for detail_TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self as *const detail_TypeID, other as *const detail_TypeID) }
    }
}
impl std::cmp::Eq for detail_TypeID {}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct AllocatorConstructController {
    pub m_expected_args: *const detail_TypeID,
    pub m_allow_constructions: (),
    pub m_allow_unchecked: (),
    pub m_expected_count: (),
}
impl AllocatorConstructController {
    pub unsafe fn clear(&mut self) {
        self.m_expected_args = std::ptr::null();
        self.m_expected_count = -1_();
    }
    pub unsafe fn check(&mut self, tid: *const detail_TypeID) -> () {
        if !(!(self.m_expected_args).is_null()) {
            unimplemented!();
            return self.m_allow_unchecked;
        }
        let mut res: () = (unsafe {
            let _LHS: *const detail_TypeID = &(*self.m_expected_args);
            let _RHS: *const detail_TypeID = tid;
            operator_eq_13(_LHS, _RHS)
        });
        if ((self.m_expected_count) == (-1_())) || ((self.m_expected_count.prefix_dec()) == (-1_()))
        {
            self.m_expected_args = std::ptr::null();
        }
        return res;
    }
    pub unsafe fn unchecked(&mut self) -> () {
        return !((self.m_expected_args).is_null());
    }
    pub unsafe fn expect(&mut self, tid: *const detail_TypeID) {
        unimplemented!();
        self.m_expected_args = (tid);
    }
    pub unsafe fn isInAllocatorConstruct(&self) -> () {
        return self.m_allow_constructions;
    }
    pub unsafe fn inAllocatorConstruct(&mut self, mut value: Option<()>) {
        let mut value: () = value.unwrap_or_else(|| unsafe { true });
        self.m_allow_constructions = value;
    }
    pub unsafe fn allowUnchecked(&mut self, mut value: Option<()>) {
        let mut value: () = value.unwrap_or_else(|| unsafe { true });
        self.m_allow_unchecked = value;
    }
    pub unsafe fn reset(&mut self) {
        self.m_allow_constructions = false;
        self.m_expected_args = std::ptr::null();
        self.m_allow_unchecked = false;
        self.m_expected_count = -1_();
    }
    unsafe fn new() -> Self {
        let mut this = Self {
            m_expected_args: std::ptr::null(),
            m_allow_constructions: unimplemented!(),
            m_allow_unchecked: unimplemented!(),
            m_expected_count: unimplemented!(),
        };
        (unsafe { AllocatorConstructController::reset(&mut this) });
        this
    }
}
impl Default for AllocatorConstructController {
    fn default() -> Self {
        unsafe { AllocatorConstructController::new() }
    }
}
pub unsafe fn getConstructController_15() -> *mut AllocatorConstructController {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_16))
        as *mut AllocatorConstructController);
}
static mut c_16: std::cell::LazyCell<AllocatorConstructController> =
    std::cell::LazyCell::new(|| unsafe { AllocatorConstructController::new() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerTestAllocator_int__int_ {
    pub construct_called: (),
    pub destroy_called: (),
    pub controller: *mut AllocatorConstructController,
}
impl ContainerTestAllocator_int__int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerTestAllocator_float__int_ {
    pub construct_called: (),
    pub destroy_called: (),
    pub controller: *mut AllocatorConstructController,
}
impl ContainerTestAllocator_float__int_ {}
const _: () = assert!(true, "std::is_same<A1T::rebind_traits<float>, A2T>::value");
const _: () = assert!(true, "std::is_same<A2T::rebind_traits<int>, A1T>::value");
pub unsafe fn test_17() -> () {
    let mut arr1: [(); 1] = [42_()];
    let mut arr2: [(); 3] = [1_(), 101_(), 42_()];
    {
        {
            let mut v: () = unimplemented!();
            (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ arr1.as_mut_ptr() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                v.assign_23_class_forward_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ arr2.as_mut_ptr() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                v.assign_23_class_forward_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            (unsafe {
                let ___first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ arr1.as_mut_ptr() });
                let ___last: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
                v.assign_22_class_cpp17_input_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            (unsafe {
                let ___first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ arr2.as_mut_ptr() });
                let ___last: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
                v.assign_22_class_cpp17_input_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) < (unimplemented!())) {
                unimplemented!();
                i.prefix_inc();
            }
            (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ arr1.as_mut_ptr() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                v.assign_23_class_forward_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) < (unimplemented!())) {
                unimplemented!();
                i.prefix_inc();
            }
            (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ arr2.as_mut_ptr() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                v.assign_23_class_forward_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) < (unimplemented!())) {
                unimplemented!();
                i.prefix_inc();
            }
            (unsafe {
                let ___first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ arr1.as_mut_ptr() });
                let ___last: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
                v.assign_22_class_cpp17_input_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) < (unimplemented!())) {
                unimplemented!();
                i.prefix_inc();
            }
            (unsafe {
                let ___first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ arr2.as_mut_ptr() });
                let ___last: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
                v.assign_22_class_cpp17_input_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ arr1.as_mut_ptr() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                v.assign_23_class_forward_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) < (2_())) {
                unimplemented!();
                i.prefix_inc();
            }
            (unsafe {
                let ___first: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ arr2.as_mut_ptr() });
                let ___last: forward_iterator_int_ptr_ =
                    forward_iterator_int_ptr_::new_1({ unimplemented!() });
                v.assign_23_class_forward_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            (unsafe {
                let ___first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ arr1.as_mut_ptr() });
                let ___last: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
                v.assign_22_class_cpp17_input_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) < (2_())) {
                unimplemented!();
                i.prefix_inc();
            }
            (unsafe {
                let ___first: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ arr2.as_mut_ptr() });
                let ___last: cpp17_input_iterator_int_ptr__int_ptr_ =
                    cpp17_input_iterator_int_ptr__int_ptr_::new({ unimplemented!() });
                v.assign_22_class_cpp17_input_iterator_int_ptr__0(___first, ___last)
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut dst: () = unimplemented!();
        let mut n: () = (unimplemented!()).wrapping_mul(2_());
        let mut src: () = unimplemented!();
        (unsafe {
            let ___first: forward_iterator_int_ptr_ =
                forward_iterator_int_ptr_::new_1({ unimplemented!() });
            let ___last: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                unimplemented!().offset((unimplemented!()) as isize)
            });
            dst.assign_23_class_forward_iterator_int_ptr__0(___first, ___last)
        });
        unimplemented!();
    }
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_17() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
