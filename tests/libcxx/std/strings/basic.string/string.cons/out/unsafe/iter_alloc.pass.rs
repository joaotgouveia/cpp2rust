extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub static mut destructed_value_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
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
            operator_eq_2(
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
pub unsafe fn operator_eq_4(x: *const test_allocator_char_, y: *const test_allocator_char_) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_char__rebind_char_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_char_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_char_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_char_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl std::cmp::PartialEq for test_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const test_allocator_char_,
                other as *const test_allocator_char_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_char_ {}
impl Clone for test_allocator_char_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_char_::copy_from(self as *const test_allocator_char_) }
    }
}
impl Default for test_allocator_char_ {
    fn default() -> Self {
        unsafe {
            test_allocator_char_ {
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
pub unsafe fn operator_ne_5(
    x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    y: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
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
    ) -> *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_,
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
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_10(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_11(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_12(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_12(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_14))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_15))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_16)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_13)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_14)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_15)) = 0_();
    }
}
pub static mut outstanding_bytes_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_17(mut _a0: min_allocator_char_, mut _a1: min_allocator_char_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_char_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_char_ {}
pub unsafe fn is_string_asan_correct_18(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_19(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn test_20(mut first: *const (), mut last: *const ()) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: *const () = first;
    'loop_: while ((it) != (last)) {
        unimplemented!();
        it.prefix_inc();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_21(
    mut first: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut last: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: cpp17_input_iterator_const_char_ptr__const_char_ptr_ = first;
    'loop_: while (unsafe {
        let _x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_ = &it;
        operator_ne_5(_x, &last)
    }) {
        unimplemented!();
        (unsafe { cpp17_input_iterator_const_char_ptr__const_char_ptr_::operator_inc_5(&mut it) });
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_22(mut first: *const (), mut last: *const ()) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: *const () = first;
    'loop_: while ((it) != (last)) {
        unimplemented!();
        it.prefix_inc();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_23(
    mut first: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut last: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: cpp17_input_iterator_const_char_ptr__const_char_ptr_ = first;
    'loop_: while (unsafe {
        let _x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_ = &it;
        operator_ne_5(_x, &last)
    }) {
        unimplemented!();
        (unsafe { cpp17_input_iterator_const_char_ptr__const_char_ptr_::operator_inc_5(&mut it) });
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_24(mut first: *const (), mut last: *const (), a: *const test_allocator_char_) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: *const () = first;
    'loop_: while ((it) != (last)) {
        unimplemented!();
        it.prefix_inc();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_25(
    mut first: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut last: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    a: *const test_allocator_char_,
) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: cpp17_input_iterator_const_char_ptr__const_char_ptr_ = first;
    'loop_: while (unsafe {
        let _x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_ = &it;
        operator_ne_5(_x, &last)
    }) {
        unimplemented!();
        (unsafe { cpp17_input_iterator_const_char_ptr__const_char_ptr_::operator_inc_5(&mut it) });
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_26(mut first: *const (), mut last: *const (), a: *const min_allocator_char_) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: *const () = first;
    'loop_: while ((it) != (last)) {
        unimplemented!();
        it.prefix_inc();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_27(
    mut first: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    mut last: cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    a: *const min_allocator_char_,
) {
    let mut s2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    let mut it: cpp17_input_iterator_const_char_ptr__const_char_ptr_ = first;
    'loop_: while (unsafe {
        let _x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_ = &it;
        operator_ne_5(_x, &last)
    }) {
        unimplemented!();
        (unsafe { cpp17_input_iterator_const_char_ptr__const_char_ptr_::operator_inc_5(&mut it) });
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_28(a: *const test_allocator_char_) {
    let mut s: *const () = c"12345678901234567890123456789012345678901234567890".as_ptr();
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s;
        test_20(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s;
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_24(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((1_()) as isize);
        test_20(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((1_()) as isize);
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_24(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((10_()) as isize);
        test_20(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((10_()) as isize);
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_24(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((50_()) as isize);
        test_20(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((50_()) as isize);
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_24(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        test_21(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_25(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((1_()) as isize)
            });
        test_21(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((1_()) as isize)
            });
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_25(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((10_()) as isize)
            });
        test_21(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((10_()) as isize)
            });
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_25(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((50_()) as isize)
            });
        test_21(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((50_()) as isize)
            });
        let mut _a: test_allocator_char_ = test_allocator_char_::copy_from({ a });
        test_25(_first, _last, &mut _a)
    });
}
pub unsafe fn test_string_29(a: *const min_allocator_char_) {
    let mut s: *const () = c"12345678901234567890123456789012345678901234567890".as_ptr();
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s;
        test_22(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s;
        let mut _a: min_allocator_char_ = (*a);
        test_26(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((1_()) as isize);
        test_22(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((1_()) as isize);
        let mut _a: min_allocator_char_ = (*a);
        test_26(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((10_()) as isize);
        test_22(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((10_()) as isize);
        let mut _a: min_allocator_char_ = (*a);
        test_26(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((50_()) as isize);
        test_22(_first, _last)
    });
    (unsafe {
        let _first: *const () = s;
        let _last: *const () = s.offset((50_()) as isize);
        let mut _a: min_allocator_char_ = (*a);
        test_26(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        test_23(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let mut _a: min_allocator_char_ = (*a);
        test_27(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((1_()) as isize)
            });
        test_23(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((1_()) as isize)
            });
        let mut _a: min_allocator_char_ = (*a);
        test_27(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((10_()) as isize)
            });
        test_23(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((10_()) as isize)
            });
        let mut _a: min_allocator_char_ = (*a);
        test_27(_first, _last, &mut _a)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((50_()) as isize)
            });
        test_23(_first, _last)
    });
    (unsafe {
        let _first: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({ s });
        let _last: cpp17_input_iterator_const_char_ptr__const_char_ptr_ =
            cpp17_input_iterator_const_char_ptr__const_char_ptr_::new({
                s.offset((50_()) as isize)
            });
        let mut _a: min_allocator_char_ = (*a);
        test_27(_first, _last, &mut _a)
    });
}
pub unsafe fn test_30() -> () {
    (unsafe {
        let mut _a: test_allocator_char_ = <test_allocator_char_>::default();
        test_string_28(&mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_char_ = test_allocator_char_::new_1({ 2_() });
        test_string_28(&mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_char_ = <min_allocator_char_>::default();
        test_string_29(&mut _a)
    });
    {
        const _: () = assert!(
            true,
            "(!std::is_constructible<std::string, std::string, std::string>::value)"
        );;
        const _: () = assert!(
            true,
            "(!std::is_constructible<std::string, std::string, std::string, std::allocator<char> >::value)"
        );;
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
    (unsafe { test_30() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_13);
    std::cell::LazyCell::force(&*&raw const alloc_count_14);
    std::cell::LazyCell::force(&*&raw const dealloc_count_15);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_16);
}
