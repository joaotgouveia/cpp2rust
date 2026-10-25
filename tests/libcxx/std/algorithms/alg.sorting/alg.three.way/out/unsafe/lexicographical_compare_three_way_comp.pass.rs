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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
pub unsafe fn operator_eq_0(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
                other as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_1(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
}
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
pub unsafe fn operator_eq_2(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
}
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
pub unsafe fn operator_sub_3(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_4(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
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
}
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
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
pub unsafe fn operator_sub_5(
    mut x: contiguous_iterator_const_int_ptr_,
    mut y: contiguous_iterator_const_int_ptr_,
) -> () {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_6(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct contiguous_iterator_const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for contiguous_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const contiguous_iterator_const_int_ptr_,
                other as *const contiguous_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
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
pub static mut compare_last_digit_strong_14: std::cell::LazyCell<FnPtr<fn((), ()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                return std::cmp::Ord::cmp(&((a) % (10_())), &((b) % (10_())));
            }
        })
    });
pub static mut compare_last_digit_weak_15: std::cell::LazyCell<FnPtr<fn((), ()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                return unimplemented!();
            }
        })
    });
pub static mut compare_last_digit_partial_16: std::cell::LazyCell<FnPtr<fn((), ()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                if ((a) == (unimplemented!())) || ((b) == (unimplemented!())) {
                    return unimplemented!();
                }
                return unimplemented!();
            }
        })
    });
