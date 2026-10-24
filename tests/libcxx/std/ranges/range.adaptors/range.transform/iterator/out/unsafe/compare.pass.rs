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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {}
pub unsafe fn operator_add_0(
    mut x: random_access_iterator_int_ptr_,
    mut n: (),
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_eq_1(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_lt_2(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) < ((*y).it_));
}
pub unsafe fn operator_le_3(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) <= ((*y).it_));
}
pub unsafe fn operator_gt_4(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) > ((*y).it_));
}
pub unsafe fn operator_ge_5(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) >= ((*y).it_));
}
pub unsafe fn base_6(i: *const random_access_iterator_int_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
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
}
impl std::cmp::Ord for random_access_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_2(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(
                other as *const random_access_iterator_int_ptr_,
                self as *const random_access_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl Default for random_access_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_int_ptr_::new_1() }
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
pub unsafe fn operator_add_7(
    mut x: three_way_contiguous_iterator_int_ptr_,
    mut n: (),
) -> three_way_contiguous_iterator_int_ptr_ {
    (unsafe { three_way_contiguous_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return three_way_contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_cmp_8(
    x: *const three_way_contiguous_iterator_int_ptr_,
    y: *const three_way_contiguous_iterator_int_ptr_,
) -> () {
    return std::cmp::Ord::cmp(&((*x).it_), &((*y).it_));
}
pub unsafe fn operator_eq_9(
    x: *const three_way_contiguous_iterator_int_ptr_,
    y: *const three_way_contiguous_iterator_int_ptr_,
) -> () {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct three_way_contiguous_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl three_way_contiguous_iterator_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null_mut(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut three_way_contiguous_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut three_way_contiguous_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut three_way_contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::Ord for three_way_contiguous_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_8(
                self as *const three_way_contiguous_iterator_int_ptr_,
                other as *const three_way_contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::PartialOrd for three_way_contiguous_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for three_way_contiguous_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const three_way_contiguous_iterator_int_ptr_,
                other as *const three_way_contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for three_way_contiguous_iterator_int_ptr_ {}
impl Default for three_way_contiguous_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe { three_way_contiguous_iterator_int_ptr_::new_1() }
    }
}
pub unsafe fn base_10(i: *const cpp20_input_iterator_int_ptr_) -> *mut () {
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    base_: *mut (),
}
impl sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_10(it) }),
        };
        this
    }
}
pub unsafe fn operator_eq_11(
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
            operator_eq_11(
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
pub unsafe fn operator_add_12(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_13(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_12(_i, n)
    });
}
pub unsafe fn iter_swap_14(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_15(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_16(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_17(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_16(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_common_range_forward_iterator__int_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct BorrowedRange_int_ {}
const _: () = assert!(!(false), "!std::ranges::view<BorrowedRange<>>");
pub static mut enable_borrowed_range_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedRange<>>");
const _: () = assert!(true, "std::ranges::view<BorrowedView>");
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedView>");
const _: () = assert!(true, "std::ranges::view<NonBorrowedView>");
const _: () = assert!(!(false), "!std::ranges::borrowed_range<NonBorrowedView>");
pub static mut globalBuff_19: std::cell::LazyCell<[(); 8]> =
    std::cell::LazyCell::new(|| unsafe { [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_()] });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct MoveOnlyView {
    pub start_: (),
    pub ptr_: *mut (),
}
impl MoveOnlyView {
    pub unsafe fn new(mut ptr: Option<*mut ()>, mut start: Option<()>) -> Self {
        let mut ptr: *mut () = ptr.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_19)).as_mut_ptr()
        });
        let mut start: () = start.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            start_: start,
            ptr_: ptr,
        };
        this
    }
    pub unsafe fn begin(&self) -> *mut () {
        return self.ptr_.offset((self.start_) as isize);
    }
    pub unsafe fn end(&self) -> *mut () {
        return self.ptr_.offset((8_()) as isize);
    }
}
impl Default for MoveOnlyView {
    fn default() -> Self {
        unsafe { MoveOnlyView::new(None, None) }
    }
}
const _: () = assert!(true, "std::ranges::view<MoveOnlyView>");
const _: () = assert!(true, "std::ranges::contiguous_range<MoveOnlyView>");
const _: () = assert!(!(false), "!std::copyable<MoveOnlyView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CopyableView {
    pub start_: (),
}
impl CopyableView {
    pub unsafe fn new(mut start: Option<()>) -> Self {
        let mut start: () = start.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { start_: start };
        this
    }
    pub unsafe fn begin(&self) -> *mut () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_19))
            .as_mut_ptr()
            .offset((self.start_) as isize);
    }
    pub unsafe fn end(&self) -> *mut () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_19))
            .as_mut_ptr()
            .offset((8_()) as isize);
    }
}
impl Default for CopyableView {
    fn default() -> Self {
        unsafe { CopyableView::new(None) }
    }
}
const _: () = assert!(true, "std::ranges::view<CopyableView>");
const _: () = assert!(true, "std::ranges::contiguous_range<CopyableView>");
const _: () = assert!(true, "std::copyable<CopyableView>");
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ForwardView {
    pub ptr_: *mut (),
}
impl ForwardView {
    pub unsafe fn new(mut ptr: Option<*mut ()>) -> Self {
        let mut ptr: *mut () = ptr.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_19)).as_mut_ptr()
        });
        let mut this = Self { ptr_: ptr };
        this
    }
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ self.ptr_ });
    }
    pub unsafe fn end(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({ self.ptr_.offset((8_()) as isize) });
    }
}
impl Default for ForwardView {
    fn default() -> Self {
        unsafe { ForwardView::new(None) }
    }
}
const _: () = assert!(true, "std::ranges::view<ForwardView>");
const _: () = assert!(true, "std::ranges::forward_range<ForwardView>");
const _: () = assert!(!(false), "!std::ranges::view<ForwardRange>");
const _: () = assert!(true, "std::ranges::forward_range<ForwardRange>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct RandomAccessView {}
const _: () = assert!(true, "std::ranges::view<RandomAccessView>");
const _: () = assert!(true, "std::ranges::random_access_range<RandomAccessView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BidirectionalView {}
const _: () = assert!(true, "std::ranges::view<BidirectionalView>");
const _: () = assert!(true, "std::ranges::bidirectional_range<BidirectionalView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BorrowableRange {}
pub static mut enable_borrowed_range_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(!(false), "!std::ranges::view<BorrowableRange>");
const _: () = assert!(true, "std::ranges::contiguous_range<BorrowableRange>");
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowableRange>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct InputView {
    pub ptr_: *mut (),
}
impl InputView {
    pub unsafe fn new(mut ptr: Option<*mut ()>) -> Self {
        let mut ptr: *mut () = ptr.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_19)).as_mut_ptr()
        });
        let mut this = Self { ptr_: ptr };
        this
    }
    pub unsafe fn begin(&self) -> cpp20_input_iterator_int_ptr_ {
        return cpp20_input_iterator_int_ptr_::new({ self.ptr_ });
    }
    pub unsafe fn end(&self) -> sentinel_wrapper_cpp20_input_iterator_int_ptr__ {
        return {
            let mut __tmp_0: cpp20_input_iterator_int_ptr_ =
                cpp20_input_iterator_int_ptr_::new({ self.ptr_.offset((8_()) as isize) });
            sentinel_wrapper_cpp20_input_iterator_int_ptr__::new_1({ &mut __tmp_0 })
        };
    }
}
impl Default for InputView {
    fn default() -> Self {
        unsafe { InputView::new(None) }
    }
}
const _: () = assert!(true, "std::ranges::view<InputView>");
const _: () = assert!(!(false), "!std::ranges::sized_range<InputView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SizedSentinelView {
    pub count_: (),
}
impl SizedSentinelView {
    pub unsafe fn new(mut count: Option<()>) -> Self {
        let mut count: () = count.unwrap_or_else(|| unsafe { 8_() });
        let mut this = Self { count_: count };
        this
    }
    pub unsafe fn begin(&self) -> random_access_iterator_int_ptr_ {
        return random_access_iterator_int_ptr_::new_2({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_19)).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> *mut () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_19))
            .as_mut_ptr()
            .offset((self.count_) as isize);
    }
}
impl Default for SizedSentinelView {
    fn default() -> Self {
        unsafe { SizedSentinelView::new(None) }
    }
}
pub unsafe fn operator_sub_21(lhs: *const random_access_iterator_int_ptr_, mut rhs: *mut ()) -> () {
    return (((unsafe { base_6(lhs) }) as usize - rhs as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_sub_22(mut lhs: *mut (), rhs: *const random_access_iterator_int_ptr_) -> () {
    return ((lhs as usize - (unsafe { base_6(rhs) }) as usize) / ::std::mem::size_of::<()>())
        as ();
}
pub unsafe fn operator_eq_23(lhs: *const random_access_iterator_int_ptr_, mut rhs: *mut ()) -> () {
    return ((unsafe { base_6(lhs) }) == (rhs));
}
pub unsafe fn operator_eq_24(mut lhs: *mut (), rhs: *const random_access_iterator_int_ptr_) -> () {
    return ((unsafe { base_6(rhs) }) == (lhs));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SizedSentinelNotConstView {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Range {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TimesTwo {}
impl TimesTwo {
    pub unsafe fn operator_call(&self, mut x: ()) -> () {
        return ((x) * (2_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PlusOneMutable {}
impl PlusOneMutable {
    pub unsafe fn operator_call(&mut self, mut x: ()) -> () {
        return ((x) + (1_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PlusOne {}
impl PlusOne {
    pub unsafe fn operator_call(&self, mut x: ()) -> () {
        return ((x) + (1_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PlusOneMutableOverload {}
impl PlusOneMutableOverload {
    pub unsafe fn operator_call_1(&self, mut x: ()) -> () {
        return ((x) + (1_()));
    }
    pub unsafe fn operator_call_2(&mut self, x: *mut ()) -> *mut () {
        return &mut (*x).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Increment {}
impl Increment {
    pub unsafe fn operator_call(&mut self, x: *mut ()) -> *mut () {
        return &mut (*x).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IncrementRvalueRef {}
impl IncrementRvalueRef {
    pub unsafe fn operator_call(&mut self, x: *mut ()) -> *mut () {
        return &mut *(&raw mut (*unimplemented!())).cast::<()>();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PlusOneNoexcept {}
impl PlusOneNoexcept {
    pub unsafe fn operator_call(&mut self, mut x: ()) -> () {
        return ((x) + (1_()));
    }
}
pub unsafe fn test_25() -> () {
    {
        const _: () = assert!(true, "std::three_way_comparable<It>");;
        const _: () = assert!(
            true,
            "std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        let mut r : std_ranges_transform_view_std_ranges_subrange_three_way_contiguous_iterator_int_ptr___three_way_contiguous_iterator_int_ptr___std_ranges_subrange_kind_sized___PlusOne_ = unimplemented!()  ;
        let mut iter1: () = unimplemented!();
        let mut iter2: () = unimplemented!();
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
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(!(false), "!std::three_way_comparable<It>");;
        const _: () = assert!(
            !(false),
            "!std::three_way_comparable<std::ranges::iterator_t<R>>"
        );;
        let mut a: [(); 3] = [1_(), 2_(), 3_()];
        let mut r : std_ranges_transform_view_std_ranges_subrange_random_access_iterator_int_ptr___random_access_iterator_int_ptr___std_ranges_subrange_kind_sized___PlusOne_ = unimplemented!()  ;
        let mut iter1: () = unimplemented!();
        let mut iter2: () = unimplemented!();
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
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_25() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_18);
    std::cell::LazyCell::force(&*&raw const globalBuff_19);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_20);
}
