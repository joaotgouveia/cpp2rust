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
pub unsafe fn operator_ne_0(
    x: *const cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_,
    y: *const cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_ {
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
    ) -> *mut cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_
    {
        self.it_.prefix_inc();
        return & mut ( * ( self as *mut cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_  )  )  ;
    }
    pub unsafe fn move_from(
        _a0 : *mut cpp17_input_iterator_const_std_pair_int__long__ptr__const_std_pair_int__long__ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
pub unsafe fn operator_ne_1(
    x: *const cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_,
    y: *const cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_ {
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
    ) -> *mut cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_ {
        self.it_.prefix_inc();
        return & mut ( * ( self as *mut cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_  )  )  ;
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_std_pair_int__int__ptr__const_std_pair_int__int__ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
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
pub unsafe fn base_2(i: *const cpp20_input_iterator_const_std_pair_int__long__ptr_) -> *const () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_const_std_pair_int__long__ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_const_std_pair_int__long__ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_const_std_pair_int__long__ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_const_std_pair_int__long__ptr_,
    ) -> *mut cpp20_input_iterator_const_std_pair_int__long__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_const_std_pair_int__long__ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp20_input_iterator_const_std_pair_int__long__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_input_iterator_const_std_pair_int__long__ptr_));
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
pub struct sentinel_wrapper_cpp20_input_iterator_const_std_pair_int__long__ptr__ {
    base_: *const (),
}
impl sentinel_wrapper_cpp20_input_iterator_const_std_pair_int__long__ptr__ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_const_std_pair_int__long__ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const cpp20_input_iterator_const_std_pair_int__long__ptr_,
    ) -> () {
        {}
        return ((self.base_) == (unsafe { base_2(other) }));
    }
}
pub unsafe fn operator_eq_3(
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
            operator_eq_3(
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
pub unsafe fn operator_add_4(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_5(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_4(_i, n)
    });
}
pub unsafe fn iter_swap_6(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_7(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_8(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_9(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_9(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
pub static mut destructed_value_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_12(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_13(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_12(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_10));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_10));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
pub unsafe fn operator_eq_14(a: *const test_less_int_, b: *const test_less_int_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_less_int_ {
    pub data_: (),
}
impl test_less_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0_() };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const (), b: *const ()) -> () {
        return ((*a) < (*b));
    }
}
impl std::cmp::PartialEq for test_less_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const test_less_int_,
                other as *const test_less_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
impl Default for test_less_int_ {
    fn default() -> Self {
        unsafe { test_less_int_::new_1() }
    }
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
    {
        let mut mr: () = unimplemented!();
        let mut pa: () = unimplemented!();
        let mut m1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut m2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut ks: () = unimplemented!();
        let mut vs: () = unimplemented!();
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
        let mut mr1: () = unimplemented!();
        let mut mr2: () = unimplemented!();
        let mut mo: () = unimplemented!();
        let mut m: () = unimplemented!();
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
        let mut vs: () = unimplemented!();
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr1: () = unimplemented!();
        let mut mr2: () = unimplemented!();
        let mut mo: () = unimplemented!();
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut mo: () = unimplemented!();
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut __decomp_15 : std_flat_multimap_int__int__test_less_int___std_vector_int__std_pmr_polymorphic_allocator_int____std_vector_int__std_pmr_polymorphic_allocator_int____containers = ( unsafe { m . extract  ( ) } )  ;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut __decomp_16 : std_flat_multimap_int__int__test_less_int___std_vector_int__std_pmr_polymorphic_allocator_int____std_vector_int__std_pmr_polymorphic_allocator_int____containers = ( unsafe { mo . extract  ( ) } )  ;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut il: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut il: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 1_();
                std_pair_int__long_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            {
                let mut __tmp_2: () = 1_();
                let mut __tmp_3: () = 2_();
                std_pair_int__long_::new_1({ &mut __tmp_2 }, { &mut __tmp_3 })
            },
            {
                let mut __tmp_4: () = 1_();
                let mut __tmp_5: () = 3_();
                std_pair_int__long_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
            },
            {
                let mut __tmp_6: () = 2_();
                let mut __tmp_7: () = 4_();
                std_pair_int__long_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
            },
            {
                let mut __tmp_8: () = 2_();
                let mut __tmp_9: () = 5_();
                std_pair_int__long_::new_1({ &mut __tmp_8 }, { &mut __tmp_9 })
            },
            {
                let mut __tmp_10: () = 3_();
                let mut __tmp_11: () = 6_();
                std_pair_int__long_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
            },
            {
                let mut __tmp_12: () = 2_();
                let mut __tmp_13: () = 7_();
                std_pair_int__long_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
            },
            {
                let mut __tmp_14: () = 3_();
                let mut __tmp_15: () = 8_();
                std_pair_int__long_::new_1({ &mut __tmp_14 }, { &mut __tmp_15 })
            },
            {
                let mut __tmp_16: () = 3_();
                let mut __tmp_17: () = 9_();
                std_pair_int__long_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
            },
        ];
        let mut expected: [(); 9] = [
            {
                let mut __tmp_18: () = 1_();
                let mut __tmp_19: () = 1_();
                std_pair_int__long_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
            },
            {
                let mut __tmp_20: () = 1_();
                let mut __tmp_21: () = 2_();
                std_pair_int__long_::new_1({ &mut __tmp_20 }, { &mut __tmp_21 })
            },
            {
                let mut __tmp_22: () = 1_();
                let mut __tmp_23: () = 3_();
                std_pair_int__long_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
            },
            {
                let mut __tmp_24: () = 2_();
                let mut __tmp_25: () = 4_();
                std_pair_int__long_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
            },
            {
                let mut __tmp_26: () = 2_();
                let mut __tmp_27: () = 5_();
                std_pair_int__long_::new_1({ &mut __tmp_26 }, { &mut __tmp_27 })
            },
            {
                let mut __tmp_28: () = 2_();
                let mut __tmp_29: () = 7_();
                std_pair_int__long_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
            },
            {
                let mut __tmp_30: () = 3_();
                let mut __tmp_31: () = 6_();
                std_pair_int__long_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
            },
            {
                let mut __tmp_32: () = 3_();
                let mut __tmp_33: () = 8_();
                std_pair_int__long_::new_1({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 3_();
                let mut __tmp_35: () = 9_();
                std_pair_int__long_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
        ];
        {
            let mut mr: () = unimplemented!();
            let mut vm: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut mr: () = unimplemented!();
            let mut vm: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut expected: [(); 4] = [
            {
                let mut __tmp_36: () = 1_();
                let mut __tmp_37: () = 1_();
                std_pair_int__int_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
            },
            {
                let mut __tmp_38: () = 1_();
                let mut __tmp_39: () = 1_();
                std_pair_int__int_::new_1({ &mut __tmp_38 }, { &mut __tmp_39 })
            },
            {
                let mut __tmp_40: () = 2_();
                let mut __tmp_41: () = 2_();
                std_pair_int__int_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
            },
            {
                let mut __tmp_42: () = 3_();
                let mut __tmp_43: () = 1_();
                std_pair_int__int_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
            },
        ];
        let mut mr1: () = unimplemented!();
        let mut mr2: () = unimplemented!();
        let mut mo: () = unimplemented!();
        let mut m: () = unimplemented!();
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
        let mut vs: () = unimplemented!();
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr1: () = unimplemented!();
        let mut mr2: () = unimplemented!();
        let mut mo: () = unimplemented!();
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar: [(); 9] = [
            {
                let mut __tmp_44: () = 1_();
                let mut __tmp_45: () = 1_();
                std_pair_int__long_::new_1({ &mut __tmp_44 }, { &mut __tmp_45 })
            },
            {
                let mut __tmp_46: () = 1_();
                let mut __tmp_47: () = 2_();
                std_pair_int__long_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
            },
            {
                let mut __tmp_48: () = 1_();
                let mut __tmp_49: () = 3_();
                std_pair_int__long_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
            },
            {
                let mut __tmp_50: () = 2_();
                let mut __tmp_51: () = 4_();
                std_pair_int__long_::new_1({ &mut __tmp_50 }, { &mut __tmp_51 })
            },
            {
                let mut __tmp_52: () = 2_();
                let mut __tmp_53: () = 5_();
                std_pair_int__long_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
            },
            {
                let mut __tmp_54: () = 3_();
                let mut __tmp_55: () = 6_();
                std_pair_int__long_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
            },
            {
                let mut __tmp_56: () = 2_();
                let mut __tmp_57: () = 7_();
                std_pair_int__long_::new_1({ &mut __tmp_56 }, { &mut __tmp_57 })
            },
            {
                let mut __tmp_58: () = 3_();
                let mut __tmp_59: () = 8_();
                std_pair_int__long_::new_1({ &mut __tmp_58 }, { &mut __tmp_59 })
            },
            {
                let mut __tmp_60: () = 3_();
                let mut __tmp_61: () = 9_();
                std_pair_int__long_::new_1({ &mut __tmp_60 }, { &mut __tmp_61 })
            },
        ];
        let mut expected: [(); 9] = [
            {
                let mut __tmp_62: () = 1_();
                let mut __tmp_63: () = 1_();
                std_pair_int__long_::new_1({ &mut __tmp_62 }, { &mut __tmp_63 })
            },
            {
                let mut __tmp_64: () = 1_();
                let mut __tmp_65: () = 2_();
                std_pair_int__long_::new_1({ &mut __tmp_64 }, { &mut __tmp_65 })
            },
            {
                let mut __tmp_66: () = 1_();
                let mut __tmp_67: () = 3_();
                std_pair_int__long_::new_1({ &mut __tmp_66 }, { &mut __tmp_67 })
            },
            {
                let mut __tmp_68: () = 2_();
                let mut __tmp_69: () = 4_();
                std_pair_int__long_::new_1({ &mut __tmp_68 }, { &mut __tmp_69 })
            },
            {
                let mut __tmp_70: () = 2_();
                let mut __tmp_71: () = 5_();
                std_pair_int__long_::new_1({ &mut __tmp_70 }, { &mut __tmp_71 })
            },
            {
                let mut __tmp_72: () = 2_();
                let mut __tmp_73: () = 7_();
                std_pair_int__long_::new_1({ &mut __tmp_72 }, { &mut __tmp_73 })
            },
            {
                let mut __tmp_74: () = 3_();
                let mut __tmp_75: () = 6_();
                std_pair_int__long_::new_1({ &mut __tmp_74 }, { &mut __tmp_75 })
            },
            {
                let mut __tmp_76: () = 3_();
                let mut __tmp_77: () = 8_();
                std_pair_int__long_::new_1({ &mut __tmp_76 }, { &mut __tmp_77 })
            },
            {
                let mut __tmp_78: () = 3_();
                let mut __tmp_79: () = 9_();
                std_pair_int__long_::new_1({ &mut __tmp_78 }, { &mut __tmp_79 })
            },
        ];
        {
            let mut mr: () = unimplemented!();
            let mut vm: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut mr: () = unimplemented!();
            let mut vm: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut ks: () = unimplemented!();
        let mut vs: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut ks: () = unimplemented!();
        let mut vs: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut ar: [(); 4] = [
            {
                let mut __tmp_80: () = 1_();
                let mut __tmp_81: () = 1_();
                std_pair_int__int_::new_1({ &mut __tmp_80 }, { &mut __tmp_81 })
            },
            {
                let mut __tmp_82: () = 1_();
                let mut __tmp_83: () = 2_();
                std_pair_int__int_::new_1({ &mut __tmp_82 }, { &mut __tmp_83 })
            },
            {
                let mut __tmp_84: () = 1_();
                let mut __tmp_85: () = 4_();
                std_pair_int__int_::new_1({ &mut __tmp_84 }, { &mut __tmp_85 })
            },
            {
                let mut __tmp_86: () = 5_();
                let mut __tmp_87: () = 5_();
                std_pair_int__int_::new_1({ &mut __tmp_86 }, { &mut __tmp_87 })
            },
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut ar: [(); 1] = [{
            let mut __tmp_88: () = 42_();
            let mut __tmp_89: () = 42_();
            std_pair_int__int_::new_1({ &mut __tmp_88 }, { &mut __tmp_89 })
        }];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut ar: [(); 4] = [
            {
                let mut __tmp_90: () = 1_();
                let mut __tmp_91: () = 1_();
                std_pair_int__int_::new_1({ &mut __tmp_90 }, { &mut __tmp_91 })
            },
            {
                let mut __tmp_92: () = 1_();
                let mut __tmp_93: () = 2_();
                std_pair_int__int_::new_1({ &mut __tmp_92 }, { &mut __tmp_93 })
            },
            {
                let mut __tmp_94: () = 1_();
                let mut __tmp_95: () = 4_();
                std_pair_int__int_::new_1({ &mut __tmp_94 }, { &mut __tmp_95 })
            },
            {
                let mut __tmp_96: () = 5_();
                let mut __tmp_97: () = 5_();
                std_pair_int__int_::new_1({ &mut __tmp_96 }, { &mut __tmp_97 })
            },
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mr: () = unimplemented!();
        let mut vm: () = unimplemented!();
        let mut ar: [(); 1] = [{
            let mut __tmp_98: () = 42_();
            let mut __tmp_99: () = 42_();
            std_pair_int__int_::new_1({ &mut __tmp_98 }, { &mut __tmp_99 })
        }];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_10);
    std::cell::LazyCell::force(&*&raw const moved_value_11);
}
