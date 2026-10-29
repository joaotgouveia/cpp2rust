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
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        assert!(!((*other).moved_from_));
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        assert!(!((*other).moved_from_));
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
    it_: *mut i32,
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
    it_: *mut i32,
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
pub unsafe fn operator_ne_0(
    x: *const cpp17_input_iterator_const_bool_ptr__const_bool_ptr_,
    y: *const cpp17_input_iterator_const_bool_ptr__const_bool_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_bool_ptr__const_bool_ptr_ {
    it_: *const bool,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_bool_ptr__const_bool_ptr_ {
    pub unsafe fn new(mut it: *const bool) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut bool {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_bool_ptr__const_bool_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_bool_ptr__const_bool_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_bool_ptr__const_bool_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_bool_ptr__const_bool_ptr_,
    ) -> *mut cpp17_input_iterator_const_bool_ptr__const_bool_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_bool_ptr__const_bool_ptr_));
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_const_bool_ptr_,
    y: *const forward_iterator_const_bool_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_bool_ptr_ {
    it_: *const bool,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_bool_ptr_ {
    pub unsafe fn new_1(mut it: *const bool) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut bool {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_bool_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_bool_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_bool_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_bool_ptr_,
    ) -> *mut forward_iterator_const_bool_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_bool_ptr_));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
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
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut i32,
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
    pub increments: usize,
    pub decrements: usize,
    pub zero_moves: usize,
    pub equal_cmps: usize,
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub unsafe fn operator_eq_2(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> bool {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut i32,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
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
pub unsafe fn operator_add_3(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_4(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_3(_i, n)
    });
}
pub unsafe fn iter_swap_5(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_6(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_7(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_8(
    lhs: *const adl_Iterator,
    rhs: *const adl_Iterator,
) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut i32,
    iter_moves_: *mut i32,
    iter_swaps_: *mut i32,
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut i32, mut iter_moves: *mut i32, mut iter_swaps: *mut i32) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut i32, iter_moves: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut i32, iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> i32 {
        assert!(!(self.iter_moves_).is_null());
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> i32 {
        assert!(!(self.iter_swaps_).is_null());
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64
            as isize);
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_8(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self as *const adl_Iterator, other as *const adl_Iterator) }
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
    it_: *mut *mut i32,
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> bool {
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
    pub unsafe fn new(mut what: *const libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_12)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_9)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)) = 0_usize;
    }
}
pub static mut outstanding_bytes_9: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_10: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_11: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_12: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_bool_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn new_1(mut _a0: min_allocator_bool_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_unsigned_long_ {}
impl min_allocator_unsigned_long_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__ {
        return min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__::new_1({
            (unsafe { <std_allocator_unsigned_long_>::default().allocate(n) })
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        (unsafe {
            let ___p: *mut u64 = p.ptr_;
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
impl min_allocator_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_bool_ {}
impl explicit_allocator_bool_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for explicit_allocator_bool_ {
    fn default() -> Self {
        unsafe { explicit_allocator_bool_::new() }
    }
}
impl explicit_allocator_unsigned_long_ {
    pub unsafe fn new_1(mut _a0: explicit_allocator_bool_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_unsigned_long_ {}
impl explicit_allocator_unsigned_long_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut u64 {
        return ((unsafe { <std_allocator_unsigned_long_>::default().allocate(n) }) as *mut u64);
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: usize) {
        (unsafe {
            let ___p: *mut u64 = p;
            let ___n: usize = n;
            <std_allocator_unsigned_long_>::default().deallocate(___p, ___n)
        });
    }
}
pub unsafe fn tests_13() -> bool {
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        let mut a: [bool; 5] = [(1 != 0), (0 != 0), (0 != 0), (1 != 0), (1 != 0)];
        let N: u32 = (((::std::mem::size_of::<[bool; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as u32);
        let mut i: UnsafeRandomAccessIterator<bool> = {
            let __off = (unsafe { v.cbegin() })
                .iter_add(10_i64)
                .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it = cpp17_input_iterator_const_bool_ptr__const_bool_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
            while __it
                != cpp17_input_iterator_const_bool_ptr__const_bool_ptr_::new({
                    (a.as_mut_ptr().offset((N) as isize)).cast_const()
                })
            {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
        };
        assert!(((v.len()) == (((100_u32).wrapping_add(N)) as usize)));
        assert!(i == v.begin().iter_add(10_i64));
        let mut j: usize = <u64>::default();
        j = 0_usize;
        'loop_: while ((j) < (10_usize)) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (N as usize)) {
            assert!((((v[(j)] as bool) as i32) == (a[(k)] as i32)));
            {
                j.prefix_inc();
                k.prefix_inc()
            };
        }
        'loop_: while ((j) < (v.len())) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        let mut a: [bool; 5] = [(1 != 0), (0 != 0), (0 != 0), (1 != 0), (1 != 0)];
        let N: u32 = (((::std::mem::size_of::<[bool; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as u32);
        let mut i: UnsafeRandomAccessIterator<bool> = {
            let __off = (unsafe { v.cbegin() })
                .iter_add(10_i64)
                .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it =
                forward_iterator_const_bool_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
            while __it
                != forward_iterator_const_bool_ptr_::new_1({
                    (a.as_mut_ptr().offset((N) as isize)).cast_const()
                })
            {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
        };
        assert!(((v.len()) == (((100_u32).wrapping_add(N)) as usize)));
        assert!(i == v.begin().iter_add(10_i64));
        let mut j: i32 = <i32>::default();
        j = 0;
        'loop_: while ((j) < (10)) {
            assert!((((v[(j as usize)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (N as usize)) {
            assert!((((v[(j as usize)] as bool) as i32) == (a[(k)] as i32)));
            {
                j.prefix_inc();
                k.prefix_inc()
            };
        }
        'loop_: while ((j) < (105)) {
            assert!((((v[(j as usize)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        'loop_: while ((v.len()) < (v.capacity())) {
            {
                let a0_clone = false.clone();
                v.push(a0_clone)
            };
        }
        let mut sz: usize = v.len();
        let mut a: [bool; 5] = [(1 != 0), (0 != 0), (0 != 0), (1 != 0), (1 != 0)];
        let N: u32 = (((::std::mem::size_of::<[bool; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as u32);
        let mut i: UnsafeRandomAccessIterator<bool> = {
            let __off = (unsafe { v.cbegin() })
                .iter_add(10_i64)
                .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it =
                forward_iterator_const_bool_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
            while __it
                != forward_iterator_const_bool_ptr_::new_1({
                    (a.as_mut_ptr().offset((N) as isize)).cast_const()
                })
            {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
        };
        assert!(((v.len()) == ((sz).wrapping_add((N as usize)))));
        assert!(i == v.begin().iter_add(10_i64));
        let mut j: usize = <u64>::default();
        j = 0_usize;
        'loop_: while ((j) < (10_usize)) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (N as usize)) {
            assert!((((v[(j)] as bool) as i32) == (a[(k)] as i32)));
            {
                j.prefix_inc();
                k.prefix_inc()
            };
        }
        'loop_: while ((j) < (v.len())) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        'loop_: while ((v.len()) < (v.capacity())) {
            {
                let a0_clone = false.clone();
                v.push(a0_clone)
            };
        }
        v.pop();
        v.pop();
        v.pop();
        let mut sz: usize = v.len();
        let mut a: [bool; 5] = [(1 != 0), (0 != 0), (0 != 0), (1 != 0), (1 != 0)];
        let N: u32 = (((::std::mem::size_of::<[bool; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as u32);
        let mut i: UnsafeRandomAccessIterator<bool> = {
            let __off = (unsafe { v.cbegin() })
                .iter_add(10_i64)
                .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it =
                forward_iterator_const_bool_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
            while __it
                != forward_iterator_const_bool_ptr_::new_1({
                    (a.as_mut_ptr().offset((N) as isize)).cast_const()
                })
            {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
        };
        assert!(((v.len()) == ((sz).wrapping_add((N as usize)))));
        assert!(i == v.begin().iter_add(10_i64));
        let mut j: usize = <u64>::default();
        j = 0_usize;
        'loop_: while ((j) < (10_usize)) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (N as usize)) {
            assert!((((v[(j)] as bool) as i32) == (a[(k)] as i32)));
            {
                j.prefix_inc();
                k.prefix_inc()
            };
        }
        'loop_: while ((j) < (v.len())) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        let mut a: [bool; 5] = [(1 != 0), (0 != 0), (0 != 0), (1 != 0), (1 != 0)];
        let N: u32 = (((::std::mem::size_of::<[bool; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as u32);
        let mut i: UnsafeRandomAccessIterator<bool> = {
            let __off = (unsafe { v.cbegin() })
                .iter_add(10_i64)
                .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it = cpp17_input_iterator_const_bool_ptr__const_bool_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
            while __it
                != cpp17_input_iterator_const_bool_ptr__const_bool_ptr_::new({
                    (a.as_mut_ptr().offset((N) as isize)).cast_const()
                })
            {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
        };
        assert!(((v.len()) == (((100_u32).wrapping_add(N)) as usize)));
        assert!(i == v.begin().iter_add(10_i64));
        let mut j: usize = <u64>::default();
        j = 0_usize;
        'loop_: while ((j) < (10_usize)) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (N as usize)) {
            assert!((((v[(j)] as bool) as i32) == (a[(k)] as i32)));
            {
                j.prefix_inc();
                k.prefix_inc()
            };
        }
        'loop_: while ((j) < (v.len())) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        let mut a: [bool; 5] = [(1 != 0), (0 != 0), (0 != 0), (1 != 0), (1 != 0)];
        let N: u32 = (((::std::mem::size_of::<[bool; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as u32);
        let mut i: UnsafeRandomAccessIterator<bool> = {
            let __off = (unsafe { v.cbegin() })
                .iter_add(10_i64)
                .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it = cpp17_input_iterator_const_bool_ptr__const_bool_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
            while __it
                != cpp17_input_iterator_const_bool_ptr__const_bool_ptr_::new({
                    (a.as_mut_ptr().offset((N) as isize)).cast_const()
                })
            {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
        };
        assert!(((v.len()) == (((100_u32).wrapping_add(N)) as usize)));
        assert!(i == v.begin().iter_add(10_i64));
        let mut j: usize = <u64>::default();
        j = 0_usize;
        'loop_: while ((j) < (10_usize)) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (N as usize)) {
            assert!((((v[(j)] as bool) as i32) == (a[(k)] as i32)));
            {
                j.prefix_inc();
                k.prefix_inc()
            };
        }
        'loop_: while ((j) < (v.len())) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
    }
    {
        let mut v: Vec<bool> = (0..(100_usize) as usize)
            .map(|_| <bool>::default())
            .collect::<Vec<_>>();
        let mut a: [bool; 5] = [(1 != 0), (0 != 0), (0 != 0), (1 != 0), (1 != 0)];
        let N: u32 = (((::std::mem::size_of::<[bool; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as u32);
        let mut i: UnsafeRandomAccessIterator<bool> = {
            let __off = (unsafe { v.cbegin() })
                .iter_add(10_i64)
                .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
                as usize;
            let mut __tmp = Vec::new();
            let mut __it =
                forward_iterator_const_bool_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
            while __it
                != forward_iterator_const_bool_ptr_::new_1({
                    (a.as_mut_ptr().offset((N) as isize)).cast_const()
                })
            {
                __tmp.push((*__it.iter_deref()).clone());
                __it.iter_inc();
            }
            v.splice(__off..__off, __tmp);
            UnsafeRandomAccessIterator::Vec(v.as_mut_ptr().add(__off))
        };
        assert!(((v.len()) == (((100_u32).wrapping_add(N)) as usize)));
        assert!(i == v.begin().iter_add(10_i64));
        let mut j: usize = <u64>::default();
        j = 0_usize;
        'loop_: while ((j) < (10_usize)) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
        }
        let mut k: usize = 0_usize;
        'loop_: while ((k) < (N as usize)) {
            assert!((((v[(j)] as bool) as i32) == (a[(k)] as i32)));
            {
                j.prefix_inc();
                k.prefix_inc()
            };
        }
        'loop_: while ((j) < (v.len())) {
            assert!((((v[(j)] as bool) as i32) == (0)));
            j.prefix_inc();
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_13() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_unsigned_long__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_9);
    std::cell::LazyCell::force(&*&raw const alloc_count_10);
    std::cell::LazyCell::force(&*&raw const dealloc_count_11);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_12);
}
