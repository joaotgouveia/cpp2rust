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
pub unsafe fn base_0(i: *const cpp17_input_iterator_int_ptr__int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn base_1(i: *const forward_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
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
pub unsafe fn base_2(i: *const bidirectional_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn base_3(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
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
pub unsafe fn base_4(i: *const contiguous_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn base_5(mut i: *mut ()) -> *mut () {
    return i;
}
pub unsafe fn base_6(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
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
pub struct sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ {
    pub unsafe fn new_1(it: *const cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_0(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp17_input_iterator_int_ptr__int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_0(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_6(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_forward_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_1(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const forward_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_1(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_bidirectional_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const bidirectional_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_2(other) }));
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
            base_: (unsafe { base_3(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const random_access_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_3(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_contiguous_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_contiguous_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_4(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const contiguous_iterator_int_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_4(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {
    pub unsafe fn new_1(it: *const *mut ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5((*it)) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const *mut ()) -> () {
        {}
        return ((self.base_) == (unsafe { base_5((*other)) }));
    }
}
pub unsafe fn operator_eq_7(
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
            operator_eq_7(
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
pub unsafe fn operator_add_8(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_9(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_8(_i, n)
    });
}
pub unsafe fn iter_swap_10(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_11(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_12(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_13(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_13(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_iterators_14(
    mut begin1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut begin2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end1 });
        let ___first2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_16(
    mut begin1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut begin2: cpp20_input_iterator_int_ptr_,
    mut end2: cpp20_input_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end1 });
        let ___first2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_17(
    mut begin1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut begin2: forward_iterator_int_ptr_,
    mut end2: forward_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end1 });
        let ___first2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_18(
    mut begin1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut begin2: bidirectional_iterator_int_ptr_,
    mut end2: bidirectional_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end1 });
        let ___first2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_19(
    mut begin1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut begin2: random_access_iterator_int_ptr_,
    mut end2: random_access_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end1 });
        let ___first2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_20(
    mut begin1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut begin2: contiguous_iterator_int_ptr_,
    mut end2: contiguous_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end1 });
        let ___first2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_21(
    mut begin1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end1: cpp17_input_iterator_int_ptr__int_ptr_,
    mut begin2: *mut (),
    mut end2: *mut (),
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end1 });
        let ___first2: *mut () = begin2;
        let ___last2: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end2 as *mut ()) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_22(
    mut begin1: cpp20_input_iterator_int_ptr_,
    mut end1: cpp20_input_iterator_int_ptr_,
    mut begin2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_23(
    mut begin1: cpp20_input_iterator_int_ptr_,
    mut end1: cpp20_input_iterator_int_ptr_,
    mut begin2: cpp20_input_iterator_int_ptr_,
    mut end2: cpp20_input_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_24(
    mut begin1: cpp20_input_iterator_int_ptr_,
    mut end1: cpp20_input_iterator_int_ptr_,
    mut begin2: forward_iterator_int_ptr_,
    mut end2: forward_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_25(
    mut begin1: cpp20_input_iterator_int_ptr_,
    mut end1: cpp20_input_iterator_int_ptr_,
    mut begin2: bidirectional_iterator_int_ptr_,
    mut end2: bidirectional_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_26(
    mut begin1: cpp20_input_iterator_int_ptr_,
    mut end1: cpp20_input_iterator_int_ptr_,
    mut begin2: random_access_iterator_int_ptr_,
    mut end2: random_access_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_27(
    mut begin1: cpp20_input_iterator_int_ptr_,
    mut end1: cpp20_input_iterator_int_ptr_,
    mut begin2: contiguous_iterator_int_ptr_,
    mut end2: contiguous_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_28(
    mut begin1: cpp20_input_iterator_int_ptr_,
    mut end1: cpp20_input_iterator_int_ptr_,
    mut begin2: *mut (),
    mut end2: *mut (),
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: *mut () = begin2;
        let ___last2: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end2 as *mut ()) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_29(
    mut begin1: forward_iterator_int_ptr_,
    mut end1: forward_iterator_int_ptr_,
    mut begin2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_30(
    mut begin1: forward_iterator_int_ptr_,
    mut end1: forward_iterator_int_ptr_,
    mut begin2: cpp20_input_iterator_int_ptr_,
    mut end2: cpp20_input_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_31(
    mut begin1: forward_iterator_int_ptr_,
    mut end1: forward_iterator_int_ptr_,
    mut begin2: forward_iterator_int_ptr_,
    mut end2: forward_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_32(
    mut begin1: forward_iterator_int_ptr_,
    mut end1: forward_iterator_int_ptr_,
    mut begin2: bidirectional_iterator_int_ptr_,
    mut end2: bidirectional_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_33(
    mut begin1: forward_iterator_int_ptr_,
    mut end1: forward_iterator_int_ptr_,
    mut begin2: random_access_iterator_int_ptr_,
    mut end2: random_access_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_34(
    mut begin1: forward_iterator_int_ptr_,
    mut end1: forward_iterator_int_ptr_,
    mut begin2: contiguous_iterator_int_ptr_,
    mut end2: contiguous_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_35(
    mut begin1: forward_iterator_int_ptr_,
    mut end1: forward_iterator_int_ptr_,
    mut begin2: *mut (),
    mut end2: *mut (),
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: *mut () = begin2;
        let ___last2: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end2 as *mut ()) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_36(
    mut begin1: bidirectional_iterator_int_ptr_,
    mut end1: bidirectional_iterator_int_ptr_,
    mut begin2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_37(
    mut begin1: bidirectional_iterator_int_ptr_,
    mut end1: bidirectional_iterator_int_ptr_,
    mut begin2: cpp20_input_iterator_int_ptr_,
    mut end2: cpp20_input_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_38(
    mut begin1: bidirectional_iterator_int_ptr_,
    mut end1: bidirectional_iterator_int_ptr_,
    mut begin2: forward_iterator_int_ptr_,
    mut end2: forward_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_39(
    mut begin1: bidirectional_iterator_int_ptr_,
    mut end1: bidirectional_iterator_int_ptr_,
    mut begin2: bidirectional_iterator_int_ptr_,
    mut end2: bidirectional_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_40(
    mut begin1: bidirectional_iterator_int_ptr_,
    mut end1: bidirectional_iterator_int_ptr_,
    mut begin2: random_access_iterator_int_ptr_,
    mut end2: random_access_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_41(
    mut begin1: bidirectional_iterator_int_ptr_,
    mut end1: bidirectional_iterator_int_ptr_,
    mut begin2: contiguous_iterator_int_ptr_,
    mut end2: contiguous_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_42(
    mut begin1: bidirectional_iterator_int_ptr_,
    mut end1: bidirectional_iterator_int_ptr_,
    mut begin2: *mut (),
    mut end2: *mut (),
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: *mut () = begin2;
        let ___last2: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end2 as *mut ()) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_43(
    mut begin1: random_access_iterator_int_ptr_,
    mut end1: random_access_iterator_int_ptr_,
    mut begin2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_44(
    mut begin1: random_access_iterator_int_ptr_,
    mut end1: random_access_iterator_int_ptr_,
    mut begin2: cpp20_input_iterator_int_ptr_,
    mut end2: cpp20_input_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_45(
    mut begin1: random_access_iterator_int_ptr_,
    mut end1: random_access_iterator_int_ptr_,
    mut begin2: forward_iterator_int_ptr_,
    mut end2: forward_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_46(
    mut begin1: random_access_iterator_int_ptr_,
    mut end1: random_access_iterator_int_ptr_,
    mut begin2: bidirectional_iterator_int_ptr_,
    mut end2: bidirectional_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_47(
    mut begin1: random_access_iterator_int_ptr_,
    mut end1: random_access_iterator_int_ptr_,
    mut begin2: random_access_iterator_int_ptr_,
    mut end2: random_access_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_48(
    mut begin1: random_access_iterator_int_ptr_,
    mut end1: random_access_iterator_int_ptr_,
    mut begin2: contiguous_iterator_int_ptr_,
    mut end2: contiguous_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_49(
    mut begin1: random_access_iterator_int_ptr_,
    mut end1: random_access_iterator_int_ptr_,
    mut begin2: *mut (),
    mut end2: *mut (),
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: *mut () = begin2;
        let ___last2: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end2 as *mut ()) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_50(
    mut begin1: contiguous_iterator_int_ptr_,
    mut end1: contiguous_iterator_int_ptr_,
    mut begin2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_51(
    mut begin1: contiguous_iterator_int_ptr_,
    mut end1: contiguous_iterator_int_ptr_,
    mut begin2: cpp20_input_iterator_int_ptr_,
    mut end2: cpp20_input_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_52(
    mut begin1: contiguous_iterator_int_ptr_,
    mut end1: contiguous_iterator_int_ptr_,
    mut begin2: forward_iterator_int_ptr_,
    mut end2: forward_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_53(
    mut begin1: contiguous_iterator_int_ptr_,
    mut end1: contiguous_iterator_int_ptr_,
    mut begin2: bidirectional_iterator_int_ptr_,
    mut end2: bidirectional_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_54(
    mut begin1: contiguous_iterator_int_ptr_,
    mut end1: contiguous_iterator_int_ptr_,
    mut begin2: random_access_iterator_int_ptr_,
    mut end2: random_access_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_55(
    mut begin1: contiguous_iterator_int_ptr_,
    mut end1: contiguous_iterator_int_ptr_,
    mut begin2: contiguous_iterator_int_ptr_,
    mut end2: contiguous_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_56(
    mut begin1: contiguous_iterator_int_ptr_,
    mut end1: contiguous_iterator_int_ptr_,
    mut begin2: *mut (),
    mut end2: *mut (),
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin1 });
        let ___last1: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end1 });
        let ___first2: *mut () = begin2;
        let ___last2: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end2 as *mut ()) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_57(
    mut begin1: *mut (),
    mut end1: *mut (),
    mut begin2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut end2: cpp17_input_iterator_int_ptr__int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: *mut () = begin1;
        let ___last1: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end1 as *mut ()) });
        let ___first2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ =
            sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_58(
    mut begin1: *mut (),
    mut end1: *mut (),
    mut begin2: cpp20_input_iterator_int_ptr_,
    mut end2: cpp20_input_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: *mut () = begin1;
        let ___last1: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end1 as *mut ()) });
        let ___first2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_cpp20_input_iterator_int_ptr__ =
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_59(
    mut begin1: *mut (),
    mut end1: *mut (),
    mut begin2: forward_iterator_int_ptr_,
    mut end2: forward_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: *mut () = begin1;
        let ___last1: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end1 as *mut ()) });
        let ___first2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_forward_iterator_int_ptr__ =
            sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_60(
    mut begin1: *mut (),
    mut end1: *mut (),
    mut begin2: bidirectional_iterator_int_ptr_,
    mut end2: bidirectional_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: *mut () = begin1;
        let ___last1: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end1 as *mut ()) });
        let ___first2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_bidirectional_iterator_int_ptr__ =
            sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_61(
    mut begin1: *mut (),
    mut end1: *mut (),
    mut begin2: random_access_iterator_int_ptr_,
    mut end2: random_access_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: *mut () = begin1;
        let ___last1: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end1 as *mut ()) });
        let ___first2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_random_access_iterator_int_ptr__ =
            sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_62(
    mut begin1: *mut (),
    mut end1: *mut (),
    mut begin2: contiguous_iterator_int_ptr_,
    mut end2: contiguous_iterator_int_ptr_,
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: *mut () = begin1;
        let ___last1: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end1 as *mut ()) });
        let ___first2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::move_from({ &mut begin2 });
        let ___last2: sentinel_wrapper_contiguous_iterator_int_ptr__ =
            sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &end2 });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iterators_63(
    mut begin1: *mut (),
    mut end1: *mut (),
    mut begin2: *mut (),
    mut end2: *mut (),
    mut expected1: *mut (),
    mut expected2: *mut (),
) {
    let mut ret: () = (unsafe {
        let ___first1: *mut () = begin1;
        let ___last1: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end1 as *mut ()) });
        let ___first2: *mut () = begin2;
        let ___last2: sentinel_wrapper_int_ptr_ =
            sentinel_wrapper_int_ptr_::new_1({ &(end2 as *mut ()) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
            ___first1, ___last1, ___first2, ___last2, None, None, None,
        )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_iters_64() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_14(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_65() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_16(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_66() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_17(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_67() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_18(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_68() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_19(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_69() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_20(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_70() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: *mut () = b.as_mut_ptr();
        let _end2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_21(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_71() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_22(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_72() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_23(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_73() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_24(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_74() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_25(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_75() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_26(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_76() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_27(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_77() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr() });
        let _end1: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: *mut () = b.as_mut_ptr();
        let _end2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_28(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_78() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_29(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_79() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_30(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_80() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_31(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_81() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_32(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_82() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_33(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_83() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_34(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_84() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: *mut () = b.as_mut_ptr();
        let _end2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_35(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_85() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_36(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_86() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_37(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_87() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_38(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_88() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_39(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_89() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_40(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_90() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_41(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_91() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: *mut () = b.as_mut_ptr();
        let _end2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_42(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_92() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_43(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_93() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_44(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_94() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_45(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_95() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_46(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_96() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_47(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_97() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_48(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_98() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: *mut () = b.as_mut_ptr();
        let _end2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_49(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_99() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_50(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_100() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_51(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_101() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_52(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_102() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_53(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_103() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_54(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_104() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_55(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_105() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr() });
        let _end1: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ a.as_mut_ptr().offset((5_()) as isize) });
        let _begin2: *mut () = b.as_mut_ptr();
        let _end2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_56(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_106() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: *mut () = a.as_mut_ptr();
        let _end1: *mut () = a.as_mut_ptr().offset((5_()) as isize);
        let _begin2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_57(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_107() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: *mut () = a.as_mut_ptr();
        let _end1: *mut () = a.as_mut_ptr().offset((5_()) as isize);
        let _begin2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr() });
        let _end2: cpp20_input_iterator_int_ptr_ =
            cpp20_input_iterator_int_ptr_::new({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_58(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_108() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: *mut () = a.as_mut_ptr();
        let _end1: *mut () = a.as_mut_ptr().offset((5_()) as isize);
        let _begin2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_59(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_109() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: *mut () = a.as_mut_ptr();
        let _end1: *mut () = a.as_mut_ptr().offset((5_()) as isize);
        let _begin2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_60(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_110() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: *mut () = a.as_mut_ptr();
        let _end1: *mut () = a.as_mut_ptr().offset((5_()) as isize);
        let _begin2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_61(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_111() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: *mut () = a.as_mut_ptr();
        let _end1: *mut () = a.as_mut_ptr().offset((5_()) as isize);
        let _begin2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr() });
        let _end2: contiguous_iterator_int_ptr_ =
            contiguous_iterator_int_ptr_::new_1({ b.as_mut_ptr().offset((5_()) as isize) });
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_62(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_iters_112() {
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
    (unsafe {
        let _begin1: *mut () = a.as_mut_ptr();
        let _end1: *mut () = a.as_mut_ptr().offset((5_()) as isize);
        let _begin2: *mut () = b.as_mut_ptr();
        let _end2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
        let _expected1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
        let _expected2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
        test_iterators_63(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
    });
}
pub unsafe fn test_113() -> () {
    (unsafe { test_iters_64() });
    (unsafe { test_iters_65() });
    (unsafe { test_iters_66() });
    (unsafe { test_iters_67() });
    (unsafe { test_iters_68() });
    (unsafe { test_iters_69() });
    (unsafe { test_iters_70() });
    (unsafe { test_iters_71() });
    (unsafe { test_iters_72() });
    (unsafe { test_iters_73() });
    (unsafe { test_iters_74() });
    (unsafe { test_iters_75() });
    (unsafe { test_iters_76() });
    (unsafe { test_iters_77() });
    (unsafe { test_iters_78() });
    (unsafe { test_iters_79() });
    (unsafe { test_iters_80() });
    (unsafe { test_iters_81() });
    (unsafe { test_iters_82() });
    (unsafe { test_iters_83() });
    (unsafe { test_iters_84() });
    (unsafe { test_iters_85() });
    (unsafe { test_iters_86() });
    (unsafe { test_iters_87() });
    (unsafe { test_iters_88() });
    (unsafe { test_iters_89() });
    (unsafe { test_iters_90() });
    (unsafe { test_iters_91() });
    (unsafe { test_iters_92() });
    (unsafe { test_iters_93() });
    (unsafe { test_iters_94() });
    (unsafe { test_iters_95() });
    (unsafe { test_iters_96() });
    (unsafe { test_iters_97() });
    (unsafe { test_iters_98() });
    (unsafe { test_iters_99() });
    (unsafe { test_iters_100() });
    (unsafe { test_iters_101() });
    (unsafe { test_iters_102() });
    (unsafe { test_iters_103() });
    (unsafe { test_iters_104() });
    (unsafe { test_iters_105() });
    (unsafe { test_iters_106() });
    (unsafe { test_iters_107() });
    (unsafe { test_iters_108() });
    (unsafe { test_iters_109() });
    (unsafe { test_iters_110() });
    (unsafe { test_iters_111() });
    (unsafe { test_iters_112() });
    {
        let mut a: () = unimplemented!();
        let mut b: () = unimplemented!();
        let mut ret: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                &mut a, &mut b, None, None, None,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut b: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
        let mut r: () = (unsafe {
            let ___first1: *mut () = a.as_mut_ptr();
            let ___last1: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_0: *mut () = a.as_mut_ptr().offset((5_()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_0 })
            };
            let ___first2: *mut () = b.as_mut_ptr();
            let ___last2: sentinel_wrapper_int_ptr_ = {
                let mut __tmp_1: *mut () = b.as_mut_ptr().offset((5_()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_1 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                ___first1, ___last1, ___first2, ___last2, None, None, None,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut a: [(); 3] = [1_(), 2_(), 3_()];
            let mut b: [(); 2] = [1_(), 2_()];
            (unsafe {
                let _begin1: *mut () = a.as_mut_ptr();
                let _end1: *mut () = a.as_mut_ptr().offset((3_()) as isize);
                let _begin2: *mut () = b.as_mut_ptr();
                let _end2: *mut () = b.as_mut_ptr().offset((2_()) as isize);
                let _expected1: *mut () = a.as_mut_ptr().offset((2_()) as isize);
                let _expected2: *mut () = b.as_mut_ptr().offset((2_()) as isize);
                test_iterators_63(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
            });
            let mut ret: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    &mut a, &mut b, None, None, None,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 2] = [1_(), 2_()];
            let mut b: [(); 3] = [1_(), 2_(), 3_()];
            (unsafe {
                let _begin1: *mut () = a.as_mut_ptr();
                let _end1: *mut () = a.as_mut_ptr().offset((2_()) as isize);
                let _begin2: *mut () = b.as_mut_ptr();
                let _end2: *mut () = b.as_mut_ptr().offset((3_()) as isize);
                let _expected1: *mut () = a.as_mut_ptr().offset((2_()) as isize);
                let _expected2: *mut () = b.as_mut_ptr().offset((2_()) as isize);
                test_iterators_63(_begin1, _end1, _begin2, _end2, _expected1, _expected2)
            });
            let mut ret: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    &mut a, &mut b, None, None, None,
                )
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut r1: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let mut r2: [(); 5] = [1_(), 2_(), 3_(), 5_(), 4_()];
        {
            let mut ret: () = (unsafe {
                let mut ___r2: () =
                    (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut all_114))(&mut r2) });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    &mut r1, &mut ___r2, None, None, None,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ret: () = (unsafe {
                let mut ___r1: () =
                    (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut all_114))(&mut r1) });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    &mut ___r1, &mut r2, None, None, None,
                )
            });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ret: () = (unsafe {
                let mut ___r1: () =
                    (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut all_114))(&mut r1) });
                let mut ___r2: () =
                    (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut all_114))(&mut r2) });
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    &mut ___r1, &mut ___r2, None, None, None,
                )
            });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut a: [(); 4] = [1_(), 2_(), 3_(), 4_()];
        let mut b: [(); 5] = [1_(), 2_(), 4_(), 8_(), 16_()];
        let mut __decomp_115: () = (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                &mut a, &mut b, None, None, None,
            )
        });
        unimplemented!();
        unimplemented!();
        let mut __decomp_116: () = (unsafe {
            let ___first1: *mut () = a.as_mut_ptr();
            let ___last1: *mut () = a.as_mut_ptr().offset((4_()) as isize);
            let ___first2: *mut () = b.as_mut_ptr();
            let ___last2: *mut () = b.as_mut_ptr().offset((5_()) as isize);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                ___first1, ___last1, ___first2, ___last2, None, None, None,
            )
        });
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut b: [(); 8] = [6_(), 5_(), 8_(), 2_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                let ___first1: *mut () = a.as_mut_ptr();
                let ___last1: *mut () = a.as_mut_ptr().offset((8_()) as isize);
                let ___first2: *mut () = b.as_mut_ptr();
                let ___last2: *mut () = b.as_mut_ptr().offset((8_()) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    ___first1,
                    ___last1,
                    ___first2,
                    ___last2,
                    unimplemented!(),
                    None,
                    None,
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut b: [(); 8] = [6_(), 5_(), 8_(), 2_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    &mut a,
                    &mut b,
                    unimplemented!(),
                    None,
                    None,
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut b: [(); 8] = [6_(), 5_(), 8_(), 2_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    &mut a,
                    &mut b,
                    unimplemented!(),
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return if ((i) == (5_())) { 100_() } else { i };
                        }
                    }),
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return if ((i) == (5_())) { -100_() } else { i };
                        }
                    }),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                let ___r1: *mut [(); 8] = &mut a;
                let ___r2: *mut [(); 8] = &mut a;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    ___r1,
                    ___r2,
                    unimplemented!(),
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (1.01_f64));
                        }
                    }),
                    lambda_unsafe!(
                        {
                            let c: () = 0_();
                        },
                        |mut i: ()| -> () {
                            return if ((c.postfix_inc()) < (5_())) {
                                ((i) * (1.02_f64))
                            } else {
                                i
                            };
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut b: [(); 8] = [6_(), 5_(), 8_(), 2_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                let ___first1: *mut () = a.as_mut_ptr();
                let ___last1: *mut () = a.as_mut_ptr().offset((8_()) as isize);
                let ___first2: *mut () = b.as_mut_ptr();
                let ___last2: *mut () = b.as_mut_ptr().offset((8_()) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    ___first1,
                    ___last1,
                    ___first2,
                    ___last2,
                    unimplemented!(),
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return if ((i) == (5_())) { 100_() } else { i };
                        }
                    }),
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return if ((i) == (5_())) { -100_() } else { i };
                        }
                    }),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                let ___first1: *mut () = a.as_mut_ptr();
                let ___last1: *mut () = a.as_mut_ptr().offset((8_()) as isize);
                let ___first2: *mut () = a.as_mut_ptr();
                let ___last2: *mut () = a.as_mut_ptr().offset((8_()) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    ___first1,
                    ___last1,
                    ___first2,
                    ___last2,
                    unimplemented!(),
                    FnPtr::<fn(()) -> ()>::new(|mut i: ()| -> () {
                        unsafe {
                            return ((i) * (1.01_f64));
                        }
                    }),
                    lambda_unsafe!(
                        {
                            let c: () = 0_();
                        },
                        |mut i: ()| -> () {
                            return if ((c.postfix_inc()) < (5_())) {
                                ((i) * (1.02_f64))
                            } else {
                                i
                            };
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut pred_count: () = 0_();
            let mut proj1_count: () = 0_();
            let mut proj2_count: () = 0_();
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                let ___r1: *mut [(); 8] = &mut a;
                let ___r2: *mut [(); 8] = &mut a;
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    ___r1,
                    ___r2,
                    lambda_unsafe!(
                        {
                            let pred_count: *mut () = &mut pred_count;
                        },
                        |mut lhs: (), mut rhs: ()| -> () {
                            (*pred_count).prefix_inc();
                            return ((lhs) == (rhs));
                        }
                    ),
                    lambda_unsafe!(
                        {
                            let proj1_count: *mut () = &mut proj1_count;
                        },
                        |mut i: ()| -> () {
                            (*proj1_count).prefix_inc();
                            return i;
                        }
                    ),
                    lambda_unsafe!(
                        {
                            let proj2_count: *mut () = &mut proj2_count;
                        },
                        |mut i: ()| -> () {
                            (*proj2_count).prefix_inc();
                            return i;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut pred_count: () = 0_();
            let mut proj1_count: () = 0_();
            let mut proj2_count: () = 0_();
            let mut a: [(); 8] = [7_(), 6_(), 9_(), 3_(), 5_(), 1_(), 2_(), 4_()];
            let mut ret: () = (unsafe {
                let ___first1: *mut () = a.as_mut_ptr();
                let ___last1: *mut () = a.as_mut_ptr().offset((8_()) as isize);
                let ___first2: *mut () = a.as_mut_ptr();
                let ___last2: *mut () = a.as_mut_ptr().offset((8_()) as isize);
                (*std::cell::LazyCell::force_mut(&mut *&raw mut mismatch_15))(
                    ___first1,
                    ___last1,
                    ___first2,
                    ___last2,
                    lambda_unsafe!(
                        {
                            let pred_count: *mut () = &mut pred_count;
                        },
                        |mut lhs: (), mut rhs: ()| -> () {
                            (*pred_count).prefix_inc();
                            return ((lhs) == (rhs));
                        }
                    ),
                    lambda_unsafe!(
                        {
                            let proj1_count: *mut () = &mut proj1_count;
                        },
                        |mut i: ()| -> () {
                            (*proj1_count).prefix_inc();
                            return i;
                        }
                    ),
                    lambda_unsafe!(
                        {
                            let proj2_count: *mut () = &mut proj2_count;
                        },
                        |mut i: ()| -> () {
                            (*proj2_count).prefix_inc();
                            return i;
                        }
                    ),
                )
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
    (unsafe { test_113() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
