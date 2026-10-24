extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_6(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_13(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
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
        unsafe {
            operator_eq_7(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_add_14(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_15(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_16(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_17(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_18(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
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
    pub unsafe fn operator_index(&self, mut n: ()) -> () {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_int_ptr_,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_16(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
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
pub unsafe fn base_19(mut i: *const ()) -> *const () {
    return i;
}
pub unsafe fn base_20(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
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
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {
    base_: InputIteratorNotDerivedFromGeneric_int_,
}
impl sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {
    base_: InputIteratorNotIndirectlyReadable,
}
impl sentinel_wrapper_InputIteratorNotIndirectlyReadable_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {
    base_: InputIteratorNotInputOrOutputIterator,
}
impl sentinel_wrapper_InputIteratorNotInputOrOutputIterator_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {
    base_: BidirectionalIteratorNotDerivedFrom,
}
impl sentinel_wrapper_BidirectionalIteratorNotDerivedFrom_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {
    base_: BidirectionalIteratorNotDecrementable,
}
impl sentinel_wrapper_BidirectionalIteratorNotDecrementable_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_ {
    base_: RandomAccessIteratorNotDerivedFrom,
}
impl sentinel_wrapper_RandomAccessIteratorNotDerivedFrom_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_RandomAccessIteratorBadIndex_ {
    base_: RandomAccessIteratorBadIndex,
}
impl sentinel_wrapper_RandomAccessIteratorBadIndex_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_OrderedConvertibleToInt_ptr_ {
    base_: *mut OrderedConvertibleToInt,
}
impl sentinel_wrapper_OrderedConvertibleToInt_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_const_int_ptr_ {
    base_: *const (),
}
impl sentinel_wrapper_const_int_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_19((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *const ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_19((*other)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_NoComparator_ptr_ {
    base_: *mut NoComparator,
}
impl sentinel_wrapper_NoComparator_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_MoveOnly_ptr_ {
    base_: *mut MoveOnly,
}
impl sentinel_wrapper_MoveOnly_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_20(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_20(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_random_access_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_18(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_18(other) }));
    }
}
pub unsafe fn operator_eq_21(
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
            operator_eq_21(
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
pub unsafe fn operator_add_22(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_23(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_22(_i, n)
    });
}
pub unsafe fn iter_swap_24(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_25(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_26(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_27(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_27(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_26(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_int_ptr__int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_InputIteratorNotDerivedFromGeneric_int___sentinel_wrapper_InputIteratorNotDerivedFromGeneric_int___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_InputIteratorNotIndirectlyReadable__sentinel_wrapper_InputIteratorNotIndirectlyReadable__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_InputIteratorNotInputOrOutputIterator__sentinel_wrapper_InputIteratorNotInputOrOutputIterator__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_BidirectionalIteratorNotDerivedFrom__sentinel_wrapper_BidirectionalIteratorNotDerivedFrom__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_BidirectionalIteratorNotDecrementable__sentinel_wrapper_BidirectionalIteratorNotDecrementable__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_RandomAccessIteratorNotDerivedFrom__sentinel_wrapper_RandomAccessIteratorNotDerivedFrom__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_RandomAccessIteratorBadIndex__sentinel_wrapper_RandomAccessIteratorBadIndex__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_int_ptr__sentinel_wrapper_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_OrderedConvertibleToInt_ptr__sentinel_wrapper_OrderedConvertibleToInt_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_const_int_ptr__sentinel_wrapper_const_int_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_NoComparator_ptr__sentinel_wrapper_NoComparator_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct UncheckedRange_MoveOnly_ptr__sentinel_wrapper_MoveOnly_ptr__ {}
const _: () = assert!(
    true,
    "std::ranges::contiguous_range<UncheckedRange<int*, int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InputIteratorNotDerivedFromGeneric_int_ {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InputIteratorNotIndirectlyReadable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_readable<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotIndirectlyReadable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_readable<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::input_iterator<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::input_range<InputRangeNotInputOrOutputIterator>"
);
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct IndirectUnaryPredicateNotCopyConstructible {}
const _: () = assert!(
    true,
    "std::predicate<IndirectUnaryPredicateNotCopyConstructible, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotCopyConstructible, int*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IndirectUnaryPredicateNotPredicate {}
const _: () = assert!(
    !(false),
    "!std::predicate<IndirectUnaryPredicateNotPredicate, int&>"
);
const _: () = assert!(
    !(false),
    "!std::indirect_unary_predicate<IndirectUnaryPredicateNotPredicate, int*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SentinelForNotSemiregular {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotSemiregular>"
);
const _: () = assert!(!(false), "!std::semiregular<SentinelForNotSemiregular>");
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotSemiregular, cpp20_input_iterator<int*>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SentinelForNotWeaklyEqualityComparableWith {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    true,
    "std::semiregular<SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!std::sentinel_for<SentinelForNotWeaklyEqualityComparableWith, cpp20_input_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct WeaklyIncrementableNotMovable {}
const _: () = assert!(!(false), "!std::movable<WeaklyIncrementableNotMovable>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<WeaklyIncrementableNotMovable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardIteratorNotDerivedFrom {}
impl ForwardIteratorNotDerivedFrom {
    pub unsafe fn operator_eq(&self, _a0: *const ForwardIteratorNotDerivedFrom) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for ForwardIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ForwardIteratorNotDerivedFrom::operator_eq(
                self,
                other as *const ForwardIteratorNotDerivedFrom,
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotDerivedFrom {}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(true, "std::incrementable<ForwardIteratorNotDerivedFrom>");
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotDerivedFrom, ForwardIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardIteratorNotIncrementable {}
impl ForwardIteratorNotIncrementable {
    pub unsafe fn operator_eq(&self, _a0: *const ForwardIteratorNotIncrementable) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for ForwardIteratorNotIncrementable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ForwardIteratorNotIncrementable::operator_eq(
                self,
                other as *const ForwardIteratorNotIncrementable,
            )
        }
    }
}
impl std::cmp::Eq for ForwardIteratorNotIncrementable {}
const _: () = assert!(true, "std::input_iterator<ForwardIteratorNotIncrementable>");
const _: () = assert!(
    !(false),
    "!std::incrementable<ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    true,
    "std::sentinel_for<ForwardIteratorNotIncrementable, ForwardIteratorNotIncrementable>"
);
const _: () = assert!(
    !(false),
    "!std::forward_iterator<ForwardIteratorNotIncrementable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BidirectionalIteratorNotDerivedFrom {}
impl BidirectionalIteratorNotDerivedFrom {
    pub unsafe fn operator_eq(&self, _a0: *const BidirectionalIteratorNotDerivedFrom) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for BidirectionalIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            BidirectionalIteratorNotDerivedFrom::operator_eq(
                self,
                other as *const BidirectionalIteratorNotDerivedFrom,
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDerivedFrom {}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BidirectionalIteratorNotDecrementable {}
impl BidirectionalIteratorNotDecrementable {
    pub unsafe fn operator_eq(&self, _a0: *const BidirectionalIteratorNotDecrementable) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for BidirectionalIteratorNotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            BidirectionalIteratorNotDecrementable::operator_eq(
                self,
                other as *const BidirectionalIteratorNotDecrementable,
            )
        }
    }
}
impl std::cmp::Eq for BidirectionalIteratorNotDecrementable {}
const _: () = assert!(
    true,
    "std::forward_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::bidirectional_iterator<BidirectionalIteratorNotDecrementable>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::bidirectional_range<BidirectionalRangeNotDecrementable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PermutableNotForwardIterator {}
const _: () = assert!(true, "std::input_iterator<PermutableNotForwardIterator>");
const _: () = assert!(
    !(false),
    "!std::forward_iterator<PermutableNotForwardIterator>"
);
const _: () = assert!(!(false), "!std::permutable<PermutableNotForwardIterator>");
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct PermutableNotSwappable_NotSwappable {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PermutableNotSwappable {}
impl PermutableNotSwappable {
    pub unsafe fn operator_eq(&self, _a0: *const PermutableNotSwappable) -> () {
        return true;
    }
}
impl std::cmp::PartialEq for PermutableNotSwappable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { PermutableNotSwappable::operator_eq(self, other as *const PermutableNotSwappable) }
    }
}
impl std::cmp::Eq for PermutableNotSwappable {}
const _: () = assert!(true, "std::input_iterator<PermutableNotSwappable>");
const _: () = assert!(true, "std::forward_iterator<PermutableNotSwappable>");
const _: () = assert!(!(false), "!std::permutable<PermutableNotSwappable>");
const _: () = assert!(
    !(false),
    "!std::indirectly_swappable<PermutableNotSwappable>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OutputIteratorNotInputOrOutputIterator {}
const _: () = assert!(
    !(false),
    "!std::input_or_output_iterator<OutputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    true,
    "std::indirectly_writable<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotInputOrOutputIterator, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputRangeNotInputOrOutputIterator, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OutputIteratorNotIndirectlyWritable {}
const _: () = assert!(
    true,
    "std::input_or_output_iterator<OutputIteratorNotIndirectlyWritable>"
);
const _: () = assert!(
    !(false),
    "!std::indirectly_writable<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::output_iterator<OutputIteratorNotIndirectlyWritable, int>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::output_range<OutputIteratorNotIndirectlyWritable, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IndirectBinaryPredicateNotIndirectlyReadable {}
const _: () = assert!(
    !(false),
    "!std::indirect_binary_predicate<std::ranges::equal_to, IndirectBinaryPredicateNotIndirectlyReadable, int*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomAccessIteratorNotDerivedFrom {}
impl RandomAccessIteratorNotDerivedFrom {
    pub unsafe fn operator_cmp(&self, _a0: *const RandomAccessIteratorNotDerivedFrom) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const RandomAccessIteratorNotDerivedFrom) -> () {
        return true;
    }
}
impl std::cmp::Ord for RandomAccessIteratorNotDerivedFrom {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            RandomAccessIteratorNotDerivedFrom::operator_cmp(
                self,
                other as *const RandomAccessIteratorNotDerivedFrom,
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorNotDerivedFrom {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorNotDerivedFrom {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            RandomAccessIteratorNotDerivedFrom::operator_cmp(
                self,
                other as *const RandomAccessIteratorNotDerivedFrom,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorNotDerivedFrom {}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorNotDerivedFrom>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomAccessIteratorBadIndex {}
impl RandomAccessIteratorBadIndex {
    pub unsafe fn operator_cmp(&self, _a0: *const RandomAccessIteratorBadIndex) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const RandomAccessIteratorBadIndex) -> () {
        return true;
    }
}
impl std::cmp::Ord for RandomAccessIteratorBadIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            RandomAccessIteratorBadIndex::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadIndex,
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadIndex {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            RandomAccessIteratorBadIndex::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadIndex,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadIndex {}
const _: () = assert!(
    true,
    "std::bidirectional_iterator<RandomAccessIteratorBadIndex>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadIndex>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomAccessIteratorBadDifferenceType {}
impl RandomAccessIteratorBadDifferenceType {
    pub unsafe fn operator_cmp(&self, _a0: *const RandomAccessIteratorBadDifferenceType) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const RandomAccessIteratorBadDifferenceType) -> () {
        return true;
    }
}
impl std::cmp::Ord for RandomAccessIteratorBadDifferenceType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            RandomAccessIteratorBadDifferenceType::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadDifferenceType,
            )
        }
    }
}
impl std::cmp::PartialOrd for RandomAccessIteratorBadDifferenceType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for RandomAccessIteratorBadDifferenceType {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            RandomAccessIteratorBadDifferenceType::operator_cmp(
                self,
                other as *const RandomAccessIteratorBadDifferenceType,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for RandomAccessIteratorBadDifferenceType {}
const _: () = assert!(true, "std::regular<RandomAccessIteratorBadDifferenceType>");
const _: () = assert!(
    !(false),
    "!std::weakly_incrementable<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    !(false),
    "!std::random_access_iterator<RandomAccessIteratorBadDifferenceType>"
);
const _: () = assert!(
    true,
    "HasPartialSortCopyIter<int*, int*, int*, int*, std::ranges::less>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<InputIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<InputIteratorNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<InputIteratorNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, RandomAccessIteratorNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, RandomAccessIteratorBadIndex>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, SentinelForNotSemiregular>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, SentinelForNotWeaklyEqualityComparableWith>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, int*, IndirectUnaryPredicateNotPredicate>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, int*, int*, IndirectUnaryPredicateNotCopyConstructible>"
);
pub unsafe fn operator_cmp_28(
    _a0: *const OrderedConvertibleToInt,
    _a1: *const OrderedConvertibleToInt,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_29(
    _a0: *const OrderedConvertibleToInt,
    _a1: *const OrderedConvertibleToInt,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OrderedConvertibleToInt {}
impl std::cmp::Ord for OrderedConvertibleToInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_28(
                self as *const OrderedConvertibleToInt,
                other as *const OrderedConvertibleToInt,
            )
        }
    }
}
impl std::cmp::PartialOrd for OrderedConvertibleToInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for OrderedConvertibleToInt {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_cmp_28(
                self as *const OrderedConvertibleToInt,
                other as *const OrderedConvertibleToInt,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for OrderedConvertibleToInt {}
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, OrderedConvertibleToInt*, OrderedConvertibleToInt*>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<int*, int*, const int*, const int*>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoComparator {}
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyIter<NoComparator*, NoComparator*, NoComparator*, NoComparator*>"
);
const _: () = assert!(
    true,
    "HasPartialSortCopyIter<int*, int*, MoveOnly*, MoveOnly*>"
);
const _: () = assert!(
    true,
    "HasPartialSortCopyRange<R<int*>, R<int*>, std::ranges::less>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<InputRangeNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<InputRangeNotIndirectlyReadable>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<InputRangeNotInputOrOutputIterator>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<int*>, RandomAccessRangeNotDerivedFrom>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<int*>, RandomAccessRangeBadIndex>"
);
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<int*>, R<OrderedConvertibleToInt*>>"
);
const _: () = assert!(!(false), "!HasPartialSortCopyRange<R<int*>, R<const int*>>");
const _: () = assert!(
    !(false),
    "!HasPartialSortCopyRange<R<NoComparator*>, R<NoComparator*>>"
);
const _: () = assert!(true, "HasPartialSortCopyRange<R<int*>, R<MoveOnly*>>");
const _: () = assert!(
    true,
    "std::is_same_v<std::ranges::partial_sort_copy_result<int, int>, std::ranges::in_out_result<int, int>>"
);
pub unsafe fn test_one_30(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_0: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_0 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_33(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_1: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_1 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_34(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_2: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_2 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_35(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_3: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_3 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_36(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_4: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_4 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_37(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_5: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_5 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_38(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_6: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_6 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_39(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_7: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_7 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_8: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_8 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_40(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_9: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_9 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_10: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_10 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_41(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_11: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_11 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_12: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_12 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_42(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_13: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_13 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_14: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_14 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_43(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_15: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_15 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_16: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_16 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_44(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_17: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_17 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_18: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_18 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_45(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                cpp20_input_iterator_int_ptr_::new({ begin }),
                {
                    let mut __tmp_19: cpp20_input_iterator_int_ptr_ =
                        cpp20_input_iterator_int_ptr_::new({ end });
                    sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_19 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_20: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_20 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_46(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_47(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_48(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_49(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_50(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_51(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_52(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_53(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_21: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_21 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_54(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_22: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_22 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_55(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_23: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_23 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_56(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_24: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_24 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_57(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_25: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_25 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_58(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_26: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_26 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_59(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                random_access_iterator_int_ptr_::new_1({ end }),
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_27: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_27 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_60(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_28: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_28 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_61(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_29: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_29 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_62(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_30: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_30 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_63(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_31: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_31 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_64(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_32: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_32 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_65(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_33: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_33 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_66(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_34: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_34 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_67(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_35: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_35 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_36: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_36 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_68(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_37: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_37 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_38: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_38 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_69(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_39: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_39 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_40: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_40 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_70(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_41: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_41 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_42: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_42 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_71(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_43: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_43 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_44: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_44 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_72(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_45: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_45 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_46: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_46 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_73(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                random_access_iterator_int_ptr_::new_1({ begin }),
                {
                    let mut __tmp_47: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_47 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_48: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_48 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_74(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_75(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_76(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_77(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_78(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_79(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_80(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_81(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_49: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_49 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_82(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_50: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_50 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_83(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_51: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_51 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_84(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_52: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_52 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_85(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_53: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_53 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_86(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_54: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_54 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_87(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                end,
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_55: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_55 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_88(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_56: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_56 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_89(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_57: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_57 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_90(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_58: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_58 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_91(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_59: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_59 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_92(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_60: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_60 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_93(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_61: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_61 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_94(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_62: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_62 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                random_access_iterator_int_ptr_::new_1({ out_end }),
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_95(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (0_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_63: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_63 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_64: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_64 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_96(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (1_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_65: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_65 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_66: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_66 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_97(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (2_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_67: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_67 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_68: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_68 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_98(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (3_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_69: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_69 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_70: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_70 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_99(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (8_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_71: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_71 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_72: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_72 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_100(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (7_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_73: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_73 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_74: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_74 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_one_101(mut input: (), mut input_size: (), mut output_size: (), mut sorted: ()) {
    unimplemented!();
    unimplemented!();
    let OutputSize: () = (5_() as ()).wrapping_add(1_());
    let mut result_size: () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_31))(
            &input_size,
            &output_size,
            None,
            None,
        )
    });
    let mut begin: *mut () = unimplemented!();
    let mut end: *mut () = unimplemented!().offset((input_size) as isize);
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                begin,
                {
                    let mut __tmp_75: *const () = end;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_75 })
                },
                random_access_iterator_int_ptr_::new_1({ out_begin }),
                {
                    let mut __tmp_76: random_access_iterator_int_ptr_ =
                        random_access_iterator_int_ptr_::new_1({ out_end });
                    sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &mut __tmp_76 })
                },
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut out: () = unimplemented!();
        let mut out_begin: *mut () = unimplemented!();
        let mut out_end: *mut () = unimplemented!().offset((output_size) as isize);
        let mut in_range: () = unimplemented!();
        let mut out_range: () = unimplemented!();
        let mut result: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                &mut in_range,
                &mut out_range,
                None,
                None,
                None,
            )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_all_subsequences_102(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_30(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_103(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_33(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_104(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_34(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_105(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_35(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_106(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_36(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_107(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_37(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_108(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_38(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_109(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_39(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_110(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_40(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_111(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_41(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_112(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_42(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_113(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_43(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_114(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_44(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_115(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_45(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_116(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_46(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_117(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_47(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_118(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_48(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_119(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_49(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_120(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_50(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_121(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_51(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_122(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_52(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_123(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_53(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_124(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_54(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_125(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_55(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_126(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_56(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_127(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_57(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_128(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_58(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_129(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_59(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_130(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_60(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_131(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_61(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_132(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_62(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_133(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_63(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_134(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_64(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_135(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_65(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_136(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_66(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_137(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_67(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_138(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_68(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_139(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_69(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_140(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_70(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_141(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_71(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_142(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_72(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_143(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_73(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_144(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_74(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_145(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_75(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_146(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_76(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_147(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_77(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_148(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_78(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_149(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_79(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_150(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_80(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_151(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_81(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_152(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_82(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_153(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_83(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_154(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_84(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_155(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_85(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_156(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_86(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_157(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_87(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_158(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_88(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_159(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_89(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_160(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_90(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_161(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_91(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_162(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_92(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_163(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_93(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_164(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_94(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_165(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((0_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_95(input.clone(), 0_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_166(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((1_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_96(input.clone(), 1_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_167(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((2_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_97(input.clone(), 2_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_168(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((3_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_98(input.clone(), 3_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_169(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((8_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_99(input.clone(), 8_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_170(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((7_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_100(input.clone(), 7_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_all_subsequences_171(input: ()) {
    let mut sorted: () = input.clone();
    unimplemented!();
    let mut out_size: () = 0_();
    'loop_: while ((out_size) <= ((5_() as ()).wrapping_add(1_()))) {
        (unsafe { test_one_101(input.clone(), 5_(), out_size, sorted.clone()) });
        out_size.prefix_inc();
    }
}
pub unsafe fn test_iterators_in_sent1_out_sent2_172() {
    (unsafe { test_all_subsequences_102(unimplemented!()) });
    (unsafe { test_all_subsequences_103(unimplemented!()) });
    (unsafe { test_all_subsequences_104(unimplemented!()) });
    (unsafe { test_all_subsequences_105(unimplemented!()) });
    (unsafe { test_all_subsequences_106(unimplemented!()) });
    (unsafe { test_all_subsequences_107(unimplemented!()) });
    (unsafe { test_all_subsequences_105(unimplemented!()) });
    (unsafe { test_all_subsequences_108(unimplemented!()) });
    (unsafe { test_all_subsequences_108(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_173() {
    (unsafe { test_all_subsequences_109(unimplemented!()) });
    (unsafe { test_all_subsequences_110(unimplemented!()) });
    (unsafe { test_all_subsequences_111(unimplemented!()) });
    (unsafe { test_all_subsequences_112(unimplemented!()) });
    (unsafe { test_all_subsequences_113(unimplemented!()) });
    (unsafe { test_all_subsequences_114(unimplemented!()) });
    (unsafe { test_all_subsequences_112(unimplemented!()) });
    (unsafe { test_all_subsequences_115(unimplemented!()) });
    (unsafe { test_all_subsequences_115(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_174() {
    (unsafe { test_all_subsequences_116(unimplemented!()) });
    (unsafe { test_all_subsequences_117(unimplemented!()) });
    (unsafe { test_all_subsequences_118(unimplemented!()) });
    (unsafe { test_all_subsequences_119(unimplemented!()) });
    (unsafe { test_all_subsequences_120(unimplemented!()) });
    (unsafe { test_all_subsequences_121(unimplemented!()) });
    (unsafe { test_all_subsequences_119(unimplemented!()) });
    (unsafe { test_all_subsequences_122(unimplemented!()) });
    (unsafe { test_all_subsequences_122(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_175() {
    (unsafe { test_all_subsequences_123(unimplemented!()) });
    (unsafe { test_all_subsequences_124(unimplemented!()) });
    (unsafe { test_all_subsequences_125(unimplemented!()) });
    (unsafe { test_all_subsequences_126(unimplemented!()) });
    (unsafe { test_all_subsequences_127(unimplemented!()) });
    (unsafe { test_all_subsequences_128(unimplemented!()) });
    (unsafe { test_all_subsequences_126(unimplemented!()) });
    (unsafe { test_all_subsequences_129(unimplemented!()) });
    (unsafe { test_all_subsequences_129(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_176() {
    (unsafe { test_all_subsequences_130(unimplemented!()) });
    (unsafe { test_all_subsequences_131(unimplemented!()) });
    (unsafe { test_all_subsequences_132(unimplemented!()) });
    (unsafe { test_all_subsequences_133(unimplemented!()) });
    (unsafe { test_all_subsequences_134(unimplemented!()) });
    (unsafe { test_all_subsequences_135(unimplemented!()) });
    (unsafe { test_all_subsequences_133(unimplemented!()) });
    (unsafe { test_all_subsequences_136(unimplemented!()) });
    (unsafe { test_all_subsequences_136(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_177() {
    (unsafe { test_all_subsequences_137(unimplemented!()) });
    (unsafe { test_all_subsequences_138(unimplemented!()) });
    (unsafe { test_all_subsequences_139(unimplemented!()) });
    (unsafe { test_all_subsequences_140(unimplemented!()) });
    (unsafe { test_all_subsequences_141(unimplemented!()) });
    (unsafe { test_all_subsequences_142(unimplemented!()) });
    (unsafe { test_all_subsequences_140(unimplemented!()) });
    (unsafe { test_all_subsequences_143(unimplemented!()) });
    (unsafe { test_all_subsequences_143(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_178() {
    (unsafe { test_all_subsequences_144(unimplemented!()) });
    (unsafe { test_all_subsequences_145(unimplemented!()) });
    (unsafe { test_all_subsequences_146(unimplemented!()) });
    (unsafe { test_all_subsequences_147(unimplemented!()) });
    (unsafe { test_all_subsequences_148(unimplemented!()) });
    (unsafe { test_all_subsequences_149(unimplemented!()) });
    (unsafe { test_all_subsequences_147(unimplemented!()) });
    (unsafe { test_all_subsequences_150(unimplemented!()) });
    (unsafe { test_all_subsequences_150(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_179() {
    (unsafe { test_all_subsequences_151(unimplemented!()) });
    (unsafe { test_all_subsequences_152(unimplemented!()) });
    (unsafe { test_all_subsequences_153(unimplemented!()) });
    (unsafe { test_all_subsequences_154(unimplemented!()) });
    (unsafe { test_all_subsequences_155(unimplemented!()) });
    (unsafe { test_all_subsequences_156(unimplemented!()) });
    (unsafe { test_all_subsequences_154(unimplemented!()) });
    (unsafe { test_all_subsequences_157(unimplemented!()) });
    (unsafe { test_all_subsequences_157(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_180() {
    (unsafe { test_all_subsequences_158(unimplemented!()) });
    (unsafe { test_all_subsequences_159(unimplemented!()) });
    (unsafe { test_all_subsequences_160(unimplemented!()) });
    (unsafe { test_all_subsequences_161(unimplemented!()) });
    (unsafe { test_all_subsequences_162(unimplemented!()) });
    (unsafe { test_all_subsequences_163(unimplemented!()) });
    (unsafe { test_all_subsequences_161(unimplemented!()) });
    (unsafe { test_all_subsequences_164(unimplemented!()) });
    (unsafe { test_all_subsequences_164(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_sent2_181() {
    (unsafe { test_all_subsequences_165(unimplemented!()) });
    (unsafe { test_all_subsequences_166(unimplemented!()) });
    (unsafe { test_all_subsequences_167(unimplemented!()) });
    (unsafe { test_all_subsequences_168(unimplemented!()) });
    (unsafe { test_all_subsequences_169(unimplemented!()) });
    (unsafe { test_all_subsequences_170(unimplemented!()) });
    (unsafe { test_all_subsequences_168(unimplemented!()) });
    (unsafe { test_all_subsequences_171(unimplemented!()) });
    (unsafe { test_all_subsequences_171(unimplemented!()) });
}
pub unsafe fn test_iterators_in_sent1_out_182() {
    (unsafe { test_iterators_in_sent1_out_sent2_172() });
    (unsafe { test_iterators_in_sent1_out_sent2_173() });
}
pub unsafe fn test_iterators_in_sent1_out_183() {
    (unsafe { test_iterators_in_sent1_out_sent2_174() });
    (unsafe { test_iterators_in_sent1_out_sent2_175() });
}
pub unsafe fn test_iterators_in_sent1_out_184() {
    (unsafe { test_iterators_in_sent1_out_sent2_176() });
    (unsafe { test_iterators_in_sent1_out_sent2_177() });
}
pub unsafe fn test_iterators_in_sent1_out_185() {
    (unsafe { test_iterators_in_sent1_out_sent2_178() });
    (unsafe { test_iterators_in_sent1_out_sent2_179() });
}
pub unsafe fn test_iterators_in_sent1_out_186() {
    (unsafe { test_iterators_in_sent1_out_sent2_180() });
    (unsafe { test_iterators_in_sent1_out_sent2_181() });
}
pub unsafe fn test_iterators_in_sent1_187() {
    (unsafe { test_iterators_in_sent1_out_182() });
}
pub unsafe fn test_iterators_in_sent1_188() {
    (unsafe { test_iterators_in_sent1_out_183() });
}
pub unsafe fn test_iterators_in_sent1_189() {
    (unsafe { test_iterators_in_sent1_out_184() });
}
pub unsafe fn test_iterators_in_sent1_190() {
    (unsafe { test_iterators_in_sent1_out_185() });
}
pub unsafe fn test_iterators_in_sent1_191() {
    (unsafe { test_iterators_in_sent1_out_186() });
}
pub unsafe fn test_iterators_in_192() {
    {}
    (unsafe { test_iterators_in_sent1_187() });
}
pub unsafe fn test_iterators_in_193() {
    {
        (unsafe { test_iterators_in_sent1_188() });
    }
    (unsafe { test_iterators_in_sent1_189() });
}
pub unsafe fn test_iterators_in_194() {
    {
        (unsafe { test_iterators_in_sent1_190() });
    }
    (unsafe { test_iterators_in_sent1_191() });
}
pub unsafe fn test_iterators_195() {
    (unsafe { test_iterators_in_192() });
    (unsafe { test_iterators_in_193() });
    (unsafe { test_iterators_in_194() });
}
pub unsafe fn test_196() -> () {
    (unsafe { test_iterators_195() });
    {
        let in_: () = unimplemented!();
        let mut comp: () = unimplemented!();
        {
            let mut out: () = unimplemented!();
            let mut result: () = (unsafe {
                let ___first: *const () = unimplemented!();
                let ___last: *const () = unimplemented!();
                let ___result_first: *mut () = unimplemented!();
                let ___result_last: *mut () = unimplemented!();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                    ___first,
                    ___last,
                    ___result_first,
                    ___result_last,
                    unimplemented!(),
                    None,
                    None,
                )
            });
            unimplemented!();
        }
        {
            let mut out: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                    &in_,
                    &mut out,
                    unimplemented!(),
                    None,
                    None,
                )
            });
            unimplemented!();
        }
    }
    {
        let in_: () = unimplemented!();
        let mut proj1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut value: ()| -> () {
            unsafe {
                return ((value) * (-1_()));
            }
        });
        let mut proj2: FnPtr<fn(A_197) -> ()> =
            FnPtr::<fn(A_197) -> ()>::new(|mut value: A_197| -> () {
                unsafe {
                    return ((value.a) * (-1_()));
                }
            });
        {
            let mut out: () = unimplemented!();
            let mut result: () = (unsafe {
                let ___first: *const () = unimplemented!();
                let ___last: *const () = unimplemented!();
                let ___result_first: *mut A_197 = unimplemented!();
                let ___result_last: *mut A_197 = unimplemented!();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                    ___first,
                    ___last,
                    ___result_first,
                    ___result_last,
                    unimplemented!(),
                    None,
                    None,
                )
            });
            unimplemented!();
            result = (unsafe {
                let ___first: *const () = unimplemented!();
                let ___last: *const () = unimplemented!();
                let ___result_first: *mut A_197 = unimplemented!();
                let ___result_last: *mut A_197 = unimplemented!();
                (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                    ___first,
                    ___last,
                    ___result_first,
                    ___result_last,
                    unimplemented!(),
                    proj1.copy_from(),
                    proj2.copy_from(),
                )
            });
            unimplemented!();
        }
        {
            let mut out: () = unimplemented!();
            let mut result: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                    &in_,
                    &mut out,
                    unimplemented!(),
                    None,
                    None,
                )
            });
            unimplemented!();
            result = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut partial_sort_copy_32))(
                    &in_,
                    &mut out,
                    unimplemented!(),
                    proj1.copy_from(),
                    proj2.copy_from(),
                )
            });
            unimplemented!();
        }
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_197 {
    pub a: (),
}
impl A_197 {
    pub unsafe fn new_1(mut value: ()) -> Self {
        let mut this = Self { a: value };
        this
    }
    pub unsafe fn operator_cmp(&self, _a0: *const A_197) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const A_197)).a), &((*_a0).a));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const A_197) -> () {
        return (((*(self as *const A_197)).a) == ((*_a0).a));
    }
}
impl std::cmp::Ord for A_197 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { A_197::operator_cmp(self, other as *const A_197) }
    }
}
impl std::cmp::PartialOrd for A_197 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for A_197 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { A_197::operator_cmp(self, other as *const A_197) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for A_197 {}
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
    (unsafe { test_196() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
