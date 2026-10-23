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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_0(
    x: *const forward_iterator_const_char_ptr_,
    y: *const forward_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_const_char_ptr_,
    y: *const forward_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_char_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const forward_iterator_const_char_ptr_,
                other as *const forward_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_char_ptr_ {}
pub unsafe fn operator_eq_2(
    x: *const forward_iterator_char_ptr_,
    y: *const forward_iterator_char_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_3(
    x: *const forward_iterator_char_ptr_,
    y: *const forward_iterator_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct forward_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_char_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const forward_iterator_char_ptr_,
                other as *const forward_iterator_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_char_ptr_ {}
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
pub unsafe fn base_4(i: *const cpp20_input_iterator_char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_char_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_char_ptr_,
    ) -> *mut cpp20_input_iterator_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_char_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_input_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_input_iterator_char_ptr_));
    }
}
pub unsafe fn base_5(i: *const cpp20_input_iterator_const_char_ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_const_char_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_const_char_ptr_,
    ) -> *mut cpp20_input_iterator_const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_const_char_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.it_);
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
pub struct sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_4(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(&self, other: *const cpp20_input_iterator_char_ptr_) -> () {
        {}
        return ((self.base_) == (unsafe { base_4(other) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_const_char_ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_cpp20_input_iterator_const_char_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_5(it) }),
        };
        this
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
    (unsafe {
        let ___x: *mut *mut () = &mut a.ptr_;
        let ___y: *mut *mut () = &mut b.ptr_;
        swap_10(___x, ___y)
    });
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_11(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct BorrowedRange_int_ {}
const _: () = assert!(!(false), "!std::ranges::view<BorrowedRange<>>");
pub static mut enable_borrowed_range_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedRange<>>");
const _: () = assert!(true, "std::ranges::view<BorrowedView>");
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedView>");
const _: () = assert!(true, "std::ranges::view<NonBorrowedView>");
const _: () = assert!(!(false), "!std::ranges::borrowed_range<NonBorrowedView>");
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct CopyableView {
    pub view_: (),
}
impl CopyableView {
    pub unsafe fn new_1(mut ptr: *const ()) -> Self {
        let mut this = Self {
            view_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self {
            view_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn end(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn operator_eq(&self, rhs: *const CopyableView) -> () {
        return unimplemented!();
    }
}
impl std::cmp::PartialEq for CopyableView {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CopyableView::operator_eq(self, other as *const CopyableView) }
    }
}
impl std::cmp::Eq for CopyableView {}
const _: () = assert!(true, "std::ranges::forward_range<CopyableView>");
const _: () = assert!(true, "std::ranges::forward_range<const CopyableView>");
const _: () = assert!(true, "std::ranges::view<CopyableView>");
const _: () = assert!(true, "std::is_copy_constructible_v<CopyableView>");
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct ForwardView {
    pub view_: (),
}
impl ForwardView {
    pub unsafe fn new_1(mut ptr: *const ()) -> Self {
        let mut this = Self {
            view_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self {
            view_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn end(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
}
const _: () = assert!(true, "std::ranges::forward_range<ForwardView>");
const _: () = assert!(true, "std::ranges::forward_range<const ForwardView>");
const _: () = assert!(true, "std::ranges::view<ForwardView>");
const _: () = assert!(true, "!std::is_copy_constructible_v<ForwardView>");
const _: () = assert!(true, "std::is_move_constructible_v<ForwardView>");
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct ForwardDiffView {
    pub buffer_: (),
}
impl ForwardDiffView {
    pub unsafe fn new_1(mut ptr: *const ()) -> Self {
        let mut this = ForwardDiffView::new_2({ unimplemented!() });
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self {
            buffer_: unimplemented!(),
        };
        unimplemented!();
        this
    }
    pub unsafe fn begin_8(&mut self) -> forward_iterator_char_ptr_ {
        return forward_iterator_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn end_9(&mut self) -> forward_iterator_char_ptr_ {
        return forward_iterator_char_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
    }
    pub unsafe fn begin_10(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn end_11(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({
            unimplemented!().offset((unimplemented!()) as isize)
        });
    }
}
impl Default for ForwardDiffView {
    fn default() -> Self {
        unsafe {
            ForwardDiffView {
                buffer_: unimplemented!(),
            }
        }
    }
}
const _: () = assert!(true, "std::ranges::forward_range<ForwardView>");
const _: () = assert!(true, "std::ranges::forward_range<const ForwardView>");
const _: () = assert!(true, "std::ranges::view<ForwardView>");
const _: () = assert!(
    !(false),
    "!std::same_as<std::ranges::iterator_t<ForwardDiffView>, std::ranges::iterator_t<const ForwardDiffView>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct almost_forward_iterator_int_ptr_ {
    it_: *mut (),
}
impl almost_forward_iterator_int_ptr_ {}
pub unsafe fn operator_eq_15(
    x: *const almost_forward_iterator_const_char_ptr_,
    y: *const almost_forward_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_16(
    x: *const almost_forward_iterator_const_char_ptr_,
    y: *const almost_forward_iterator_const_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct almost_forward_iterator_const_char_ptr_ {
    it_: *const (),
}
impl almost_forward_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const ()) -> Self {
        let mut this = Self { it_: it };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut almost_forward_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut almost_forward_iterator_const_char_ptr_));
    }
}
impl std::cmp::PartialEq for almost_forward_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_15(
                self as *const almost_forward_iterator_const_char_ptr_,
                other as *const almost_forward_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for almost_forward_iterator_const_char_ptr_ {}
const _: () = assert!(
    !(false),
    "!std::forward_iterator<almost_forward_iterator<int*>>"
);
const _: () = assert!(true, "std::input_iterator<almost_forward_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct ForwardOnlyIfNonConstView {
    pub view_: (),
}
impl ForwardOnlyIfNonConstView {
    pub unsafe fn new_1(mut ptr: *const ()) -> Self {
        let mut this = Self {
            view_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self {
            view_: unimplemented!(),
        };
        this
    }
    pub unsafe fn begin_6(&mut self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn end_7(&mut self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn begin_8(&self) -> almost_forward_iterator_const_char_ptr_ {
        return almost_forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
    pub unsafe fn end_9(&self) -> almost_forward_iterator_const_char_ptr_ {
        return almost_forward_iterator_const_char_ptr_::new_1({ unimplemented!() });
    }
}
const _: () = assert!(
    true,
    "std::ranges::forward_range<ForwardOnlyIfNonConstView>"
);
const _: () = assert!(
    !(false),
    "!std::ranges::forward_range<const ForwardOnlyIfNonConstView>"
);
const _: () = assert!(true, "std::ranges::view<ForwardOnlyIfNonConstView>");
pub unsafe fn operator_eq_17(lhs: *const InputView, rhs: *const InputView) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct InputView {
    pub buffer_: (),
}
impl InputView {
    pub unsafe fn new_1(mut s: *const ()) -> Self {
        let mut this = InputView::new_2({ unimplemented!() });
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self {
            buffer_: unimplemented!(),
        };
        unimplemented!();
        this
    }
    pub unsafe fn begin_4(&mut self) -> cpp20_input_iterator_char_ptr_ {
        return cpp20_input_iterator_char_ptr_::new({ unimplemented!() });
    }
    pub unsafe fn end_5(&mut self) -> sentinel_wrapper_cpp20_input_iterator_char_ptr__ {
        return {
            let mut __tmp_0: cpp20_input_iterator_char_ptr_ =
                cpp20_input_iterator_char_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_input_iterator_char_ptr__::new_1({ &mut __tmp_0 })
        };
    }
    pub unsafe fn begin_6(&self) -> cpp20_input_iterator_const_char_ptr_ {
        return cpp20_input_iterator_const_char_ptr_::new({ unimplemented!() });
    }
    pub unsafe fn end_7(&self) -> sentinel_wrapper_cpp20_input_iterator_const_char_ptr__ {
        return {
            let mut __tmp_1: cpp20_input_iterator_const_char_ptr_ =
                cpp20_input_iterator_const_char_ptr_::new({
                    unimplemented!().offset((unimplemented!()) as isize)
                });
            sentinel_wrapper_cpp20_input_iterator_const_char_ptr__::new_1({ &mut __tmp_1 })
        };
    }
}
impl std::cmp::PartialEq for InputView {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const InputView, other as *const InputView) }
    }
}
impl std::cmp::Eq for InputView {}
impl Default for InputView {
    fn default() -> Self {
        unsafe {
            InputView {
                buffer_: unimplemented!(),
            }
        }
    }
}
const _: () = assert!(true, "std::ranges::input_range<InputView>");
const _: () = assert!(true, "std::ranges::input_range<const InputView>");
const _: () = assert!(true, "std::ranges::view<InputView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ForwardTinyView {
    pub c_: [(); 1],
}
impl ForwardTinyView {
    pub unsafe fn new_1(mut c: ()) -> Self {
        let mut this = Self {
            c_: [unimplemented!()],
        };
        (*this.c_.as_mut_ptr()) = c;
        this
    }
    pub unsafe fn begin(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({ self.c_.as_ptr() });
    }
    pub unsafe fn end(&self) -> forward_iterator_const_char_ptr_ {
        return forward_iterator_const_char_ptr_::new_1({
            self.c_.as_ptr().offset((1_()) as isize)
        });
    }
    pub unsafe fn size() -> () {
        return 1_();
    }
}
impl Default for ForwardTinyView {
    fn default() -> Self {
        unsafe {
            ForwardTinyView {
                c_: [unimplemented!()],
            }
        }
    }
}
const _: () = assert!(true, "std::ranges::forward_range<ForwardTinyView>");
const _: () = assert!(true, "std::ranges::view<ForwardTinyView>");
const _: () = assert!(true, "std::ranges::__tiny_range<ForwardTinyView>");
const _: () = assert!(!(false), "!std::same_as<OuterIterConst, OuterIterNonConst>");
const _: () = assert!(!(false), "!std::same_as<InnerIterConst, InnerIterNonConst>");
pub unsafe fn test_18() -> () {
    {
        const _: () = assert!(true, "std::ranges::forward_range<V>");;
        const _: () = assert!(true, "std::ranges::forward_range<const V>");;
        const _: () = assert!(true, "simple_view<V>");;
        const _: () = assert!(true, "simple_view<P>");;
        {
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(it)::iterator_concept, std::forward_iterator_tag>"
            );;
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(*(*it).begin()), const char&>"
            );;
        }
        {
            let cv: () = unimplemented!();
            let mut it: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(it)::iterator_concept, std::forward_iterator_tag>"
            );;
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(*(*it).begin()), const char&>"
            );;
        }
    }
    {
        const _: () = assert!(true, "std::ranges::forward_range<V>");;
        const _: () = assert!(true, "std::ranges::forward_range<const V>");;
        const _: () = assert!(!(false), "!simple_view<V>");;
        const _: () = assert!(!(false), "!simple_view<P>");;
        {
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(it)::iterator_concept, std::forward_iterator_tag>"
            );;
            const _: () = assert!(true, "std::is_same_v<decltype(*(*it).begin()), char&>");;
        }
        {
            let cv: () = unimplemented!();
            let mut it: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(it)::iterator_concept, std::forward_iterator_tag>"
            );;
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(*(*it).begin()), const char&>"
            );;
        }
    }
    {
        const _: () = assert!(true, "std::ranges::forward_range<V>");;
        const _: () = assert!(!(false), "!std::ranges::forward_range<const V>");;
        const _: () = assert!(!(false), "!simple_view<V>");;
        const _: () = assert!(!(false), "!simple_view<P>");;
        let mut v: () = unimplemented!();
        let mut it: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(it)::iterator_concept, std::forward_iterator_tag>"
        );;
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(*(*it).begin()), const char&>"
        );;
        const _: () = assert!(true, "ConstBeginDisabled<decltype(v)>");;
    }
    {
        const _: () = assert!(true, "std::ranges::forward_range<V>");;
        const _: () = assert!(true, "std::ranges::forward_range<const V>");;
        const _: () = assert!(true, "simple_view<V>");;
        const _: () = assert!(!(false), "!simple_view<P>");;
        {
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(it)::iterator_concept, std::forward_iterator_tag>"
            );;
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(*(*it).begin()), const char&>"
            );;
        }
        {
            let cv: () = unimplemented!();
            let mut it: () = unimplemented!();
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(it)::iterator_concept, std::forward_iterator_tag>"
            );;
            const _: () = assert!(
                true,
                "std::is_same_v<decltype(*(*it).begin()), const char&>"
            );;
        }
    }
    {
        const _: () = assert!(!(false), "!std::ranges::forward_range<V>");;
        const _: () = assert!(true, "std::ranges::forward_range<P>");;
        let mut v: () = unimplemented!();
        let mut it: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(it)::iterator_concept, std::input_iterator_tag>"
        );;
        const _: () = assert!(true, "std::is_same_v<decltype(*(*it).begin()), char&>");;
        const _: () = assert!(true, "ConstBeginDisabled<decltype(v)>");;
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
    (unsafe { test_18() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_14);
}