pub static mut compare_int_result_17: std::cell::LazyCell<FnPtr<fn((), ()) -> ()>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn((), ()) -> ()>::new(|mut a: (), mut b: ()| -> () {
            unsafe {
                return (((a) % (10_())) - ((b) % (10_())));
            }
        })
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StructWithoutCallOperator {}
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_strong)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_weak)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_partial)>"
);
const _: () = assert!(
    !(false),
    "!has_lexicographical_compare<StructWithoutCallOperator>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_int_result)>"
);
pub unsafe fn test_lexicographical_compare_18(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_19(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_20(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_21(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_22(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_23(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_24(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_25(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_26(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_27(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_28(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_29(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_30(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_31(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_32(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_33(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_34(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_35(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_36(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_37(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_38(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_39(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_40(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_41(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_42(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_43(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_44(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_45(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_46(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_47(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_48(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_49(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_50(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_51(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_52(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_53(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_54(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_55(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_56(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_57(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_58(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_59(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_60(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_61(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_62(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_63(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_64(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_65(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_66(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_67(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_68(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_69(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_70(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_71(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_72(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_73(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_74(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_75(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_76(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_77(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_78(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_79(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_80(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_81(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_82(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_83(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_84(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_85(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_86(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_87(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_88(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_89(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_90(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_91(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_92(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_93(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_94(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_95(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_96(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_97(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_98(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_99(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_100(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_101(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_102(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_103(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_104(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_105(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_106(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_107(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_108(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_109(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_110(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_111(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_112(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_113(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_114(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_115(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_116(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_117(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_118(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_119(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_120(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_121(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_122(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_123(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_124(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_125(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_126(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_127(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_128(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_129(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_130(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_131(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_132(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_133(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_134(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_135(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_136(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_137(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_138(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_139(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_140(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_141(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_142(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_143(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_144(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_145(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_146(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_147(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_148(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_149(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_150(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_151(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_152(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_153(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_154(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_155(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_156(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_157(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_158(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_159(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_160(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_161(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_162(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_163(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_164(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_165(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_166(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_167(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_168(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_169(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_170(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_171(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_172(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_173(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_174(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_175(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_176(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_177(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_178(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_179(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_180(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_181(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_182(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_183(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_184(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_185(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_186(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_187(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_188(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_189(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_190(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_191(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_192(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_193(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_194(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_195(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_196(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_197(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_198(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_199(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_200(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_201(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_202(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_203(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_204(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_205(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_206(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_207(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_208(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_209(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_210(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_211(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_212(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_213(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_214(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_215(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_216(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_217(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_218(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_219(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_220(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_221(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_222(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_223(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_224(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_225(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_226(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_227(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_228(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_229(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_230(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_231(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_232(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_233(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_234(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_235(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_236(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_237(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_238(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_239(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_240(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_241(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_242(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_243(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_244(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_245(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_246(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_247(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_248(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_249(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_250(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_251(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_252(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_253(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_254(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_255(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_256(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_257(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_258(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_259(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_260(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_261(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_262(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_263(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_264(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_265(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_266(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_267(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_268(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_269(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_270(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_271(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_272(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_273(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_274(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_275(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_276(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_277(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_278(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_279(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_280(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_281(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_282(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_283(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_284(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_285(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_286(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_287(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_288(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_289(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_290(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_291(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_292(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_293(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_294(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_295(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_296(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_297(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_298(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_299(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_300(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_301(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_302(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_303(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_304(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_305(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_306(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_307(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_308(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_309(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_310(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_311(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_312(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_313(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_314(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_315(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_316(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_317(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_318(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_lexicographical_compare_319(
    mut a: (),
    mut b: (),
    mut comp: FnPtr<fn((), ()) -> ()>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_given_iterator_types_320() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_18(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_19(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_20(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_22(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_23(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_21(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_321() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_24(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_25(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_26(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_28(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_29(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_27(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_322() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_30(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_31(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_32(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_34(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_35(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_323() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_36(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_37(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_38(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_40(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_41(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_39(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_324() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_42(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_43(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_44(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_46(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_47(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_45(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_325() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_48(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_49(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_50(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_52(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_53(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_51(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_326() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_54(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_55(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_56(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_58(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_59(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_57(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_327() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_60(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_61(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_62(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_64(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_65(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_63(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_328() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_66(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_67(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_68(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_70(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_71(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_69(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_329() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_72(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_73(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_74(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_76(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_77(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_330() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_78(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_79(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_80(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_82(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_83(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_81(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_331() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_84(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_85(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_86(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_88(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_89(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_87(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_332() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_90(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_91(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_92(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_94(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_95(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_93(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_333() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_96(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_97(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_98(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_100(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_101(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_99(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_334() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_102(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_103(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_104(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_106(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_107(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_105(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_335() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_108(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_109(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_110(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_112(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_113(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_111(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_336() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_114(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_115(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_116(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_118(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_119(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_337() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_120(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_121(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_122(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_124(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_125(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_123(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_338() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_126(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_127(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_128(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_130(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_131(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_129(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_339() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_132(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_133(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_134(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_136(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_137(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_135(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_340() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_138(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_139(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_140(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_142(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_143(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_141(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_341() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_144(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_145(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_146(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_148(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_149(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_147(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_342() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_150(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_151(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_152(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_154(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_155(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_153(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_343() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_156(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_157(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_158(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_160(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_161(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_344() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_162(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_163(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_164(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_166(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_167(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_165(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_345() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_168(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_169(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_170(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_172(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_173(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_171(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_346() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_174(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_175(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_176(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_178(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_179(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_177(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_347() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_180(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_181(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_182(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_184(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_185(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_183(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_348() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_186(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_187(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_188(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_190(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_191(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_189(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_349() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_192(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_193(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_194(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_196(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_197(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_195(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_350() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_198(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_199(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_200(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_202(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_203(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_351() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_204(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_205(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_206(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_208(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_209(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_207(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_352() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_210(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_211(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_212(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_214(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_215(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_213(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_353() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_216(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_217(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_218(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_220(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_221(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_219(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_354() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_222(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_223(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_224(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_226(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_227(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_225(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_355() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_228(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_229(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_230(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_232(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_233(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_231(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_356() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_234(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_235(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_236(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_238(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_239(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_237(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_357() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_240(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_241(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_242(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_244(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_245(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_358() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_246(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_247(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_248(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_250(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_251(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_249(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_359() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_252(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_253(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_254(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_256(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_257(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_255(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_360() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_258(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_259(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_260(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_262(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_263(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_261(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_361() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_264(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_265(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_266(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_268(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_269(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_267(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_362() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_270(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_271(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_272(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_274(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_275(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_273(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_363() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_276(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_277(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_278(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_280(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_281(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_279(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_364() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_282(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_283(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_284(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_286(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_287(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_365() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_288(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_289(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_290(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_292(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_293(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_291(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_366() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_294(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_295(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_296(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_298(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_299(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_297(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_367() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_300(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_301(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_302(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_304(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_305(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_303(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_given_iterator_types_368() {
    let mut cmp: FnPtr<fn((), ()) -> ()> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14)).copy_from();
    (unsafe {
        test_lexicographical_compare_306(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_307(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_308(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_310(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_311(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_309(
            unimplemented!(),
            unimplemented!(),
            cmp.copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_iterator_types1_369() {
    (unsafe { test_given_iterator_types_320() });
    (unsafe { test_given_iterator_types_321() });
    (unsafe { test_given_iterator_types_322() });
    (unsafe { test_given_iterator_types_323() });
    (unsafe { test_given_iterator_types_324() });
    (unsafe { test_given_iterator_types_325() });
    (unsafe { test_given_iterator_types_326() });
}
pub unsafe fn test_iterator_types1_370() {
    (unsafe { test_given_iterator_types_327() });
    (unsafe { test_given_iterator_types_328() });
    (unsafe { test_given_iterator_types_329() });
    (unsafe { test_given_iterator_types_330() });
    (unsafe { test_given_iterator_types_331() });
    (unsafe { test_given_iterator_types_332() });
    (unsafe { test_given_iterator_types_333() });
}
pub unsafe fn test_iterator_types1_371() {
    (unsafe { test_given_iterator_types_334() });
    (unsafe { test_given_iterator_types_335() });
    (unsafe { test_given_iterator_types_336() });
    (unsafe { test_given_iterator_types_337() });
    (unsafe { test_given_iterator_types_338() });
    (unsafe { test_given_iterator_types_339() });
    (unsafe { test_given_iterator_types_340() });
}
pub unsafe fn test_iterator_types1_372() {
    (unsafe { test_given_iterator_types_341() });
    (unsafe { test_given_iterator_types_342() });
    (unsafe { test_given_iterator_types_343() });
    (unsafe { test_given_iterator_types_344() });
    (unsafe { test_given_iterator_types_345() });
    (unsafe { test_given_iterator_types_346() });
    (unsafe { test_given_iterator_types_347() });
}
pub unsafe fn test_iterator_types1_373() {
    (unsafe { test_given_iterator_types_348() });
    (unsafe { test_given_iterator_types_349() });
    (unsafe { test_given_iterator_types_350() });
    (unsafe { test_given_iterator_types_351() });
    (unsafe { test_given_iterator_types_352() });
    (unsafe { test_given_iterator_types_353() });
    (unsafe { test_given_iterator_types_354() });
}
pub unsafe fn test_iterator_types1_374() {
    (unsafe { test_given_iterator_types_355() });
    (unsafe { test_given_iterator_types_356() });
    (unsafe { test_given_iterator_types_357() });
    (unsafe { test_given_iterator_types_358() });
    (unsafe { test_given_iterator_types_359() });
    (unsafe { test_given_iterator_types_360() });
    (unsafe { test_given_iterator_types_361() });
}
pub unsafe fn test_iterator_types1_375() {
    (unsafe { test_given_iterator_types_362() });
    (unsafe { test_given_iterator_types_363() });
    (unsafe { test_given_iterator_types_364() });
    (unsafe { test_given_iterator_types_365() });
    (unsafe { test_given_iterator_types_366() });
    (unsafe { test_given_iterator_types_367() });
    (unsafe { test_given_iterator_types_368() });
}
pub unsafe fn test_iterator_types_376() {
    (unsafe { test_iterator_types1_369() });
    (unsafe { test_iterator_types1_370() });
    (unsafe { test_iterator_types1_371() });
    (unsafe { test_iterator_types1_372() });
    (unsafe { test_iterator_types1_373() });
    (unsafe { test_iterator_types1_374() });
    (unsafe { test_iterator_types1_375() });
}
pub unsafe fn test_comparison_categories_377() {
    (unsafe {
        test_lexicographical_compare_312(
            unimplemented!(),
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_weak_15))
                .copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_313(
            unimplemented!(),
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_partial_16))
                .copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_314(
            unimplemented!(),
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_14))
                .copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_315(
            unimplemented!(),
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_weak_15))
                .copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_316(
            unimplemented!(),
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_partial_16))
                .copy_from(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_317(
            unimplemented!(),
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_partial_16))
                .copy_from(),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_comparator_invocation_count_378() {
    let mut compare_invocation_count: () = 0_();
    let mut compare_last_digit_counting: FnPtr<fn((), ()) -> ()> = lambda_unsafe!(
        {
            let compare_invocation_count: *mut () = &mut compare_invocation_count;
        },
        |mut a: (), mut b: ()| -> () {
            (*compare_invocation_count).prefix_inc();
            return std::cmp::Ord::cmp(&((a) % (10_())), &((b) % (10_())));
        }
    );
    compare_invocation_count = 0_();
    (unsafe {
        test_lexicographical_compare_318(
            unimplemented!(),
            unimplemented!(),
            compare_last_digit_counting.copy_from(),
            unimplemented!(),
        )
    });
    unimplemented!();
    (unsafe {
        test_lexicographical_compare_319(
            unimplemented!(),
            unimplemented!(),
            compare_last_digit_counting.copy_from(),
            unimplemented!(),
        )
    });
    unimplemented!();
}
pub unsafe fn test_proxy_iterators_379() {
    let mut vec_: () = unimplemented!();
    let mut result: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_380() -> () {
    (unsafe { test_iterator_types_376() });
    (unsafe { test_comparison_categories_377() });
    (unsafe { test_comparator_invocation_count_378() });
    (unsafe { test_proxy_iterators_379() });
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
    (unsafe { test_380() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const compare_last_digit_strong_14);
    std::cell::LazyCell::force(&*&raw const compare_last_digit_weak_15);
    std::cell::LazyCell::force(&*&raw const compare_last_digit_partial_16);
    std::cell::LazyCell::force(&*&raw const compare_int_result_17);
}
