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
pub unsafe fn base_2(i: *const forward_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
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
pub unsafe fn operator_eq_3(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_5(i: *const bidirectional_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
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
            operator_eq_3(
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
pub unsafe fn base_8(i: *const random_access_iterator_const_int_ptr_) -> *const () {
    return (*i).it_;
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
pub unsafe fn base_9(mut i: *const ()) -> *const () {
    return i;
}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_forward_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_2(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_bidirectional_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_5(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_random_access_iterator_const_int_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_random_access_iterator_const_int_ptr__ {
    pub unsafe fn new_1(it: *const random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_8(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_const_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_8(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_const_int_ptr_ {
    base_: *const (),
}
impl sentinel_wrapper_const_int_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_9((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *const ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_9((*other)) }));
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
pub struct NoLessThanOp {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotTotallyOrdered {
    pub i: (),
}
impl NotTotallyOrdered {
    pub unsafe fn operator_lt(&self, o: *const NotTotallyOrdered) -> () {
        return ((self.i) < ((*o).i));
    }
}
impl std::cmp::Ord for NotTotallyOrdered {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if NotTotallyOrdered::operator_lt(self, other as *const NotTotallyOrdered) {
                std::cmp::Ordering::Less
            } else if NotTotallyOrdered::operator_lt(other, self as *const NotTotallyOrdered) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for NotTotallyOrdered {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for NotTotallyOrdered {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(NotTotallyOrdered::operator_lt(self, other as *const NotTotallyOrdered))
                && !(NotTotallyOrdered::operator_lt(other, self as *const NotTotallyOrdered))
        }
    }
}
impl std::cmp::Eq for NotTotallyOrdered {}
const _: () = assert!(true, "HasMinElement<std::array<int, 0>>");
const _: () = assert!(!(false), "!HasMinElement<int>");
const _: () = assert!(!(false), "!HasMinElement<NoLessThanOp>");
const _: () = assert!(!(false), "!HasMinElement<NotTotallyOrdered>");
pub unsafe fn test_iterators_17(mut first: *mut (), mut last: *mut ()) {
    let mut it: *mut () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(first, last, None, None)
    });
    if ((first) != (last)) {
        let mut j: *mut () = first;
        'loop_: while ((j) != (last)) {
            unimplemented!();
            j.prefix_inc();
        }
    } else {
        unimplemented!();
    }
}
pub unsafe fn test_range_19(rng: *mut [(); 8], mut begin: *mut (), mut end: *mut ()) {
    let mut it: *mut () = (unsafe {
        let ___r: *mut [(); 8] = rng;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(___r, None, None)
    });
    if ((begin) != (end)) {
        let mut j: *mut () = begin;
        'loop_: while ((j) != (end)) {
            unimplemented!();
            j.prefix_inc();
        }
    } else {
        unimplemented!();
    }
}
pub unsafe fn test_20(mut a: (), mut expected: ()) {
    let mut first: *const () = unimplemented!();
    let mut last: *const () = unimplemented!();
    {
        let mut it: forward_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                forward_iterator_const_int_ptr_::new_1({ first }),
                forward_iterator_const_int_ptr_::new_1({ last }),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let mut it: forward_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                forward_iterator_const_int_ptr_::new_1({ first }),
                {
                    let mut __tmp_0: forward_iterator_const_int_ptr_ =
                        forward_iterator_const_int_ptr_::new_1({ last });
                    sentinel_wrapper_forward_iterator_const_int_ptr__::new_1({ &mut __tmp_0 })
                },
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: forward_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: forward_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
}
pub unsafe fn test_21(mut a: (), mut expected: ()) {
    let mut first: *const () = unimplemented!();
    let mut last: *const () = unimplemented!();
    {
        let mut it: bidirectional_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                bidirectional_iterator_const_int_ptr_::new_1({ first }),
                bidirectional_iterator_const_int_ptr_::new_1({ last }),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let mut it: bidirectional_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                bidirectional_iterator_const_int_ptr_::new_1({ first }),
                {
                    let mut __tmp_1: bidirectional_iterator_const_int_ptr_ =
                        bidirectional_iterator_const_int_ptr_::new_1({ last });
                    sentinel_wrapper_bidirectional_iterator_const_int_ptr__::new_1({ &mut __tmp_1 })
                },
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: bidirectional_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: bidirectional_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
}
pub unsafe fn test_22(mut a: (), mut expected: ()) {
    let mut first: *const () = unimplemented!();
    let mut last: *const () = unimplemented!();
    {
        let mut it: random_access_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                random_access_iterator_const_int_ptr_::new_1({ first }),
                random_access_iterator_const_int_ptr_::new_1({ last }),
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let mut it: random_access_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                random_access_iterator_const_int_ptr_::new_1({ first }),
                {
                    let mut __tmp_2: random_access_iterator_const_int_ptr_ =
                        random_access_iterator_const_int_ptr_::new_1({ last });
                    sentinel_wrapper_random_access_iterator_const_int_ptr__::new_1({ &mut __tmp_2 })
                },
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: random_access_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: random_access_iterator_const_int_ptr_ = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
}
pub unsafe fn test_23(mut a: (), mut expected: ()) {
    let mut first: *const () = unimplemented!();
    let mut last: *const () = unimplemented!();
    {
        let mut it: *const () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                first, last, None, None,
            )
        });
        unimplemented!();
    }
    {
        let mut it: *const () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                first,
                {
                    let mut __tmp_3: *const () = last;
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_3 })
                },
                None,
                None,
            )
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: *const () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
    {
        let mut range: () = unimplemented!();
        let mut it: *const () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut range, None, None)
        });
        unimplemented!();
    }
}
pub unsafe fn test_24() -> () {
    (unsafe { test_20(unimplemented!(), 0_()) });
    (unsafe { test_20(unimplemented!(), 0_()) });
    (unsafe { test_20(unimplemented!(), 0_()) });
    (unsafe { test_20(unimplemented!(), 1_()) });
    (unsafe { test_20(unimplemented!(), 1_()) });
    (unsafe { test_20(unimplemented!(), 1_()) });
    return true;
}
pub unsafe fn test_25() -> () {
    (unsafe { test_21(unimplemented!(), 0_()) });
    (unsafe { test_21(unimplemented!(), 0_()) });
    (unsafe { test_21(unimplemented!(), 0_()) });
    (unsafe { test_21(unimplemented!(), 1_()) });
    (unsafe { test_21(unimplemented!(), 1_()) });
    (unsafe { test_21(unimplemented!(), 1_()) });
    return true;
}
pub unsafe fn test_26() -> () {
    (unsafe { test_22(unimplemented!(), 0_()) });
    (unsafe { test_22(unimplemented!(), 0_()) });
    (unsafe { test_22(unimplemented!(), 0_()) });
    (unsafe { test_22(unimplemented!(), 1_()) });
    (unsafe { test_22(unimplemented!(), 1_()) });
    (unsafe { test_22(unimplemented!(), 1_()) });
    return true;
}
pub unsafe fn test_27() -> () {
    (unsafe { test_23(unimplemented!(), 0_()) });
    (unsafe { test_23(unimplemented!(), 0_()) });
    (unsafe { test_23(unimplemented!(), 0_()) });
    (unsafe { test_23(unimplemented!(), 1_()) });
    (unsafe { test_23(unimplemented!(), 1_()) });
    (unsafe { test_23(unimplemented!(), 1_()) });
    return true;
}
pub unsafe fn test_borrowed_range_and_sentinel_28() {
    let mut a: [(); 8] = [7_(), 6_(), 1_(), 3_(), 5_(), 1_(), 2_(), 4_()];
    let mut ret: *mut () = (unsafe {
        let mut ___r: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut all_29))(&mut a) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(&mut ___r, None, None)
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_comparator_30() {
    let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
    let mut ret: *mut () = (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
            &mut a,
            unimplemented!(),
            None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_projection_31() {
    let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
    {
        let mut ret: *mut () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                &mut a,
                unimplemented!(),
                FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                    unsafe {
                        return if ((i) == (5_())) { -100_() } else { i };
                    }
                }),
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ret: *mut () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
                &mut a,
                unimplemented!(),
                FnPtr::<fn(*mut ()) -> *mut ()>::new(|i: *mut ()| -> *mut () {
                    unsafe {
                        return (i);
                    }
                }),
            )
        });
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct Immobile {
    pub i: (),
}
impl Immobile {
    pub unsafe fn new(mut i_: ()) -> Self {
        let mut this = Self { i: i_ };
        this
    }
    pub unsafe fn operator_cmp(&self, _a0: *const Immobile) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(&((*(self as *const Immobile)).i), &((*_a0).i));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const Immobile) -> () {
        return (((*(self as *const Immobile)).i) == ((*_a0).i));
    }
}
impl std::cmp::Ord for Immobile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { Immobile::operator_cmp(self, other as *const Immobile) }
    }
}
impl std::cmp::PartialOrd for Immobile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Immobile {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            Immobile::operator_cmp(self, other as *const Immobile) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for Immobile {}
pub unsafe fn test_immobile_32() {
    let mut arr: [Immobile; 3] = [
        Immobile::new({ 1_() }),
        Immobile::new({ 2_() }),
        Immobile::new({ 3_() }),
    ];
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_dangling_33() {
    let mut compares: () = 0_();
    let mut projections: () = 0_();
    let mut comparator: FnPtr<fn((), ()) -> ()> = lambda_unsafe!(
        {
            let compares: *mut () = &mut compares;
        },
        |mut a: (), mut b: ()| -> () {
            (*compares).prefix_inc();
            return ((a) < (b));
        }
    );
    let mut projection: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let projections: *mut () = &mut projections;
        },
        |mut a: ()| -> () {
            (*projections).prefix_inc();
            return a;
        }
    );
    let mut ret: () = (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut min_element_18))(
            &mut ___r,
            comparator.copy_from(),
            projection.copy_from(),
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_34() -> () {
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    let mut a: [(); 8] = [7_(), 6_(), 5_(), 3_(), 4_(), 2_(), 1_(), 8_()];
    (unsafe {
        let _first: *mut () = a.as_mut_ptr();
        let _last: *mut () = a.as_mut_ptr().offset((8_()) as isize);
        test_iterators_17(_first, _last)
    });
    let mut a2: [(); 8] = [7_(), 6_(), 5_(), 3_(), 4_(), 2_(), 1_(), 8_()];
    (unsafe {
        let _rng: *mut [(); 8] = &mut a2;
        let _begin: *mut () = a2.as_mut_ptr();
        let _end: *mut () = a2.as_mut_ptr().offset((8_()) as isize);
        test_range_19(_rng, _begin, _end)
    });
    (unsafe { test_borrowed_range_and_sentinel_28() });
    (unsafe { test_comparator_30() });
    (unsafe { test_projection_31() });
    (unsafe { test_dangling_33() });
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
    (unsafe { test_34() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
