extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_output_iterator_int_ptr_,
    ) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_int_ptr__int_ptr_,
    ) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_0(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_int_ptr_,
    ) -> *mut forward_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
pub unsafe fn operator_eq_2(
    x: *const forward_iterator_User_S_ptr_,
    y: *const forward_iterator_User_S_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_3(
    x: *const forward_iterator_User_S_ptr_,
    y: *const forward_iterator_User_S_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_User_S_ptr_ {
    it_: *mut User_S,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_User_S_ptr_ {
    pub unsafe fn new_1(mut it: *mut User_S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_User_S_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_User_S_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_User_S_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_User_S_ptr_,
    ) -> *mut forward_iterator_User_S_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_User_S_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_User_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const forward_iterator_User_S_ptr_,
                other as *const forward_iterator_User_S_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_User_S_ptr_ {}
pub unsafe fn operator_eq_4(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_5(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_const_int_ptr_,
    ) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
pub unsafe fn operator_eq_6(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_7(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_int_ptr_,
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
pub unsafe fn operator_eq_8(
    x: *const random_access_iterator_User_S_ptr_,
    y: *const random_access_iterator_User_S_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_9(
    x: *const random_access_iterator_User_S_ptr_,
    y: *const random_access_iterator_User_S_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_User_S_ptr_ {
    it_: *mut User_S,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_User_S_ptr_ {
    pub unsafe fn new_1(mut it: *mut User_S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_User_S_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_User_S_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_User_S_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_User_S_ptr_,
    ) -> *mut random_access_iterator_User_S_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_User_S_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_User_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_8(
                self as *const random_access_iterator_User_S_ptr_,
                other as *const random_access_iterator_User_S_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_User_S_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
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
    pub unsafe fn move_from(_a0: *mut cpp20_random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_random_access_iterator_int_ptr_,
    ) -> *mut cpp20_random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_random_access_iterator_int_ptr_));
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
pub unsafe fn operator_eq_10(
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
            operator_eq_10(
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
pub unsafe fn operator_add_11(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_12(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_11(_i, n)
    });
}
pub unsafe fn iter_swap_13(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_14(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_15(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_16(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_16(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_15(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct MySearcherC {}
impl MySearcherC {
    pub unsafe fn operator_call_1_int_ptr(&self, mut b: *mut (), mut e: *mut ()) -> () {
        return unimplemented!();
    }
}
pub unsafe fn test_constexpr_17() -> () {
    let mut ia: [(); 4] = [0_(), 1_(), 2_(), 3_()];
    let mut ib: [(); 4] = [0_(), 1_(), 5_(), 3_()];
    let mut ic: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    return (((unimplemented!()) == (ic.as_mut_ptr().offset((3_()) as isize)))
        && ((unimplemented!()) == (unimplemented!())))
        && ((unimplemented!()) == (unimplemented!()));
}
pub static mut searcher_called_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MySearcher {}
impl MySearcher {
    pub unsafe fn operator_call_1_int_ptr(&self, mut b: *mut (), mut e: *mut ()) -> () {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut searcher_called_18)).prefix_inc();
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct User_S {
    pub x_: (),
}
impl User_S {
    pub unsafe fn new(mut x: ()) -> Self {
        let mut this = Self { x_: x };
        this
    }
}
impl std::cmp::PartialEq for User_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_19(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for User_S {}
pub unsafe fn operator_eq_19(mut lhs: User_S, mut rhs: User_S) -> () {
    return ((lhs.x_) == (rhs.x_));
}
pub unsafe fn test_20() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_21() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_22() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_23() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_24() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_25() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_26() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_27() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn test_28() {
    let mut ia: [(); 6] = [0_(), 1_(), 2_(), 3_(), 4_(), 5_()];
    let sa: () =
        (::std::mem::size_of::<[(); 6]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
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
    let mut ib: [(); 12] = [
        0_(),
        1_(),
        2_(),
        0_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sb: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ic: [(); 1] = [1_()];
    unimplemented!();
    let mut id: [(); 2] = [1_(), 2_()];
    unimplemented!();
    let mut ie: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut ig: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    unimplemented!();
    let mut ih: [(); 12] = [
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        2_(),
        3_(),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
    ];
    let sh: () =
        (::std::mem::size_of::<[(); 12]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ii: [(); 3] = [1_(), 1_(), 2_()];
    unimplemented!();
    let mut ij: [(); 16] = [
        0_(),
        1_(),
        0_(),
        0_(),
        1_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
        1_(),
        1_(),
        1_(),
        0_(),
        0_(),
    ];
    let sj: () =
        (::std::mem::size_of::<[(); 16]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut ik: [(); 10] = [0_(), 0_(), 0_(), 0_(), 1_(), 1_(), 1_(), 1_(), 0_(), 0_()];
    let sk: () =
        (::std::mem::size_of::<[(); 10]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    unimplemented!();
}
pub unsafe fn adl_test_29() {
    let mut ua: [User_S; 1] = [User_S::new({ 1_() })];
    unimplemented!();
}
pub unsafe fn adl_test_30() {
    let mut ua: [User_S; 1] = [User_S::new({ 1_() })];
    unimplemented!();
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
    (unsafe { test_20() });
    (unsafe { test_21() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { adl_test_29() });
    (unsafe { adl_test_30() });
    {
        const _: () = assert!(
            true,
            "(std::is_same<RI, decltype(std::search(RI(), RI(), MySearcher()))>::value)"
        );;
        let mut it: *mut () = std::ptr::null_mut();
        unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const searcher_called_18);
}
