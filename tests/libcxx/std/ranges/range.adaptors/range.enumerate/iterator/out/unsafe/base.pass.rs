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
pub unsafe fn base_6(mut i: *const ()) -> *const () {
    return i;
}
pub unsafe fn base_7(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
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
pub unsafe fn base_8(
    s: *const sentinel_wrapper_cpp20_input_iterator_int_ptr__,
) -> cpp20_input_iterator_int_ptr_ {
    return cpp20_input_iterator_int_ptr_::new({ (*s).base_ });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_7(it) }),
        };
        this
    }
}
pub unsafe fn base_9(
    s: *const sentinel_wrapper_forward_iterator_int_ptr__,
) -> forward_iterator_int_ptr_ {
    return forward_iterator_int_ptr_::new_1({ (*s).base_ });
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
}
pub unsafe fn base_10(
    s: *const sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__,
) -> cpp17_input_iterator_int_ptr__int_ptr_ {
    return cpp17_input_iterator_int_ptr__int_ptr_::new({ (*s).base_ });
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
}
pub unsafe fn base_11(
    s: *const sentinel_wrapper_random_access_iterator_int_ptr__,
) -> random_access_iterator_int_ptr_ {
    return random_access_iterator_int_ptr_::new_1({ (*s).base_ });
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
}
pub unsafe fn base_12(
    s: *const sentinel_wrapper_bidirectional_iterator_int_ptr__,
) -> bidirectional_iterator_int_ptr_ {
    return bidirectional_iterator_int_ptr_::new_1({ (*s).base_ });
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
}
pub unsafe fn base_13(
    s: *const sentinel_wrapper_contiguous_iterator_int_ptr__,
) -> contiguous_iterator_int_ptr_ {
    return contiguous_iterator_int_ptr_::new_1({ (*s).base_ });
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
}
pub unsafe fn base_14(s: *const sentinel_wrapper_int_ptr_) -> *mut () {
    return (*s).base_;
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
}
pub unsafe fn base_15(s: *const sentinel_wrapper_const_int_ptr_) -> *const () {
    return (*s).base_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_const_int_ptr_ {
    base_: *const (),
}
impl sentinel_wrapper_const_int_ptr_ {
    pub unsafe fn new_1(it: *const *const ()) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_eq_16(
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
            operator_eq_16(
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
pub unsafe fn operator_add_17(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_18(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_17(_i, n)
    });
}
pub unsafe fn iter_swap_19(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_20(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*unimplemented!());
}
pub unsafe fn operator_eq_21(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_22(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_22(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_21(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[derive(VaArg, FnPtrArg)]
pub struct RangeView {
    pub begin_: *mut (),
    pub end_: *mut (),
    pub wasCopyInitialized: (),
    pub wasMoveInitialized: (),
}
impl RangeView {
    pub unsafe fn copy_from(other: *const RangeView) -> Self {
        let mut this = Self {
            begin_: (*other).begin_,
            end_: (*other).end_,
            wasCopyInitialized: true,
            wasMoveInitialized: false,
        };
        this
    }
    pub unsafe fn move_from(other: *mut RangeView) -> Self {
        let mut this = Self {
            begin_: (*other).begin_,
            end_: (*other).end_,
            wasCopyInitialized: false,
            wasMoveInitialized: true,
        };
        this
    }
    pub unsafe fn begin(&self) -> *mut () {
        return self.begin_;
    }
    pub unsafe fn end(&self) -> *mut () {
        return self.end_;
    }
}
impl Clone for RangeView {
    fn clone(&self) -> Self {
        unsafe { RangeView::copy_from(self as *const RangeView) }
    }
}
impl Default for RangeView {
    fn default() -> Self {
        unsafe {
            RangeView {
                begin_: std::ptr::null_mut(),
                end_: std::ptr::null_mut(),
                wasCopyInitialized: false,
                wasMoveInitialized: false,
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::ranges::__range_with_movable_references<RangeView>"
);
const _: () = assert!(true, "std::ranges::view<RangeView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalDefaultConstructedView {}
const _: () = assert!(true, "std::ranges::view<MinimalDefaultConstructedView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalView_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ {
    it_: *mut (),
    sent_: *mut (),
}
impl MinimalView_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ {
    pub unsafe fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({ self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalView_cpp17_input_iterator_int_ptr__int_ptr___sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr___
{
    it_: *mut (),
    sent_: cpp17_input_iterator_int_ptr__int_ptr_,
}
impl MinimalView_cpp17_input_iterator_int_ptr__int_ptr___sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr___ { pub unsafe fn begin ( &self , ) -> cpp17_input_iterator_int_ptr__int_ptr_ { return cpp17_input_iterator_int_ptr__int_ptr_ :: new ( { self . it_  } , )   ;
 } pub unsafe fn end ( &self , ) -> sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ { return sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ :: new_1 ( { & self . sent_  } , )   ;
 } }
const _: () = assert!(
    true,
    "std::ranges::view<MinimalView<random_access_iterator<int*>>>"
);
const _: () = assert!(
    true,
    "std::ranges::view<MinimalView<cpp17_input_iterator<int*>, sentinel_wrapper<cpp17_input_iterator<int*>>>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotInvocable {}
const _: () = assert!(!(false), "!std::invocable<NotInvocable>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotAView {}
const _: () = assert!(!(false), "!std::ranges::view<NotAView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotAViewRange {
    pub begin_: *mut (),
    pub end_: *mut (),
}
impl NotAViewRange {
    pub unsafe fn new_1(mut b: *mut (), mut e: *mut ()) -> Self {
        let mut this = Self { begin_: b, end_: e };
        this
    }
    pub unsafe fn begin(&self) -> *mut () {
        return self.begin_;
    }
    pub unsafe fn end(&self) -> *mut () {
        return self.end_;
    }
}
const _: () = assert!(true, "std::ranges::range<NotAViewRange>");
const _: () = assert!(!(false), "!std::ranges::view<NotAViewRange>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ {
    it_: *mut (),
    sent_: *mut (),
}
impl MinimalRange_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ {
    pub unsafe fn new(
        mut it: random_access_iterator_int_ptr_,
        mut sent: random_access_iterator_int_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_3(&(*unimplemented!())) }),
            sent_: (unsafe { base_3(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({ self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___
{
    it_: *mut (),
    sent_: random_access_iterator_int_ptr_,
}
impl
    MinimalRange_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___
{
    pub unsafe fn new(
        mut it: random_access_iterator_int_ptr_,
        mut sent: sentinel_wrapper_random_access_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_3(&(*unimplemented!())) }),
            sent_: (unsafe { base_11(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_random_access_iterator_int_ptr__ {
        return sentinel_wrapper_random_access_iterator_int_ptr__::new_1({ &self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_forward_iterator_int_ptr___forward_iterator_int_ptr__ {
    it_: *mut (),
    sent_: *mut (),
}
impl MinimalRange_forward_iterator_int_ptr___forward_iterator_int_ptr__ {
    pub unsafe fn new(
        mut it: forward_iterator_int_ptr_,
        mut sent: forward_iterator_int_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_1(&(*unimplemented!())) }),
            sent_: (unsafe { base_1(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__ {
    it_: *mut (),
    sent_: *mut (),
}
impl MinimalRange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__ {
    pub unsafe fn new(
        mut it: bidirectional_iterator_int_ptr_,
        mut sent: bidirectional_iterator_int_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_2(&(*unimplemented!())) }),
            sent_: (unsafe { base_2(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__ {
    it_: *mut (),
    sent_: *mut (),
}
impl MinimalRange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__ {
    pub unsafe fn new(
        mut it: contiguous_iterator_int_ptr_,
        mut sent: contiguous_iterator_int_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_4(&(*unimplemented!())) }),
            sent_: (unsafe { base_4(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({ self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_int_ptr__int_ptr_ {
    it_: *mut (),
    sent_: *mut (),
}
impl MinimalRange_int_ptr__int_ptr_ {
    pub unsafe fn new(mut it: *mut (), mut sent: *mut ()) -> Self {
        let mut this = Self {
            it_: (unsafe { base_5((*unimplemented!())) }),
            sent_: (unsafe { base_5((*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> *mut () {
        return self.it_;
    }
    pub unsafe fn end(&self) -> *mut () {
        return self.sent_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_const_int_ptr__const_int_ptr_ {
    it_: *const (),
    sent_: *const (),
}
impl MinimalRange_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut it: *const (), mut sent: *const ()) -> Self {
        let mut this = Self {
            it_: (unsafe { base_6((*unimplemented!())) }),
            sent_: (unsafe { base_6((*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> *const () {
        return self.it_;
    }
    pub unsafe fn end(&self) -> *const () {
        return self.sent_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_cpp17_input_iterator_int_ptr__int_ptr___sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr___
{
    it_: *mut (),
    sent_: cpp17_input_iterator_int_ptr__int_ptr_,
}
impl MinimalRange_cpp17_input_iterator_int_ptr__int_ptr___sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr___ { pub unsafe fn new ( mut it : cpp17_input_iterator_int_ptr__int_ptr_ , mut sent : sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ , ) -> Self { let mut this = Self { it_ : ( unsafe { base_0 ( & ( * unimplemented!()  )  , ) } )  , sent_ : ( unsafe { base_10 ( & ( * unimplemented!()  )  , ) } )  , } ;
 this } pub unsafe fn begin ( &self , ) -> cpp17_input_iterator_int_ptr__int_ptr_ { return cpp17_input_iterator_int_ptr__int_ptr_ :: new ( { self . it_  } , )   ;
 } pub unsafe fn end ( &self , ) -> sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ { return sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ :: new_1 ( { & self . sent_  } , )   ;
 } }
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___
{
    it_: *mut (),
    sent_: cpp20_input_iterator_int_ptr_,
}
impl MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___ {
    pub unsafe fn new(
        mut it: cpp20_input_iterator_int_ptr_,
        mut sent: sentinel_wrapper_cpp20_input_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_7(&(*unimplemented!())) }),
            sent_: (unsafe { base_8(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn move_from(
        _a0 : *mut MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            sent_: cpp20_input_iterator_int_ptr_::move_from({ &mut (*_a0).sent_ }),
        };
        this
    }    pub unsafe fn move_assign ( &mut self , _a0 : *mut MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___ , ) -> *mut MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___{
        self.it_ = (*_a0).it_;
        (unsafe {
            let _arg0: *mut cpp20_input_iterator_int_ptr_ = &mut (*_a0).sent_;
            cpp20_input_iterator_int_ptr_::move_assign(&mut self.sent_, _arg0)
        });
        return & mut ( * ( self as *mut MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___  )  )  ;
    }
    pub unsafe fn begin(&self) -> cpp20_input_iterator_int_ptr_ {
        return cpp20_input_iterator_int_ptr_::new({ self.it_ });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
        return sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ {
    it_: *mut (),
    sent_: forward_iterator_int_ptr_,
}
impl MinimalRange_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ {
    pub unsafe fn new(
        mut it: forward_iterator_int_ptr_,
        mut sent: sentinel_wrapper_forward_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_1(&(*unimplemented!())) }),
            sent_: (unsafe { base_9(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_forward_iterator_int_ptr__ {
        return sentinel_wrapper_forward_iterator_int_ptr__::new_1({ &self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___
{
    it_: *mut (),
    sent_: bidirectional_iterator_int_ptr_,
}
impl
    MinimalRange_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___
{
    pub unsafe fn new(
        mut it: bidirectional_iterator_int_ptr_,
        mut sent: sentinel_wrapper_bidirectional_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_2(&(*unimplemented!())) }),
            sent_: (unsafe { base_12(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> bidirectional_iterator_int_ptr_ {
        return bidirectional_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_bidirectional_iterator_int_ptr__ {
        return sentinel_wrapper_bidirectional_iterator_int_ptr__::new_1({ &self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___
{
    it_: *mut (),
    sent_: contiguous_iterator_int_ptr_,
}
impl MinimalRange_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___ {
    pub unsafe fn new(
        mut it: contiguous_iterator_int_ptr_,
        mut sent: sentinel_wrapper_contiguous_iterator_int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (unsafe { base_4(&(*unimplemented!())) }),
            sent_: (unsafe { base_13(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> contiguous_iterator_int_ptr_ {
        return contiguous_iterator_int_ptr_::new_1({ self.it_ });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_contiguous_iterator_int_ptr__ {
        return sentinel_wrapper_contiguous_iterator_int_ptr__::new_1({ &self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_int_ptr__sentinel_wrapper_int_ptr__ {
    it_: *mut (),
    sent_: *mut (),
}
impl MinimalRange_int_ptr__sentinel_wrapper_int_ptr__ {
    pub unsafe fn new(mut it: *mut (), mut sent: sentinel_wrapper_int_ptr_) -> Self {
        let mut this = Self {
            it_: (unsafe { base_5((*unimplemented!())) }),
            sent_: (unsafe { base_14(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> *mut () {
        return self.it_;
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_int_ptr_ {
        return sentinel_wrapper_int_ptr_::new_1({ &self.sent_ });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MinimalRange_const_int_ptr__sentinel_wrapper_const_int_ptr__ {
    it_: *const (),
    sent_: *const (),
}
impl MinimalRange_const_int_ptr__sentinel_wrapper_const_int_ptr__ {
    pub unsafe fn new(mut it: *const (), mut sent: sentinel_wrapper_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (unsafe { base_6((*unimplemented!())) }),
            sent_: (unsafe { base_15(&(*unimplemented!())) }),
        };
        this
    }
    pub unsafe fn begin(&self) -> *const () {
        return self.it_;
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_const_int_ptr_ {
        return sentinel_wrapper_const_int_ptr_::new_1({ &self.sent_ });
    }
}
const _: () = assert!(
    true,
    "std::ranges::range<MinimalRange<random_access_iterator<int*>>>"
);
const _: () = assert!(
    true,
    "std::ranges::range<MinimalRange<random_access_iterator<int*>, \n                                              sentinel_wrapper<random_access_iterator<int*>>>>"
);
pub unsafe fn testBase_with_CommonRange_23() {
    const _: () = assert!(true, "std::ranges::range<CommonRange>");;
    const _: () = assert!(true, "std::ranges::common_range<CommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_forward_iterator_int_ptr___forward_iterator_int_ptr__ =
        MinimalRange_forward_iterator_int_ptr___forward_iterator_int_ptr__::new(
            { forward_iterator_int_ptr_::new_1({ arr.as_mut_ptr() }) },
            { forward_iterator_int_ptr_::new_1({ arr.as_mut_ptr().offset((4_()) as isize) }) },
        );
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const forward_iterator_int_ptr_ = (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: forward_iterator_int_ptr_ = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_CommonRange_24() {
    const _: () = assert!(true, "std::ranges::range<CommonRange>");;
    const _: () = assert!(true, "std::ranges::common_range<CommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__ =
        MinimalRange_bidirectional_iterator_int_ptr___bidirectional_iterator_int_ptr__::new(
            { bidirectional_iterator_int_ptr_::new_1({ arr.as_mut_ptr() }) },
            {
                bidirectional_iterator_int_ptr_::new_1({ arr.as_mut_ptr().offset((4_()) as isize) })
            },
        );
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const bidirectional_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: bidirectional_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_CommonRange_25() {
    const _: () = assert!(true, "std::ranges::range<CommonRange>");;
    const _: () = assert!(true, "std::ranges::common_range<CommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_random_access_iterator_int_ptr___random_access_iterator_int_ptr__ =
        MinimalRange_random_access_iterator_int_ptr___random_access_iterator_int_ptr__::new(
            { random_access_iterator_int_ptr_::new_1({ arr.as_mut_ptr() }) },
            {
                random_access_iterator_int_ptr_::new_1({ arr.as_mut_ptr().offset((4_()) as isize) })
            },
        );
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const random_access_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: random_access_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_CommonRange_26() {
    const _: () = assert!(true, "std::ranges::range<CommonRange>");;
    const _: () = assert!(true, "std::ranges::common_range<CommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__ =
        MinimalRange_contiguous_iterator_int_ptr___contiguous_iterator_int_ptr__::new(
            { contiguous_iterator_int_ptr_::new_1({ arr.as_mut_ptr() }) },
            { contiguous_iterator_int_ptr_::new_1({ arr.as_mut_ptr().offset((4_()) as isize) }) },
        );
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const contiguous_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: contiguous_iterator_int_ptr_ = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_CommonRange_27() {
    const _: () = assert!(true, "std::ranges::range<CommonRange>");;
    const _: () = assert!(true, "std::ranges::common_range<CommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_int_ptr__int_ptr_ =
        MinimalRange_int_ptr__int_ptr_::new({ arr.as_mut_ptr() }, {
            arr.as_mut_ptr().offset((4_()) as isize)
        });
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const *mut () = (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: *mut () = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_CommonRange_28() {
    const _: () = assert!(true, "std::ranges::range<CommonRange>");;
    const _: () = assert!(true, "std::ranges::common_range<CommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_const_int_ptr__const_int_ptr_ =
        MinimalRange_const_int_ptr__const_int_ptr_::new({ (arr.as_mut_ptr()).cast_const() }, {
            (arr.as_mut_ptr().offset((4_()) as isize)).cast_const()
        });
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const *const () = (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: *const () = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_29() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range : MinimalRange_cpp17_input_iterator_int_ptr__int_ptr___sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr___ = MinimalRange_cpp17_input_iterator_int_ptr__int_ptr___sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr___ :: new ( { cpp17_input_iterator_int_ptr__int_ptr_ :: new ( { arr .as_mut_ptr() } , )  } , { { let mut __tmp_0 : cpp17_input_iterator_int_ptr__int_ptr_  = cpp17_input_iterator_int_ptr__int_ptr_ :: new ( { arr .as_mut_ptr() . offset ( ( 4_() ) as isize ) } , )  ; sentinel_wrapper_cpp17_input_iterator_int_ptr__int_ptr__ :: new_1 ( { & mut __tmp_0 } , )  } } , )  ;
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const cpp17_input_iterator_int_ptr__int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: cpp17_input_iterator_int_ptr__int_ptr_ =
            (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_30() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range : MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___ = MinimalRange_cpp20_input_iterator_int_ptr___sentinel_wrapper_cpp20_input_iterator_int_ptr___ :: new ( { cpp20_input_iterator_int_ptr_ :: new ( { arr .as_mut_ptr() } , )  } , { { let mut __tmp_1 : cpp20_input_iterator_int_ptr_  = cpp20_input_iterator_int_ptr_ :: new ( { arr .as_mut_ptr() . offset ( ( 4_() ) as isize ) } , )  ; sentinel_wrapper_cpp20_input_iterator_int_ptr__ :: new_1 ( { & mut __tmp_1 } , )  } } , )  ;
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const cpp20_input_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: cpp20_input_iterator_int_ptr_ = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_31() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range : MinimalRange_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ = MinimalRange_forward_iterator_int_ptr___sentinel_wrapper_forward_iterator_int_ptr___ :: new ( { forward_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() } , )  } , { { let mut __tmp_2 : forward_iterator_int_ptr_  = forward_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() . offset ( ( 4_() ) as isize ) } , )  ; sentinel_wrapper_forward_iterator_int_ptr__ :: new_1 ( { & mut __tmp_2 } , )  } } , )  ;
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const forward_iterator_int_ptr_ = (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: forward_iterator_int_ptr_ = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_32() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range : MinimalRange_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___ = MinimalRange_bidirectional_iterator_int_ptr___sentinel_wrapper_bidirectional_iterator_int_ptr___ :: new ( { bidirectional_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() } , )  } , { { let mut __tmp_3 : bidirectional_iterator_int_ptr_  = bidirectional_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() . offset ( ( 4_() ) as isize ) } , )  ; sentinel_wrapper_bidirectional_iterator_int_ptr__ :: new_1 ( { & mut __tmp_3 } , )  } } , )  ;
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const bidirectional_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: bidirectional_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_33() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range : MinimalRange_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___ = MinimalRange_random_access_iterator_int_ptr___sentinel_wrapper_random_access_iterator_int_ptr___ :: new ( { random_access_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() } , )  } , { { let mut __tmp_4 : random_access_iterator_int_ptr_  = random_access_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() . offset ( ( 4_() ) as isize ) } , )  ; sentinel_wrapper_random_access_iterator_int_ptr__ :: new_1 ( { & mut __tmp_4 } , )  } } , )  ;
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const random_access_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: random_access_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_34() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range : MinimalRange_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___ = MinimalRange_contiguous_iterator_int_ptr___sentinel_wrapper_contiguous_iterator_int_ptr___ :: new ( { contiguous_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() } , )  } , { { let mut __tmp_5 : contiguous_iterator_int_ptr_  = contiguous_iterator_int_ptr_ :: new_1 ( { arr .as_mut_ptr() . offset ( ( 4_() ) as isize ) } , )  ; sentinel_wrapper_contiguous_iterator_int_ptr__ :: new_1 ( { & mut __tmp_5 } , )  } } , )  ;
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const contiguous_iterator_int_ptr_ =
            (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: contiguous_iterator_int_ptr_ = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_35() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_int_ptr__sentinel_wrapper_int_ptr__ =
        MinimalRange_int_ptr__sentinel_wrapper_int_ptr__::new({ arr.as_mut_ptr() }, {
            {
                let mut __tmp_6: *mut () = arr.as_mut_ptr().offset((4_()) as isize);
                sentinel_wrapper_int_ptr_::new_1({ &mut __tmp_6 })
            }
        });
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const *mut () = (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: *mut () = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testBase_with_NonCommonRange_36() {
    const _: () = assert!(!(false), "!std::ranges::common_range<NonCommonRange>");;
    let mut arr: [(); 4] = [94_(), 1_(), 2_(), 82_()];
    let mut range: MinimalRange_const_int_ptr__sentinel_wrapper_const_int_ptr__ =
        MinimalRange_const_int_ptr__sentinel_wrapper_const_int_ptr__::new(
            { (arr.as_mut_ptr()).cast_const() },
            {
                {
                    let mut __tmp_7: *const () =
                        (arr.as_mut_ptr().offset((4_()) as isize)).cast_const();
                    sentinel_wrapper_const_int_ptr_::new_1({ &mut __tmp_7 })
                }
            },
        );
    let mut ev: () = unimplemented!();
    let mut it: () = unimplemented!();
    {
        let resultIt: *const *const () = (unsafe { (*unimplemented!()).base_5() });
        const _: () = assert!(true, "noexcept((it.base()))");;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut resultIt: *const () = (unsafe { (*unimplemented!()).base_6() });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_37() -> () {
    (unsafe { testBase_with_CommonRange_23() });
    (unsafe { testBase_with_CommonRange_24() });
    (unsafe { testBase_with_CommonRange_25() });
    (unsafe { testBase_with_CommonRange_26() });
    (unsafe { testBase_with_CommonRange_27() });
    (unsafe { testBase_with_CommonRange_28() });
    (unsafe { testBase_with_NonCommonRange_29() });
    (unsafe { testBase_with_NonCommonRange_30() });
    (unsafe { testBase_with_NonCommonRange_31() });
    (unsafe { testBase_with_NonCommonRange_32() });
    (unsafe { testBase_with_NonCommonRange_33() });
    (unsafe { testBase_with_NonCommonRange_34() });
    (unsafe { testBase_with_NonCommonRange_35() });
    (unsafe { testBase_with_NonCommonRange_36() });
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
    (unsafe { test_37() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
