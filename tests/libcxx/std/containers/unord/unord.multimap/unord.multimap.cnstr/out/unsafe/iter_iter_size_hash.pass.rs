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
    x : *const cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_,
    y : *const cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_
{
    it_: *mut (i32, Vec<libc::c_char>),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ { pub unsafe fn new ( mut it : *mut (i32 , Vec<libc::c_char > ) , ) -> Self { let mut this = Self { it_ : it , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *mut (i32 , Vec<libc::c_char > )  { return & mut ( * self . it_   )  ;
 } pub unsafe fn operator_inc_5 ( &mut self , ) -> *mut cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ { self . it_  .prefix_inc() ;
 return & mut ( * ( self as *mut cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ , ) -> *mut cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ { self . it_   = ( * _a0 ) . it_   ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_  )  )  ;
 } }
impl std::cmp::PartialEq for cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ { fn eq(&self, other: &Self) -> bool { unsafe { !(operator_ne_0(self as *const cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_, other as *const cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_)) }}}
impl std::cmp::Eq for cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ { unsafe fn cpp_inc(&mut self) { self.operator_inc_5(); } }
impl UnsafeCppIterator<(i32 , Vec<libc::c_char > ) > for cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ { fn iter_deref(&self) -> *mut (i32 , Vec<libc::c_char > )  { unsafe { cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_::operator_deref(self) as *mut (i32 , Vec<libc::c_char > )  } } fn iter_inc(&mut self) -> &mut Self { unsafe { cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_::operator_inc_5(self); } self } }
impl UnsafeCppInputIterator<(i32 , Vec<libc::c_char > ) > for cpp17_input_iterator_std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr__std_pair_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
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
pub unsafe fn operator_eq_1(
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
            operator_eq_1(
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
pub unsafe fn operator_add_2(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_3(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_2(_i, n)
    });
}
pub unsafe fn iter_swap_4(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_5(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_6(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_7(
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
        unsafe { operator_cmp_7(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl UnsafeCppAdvance for adl_Iterator {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_14();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_16();
    }
}
impl UnsafeCppIterator<i32> for adl_Iterator {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { adl_Iterator::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            adl_Iterator::operator_inc_14(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for adl_Iterator {}
impl UnsafeCppBidirectionalIterator<i32> for adl_Iterator {
    fn iter_dec(&mut self) -> &mut Self {
        unsafe {
            adl_Iterator::operator_dec_16(self);
        }
        self
    }
}
impl UnsafeCppRandomAccessIterator<i32> for adl_Iterator {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        unsafe {
            adl_Iterator::operator_add_assign(self, n as _);
        }
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        unsafe { (adl_Iterator::operator_sub_11(self, from.clone())) as isize }
    }
}
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*const libc::c_char , new, value)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn CheckConsecutiveKeys_8(mut pos: (), mut end: (), key: (), values: *mut ()) {
    'loop_: while !(unimplemented!()) {
        assert!(pos != end);
        assert!((((*pos.iter_deref()).0) == (key)));
        assert!(unimplemented!() != unimplemented!());
        unimplemented!();
        pos.iter_inc();
    }
    assert!((pos == end) || (((*pos.iter_deref()).0) != (key)));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotConstructible {}
impl NotConstructible {}
impl std::cmp::PartialEq for NotConstructible {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const NotConstructible,
                other as *const NotConstructible,
            )
        }
    }
}
impl std::cmp::Eq for NotConstructible {}
pub unsafe fn operator_eq_9(_a0: *const NotConstructible, _a1: *const NotConstructible) -> bool {
    return true;
}
pub unsafe fn operator_eq_10(a: *const test_equal_to_int_, b: *const test_equal_to_int_) -> bool {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_equal_to_int_ {
    pub data_: i32,
}
impl test_equal_to_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0 };
        this
    }
    pub unsafe fn operator_call(&self, a: *const i32, b: *const i32) -> bool {
        return ((*a) == (*b));
    }
}
impl std::cmp::PartialEq for test_equal_to_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const test_equal_to_int_,
                other as *const test_equal_to_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_equal_to_int_ {}
impl Callable2<*const i32, *const i32, bool> for test_equal_to_int_ {
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe { test_equal_to_int_::operator_call(self, a1, a2) }
    }
}
impl Default for test_equal_to_int_ {
    fn default() -> Self {
        unsafe { test_equal_to_int_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_hash_int_ {
    data_: i32,
}
impl test_hash_int_ {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, x: *const i32) -> usize {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, c: *const test_hash_int_) -> bool {
        return ((self.data_) == ((*c).data_));
    }
}
impl std::cmp::PartialEq for test_hash_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { test_hash_int_::operator_eq(self, other as *const test_hash_int_) }
    }
}
impl std::cmp::Eq for test_hash_int_ {}
impl Callable1<*const i32, usize> for test_hash_int_ {
    fn call(&self, a1: *const i32) -> usize {
        unsafe { test_hash_int_::operator_call(self, a1) }
    }
}
impl Default for test_hash_int_ {
    fn default() -> Self {
        unsafe { test_hash_int_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: i32,
    pub throw_after: i32,
    pub count: i32,
    pub alloc_count: i32,
    pub allocated_size: i32,
    pub construct_count: i32,
    pub destroy_count: i32,
    pub copied: i32,
    pub moved: i32,
    pub converted: i32,
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        assert!(
            ((self.count) == (0)) && (!(c"clearing leaking allocator data?".as_ptr()).is_null())
        );
        self.count = 0;
        self.time_to_throw = 0;
        self.alloc_count = 0;
        self.allocated_size = 0;
        self.construct_count = 0;
        self.destroy_count = 0;
        self.throw_after = 2147483647;
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0;
        self.moved = 0;
        self.converted = 0;
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
pub static mut destructed_value_11: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_12: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_13(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_14(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_13(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: i32,
        mut id: i32,
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11));
    }
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> i32 {
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
            operator_eq_13(
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
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_15(
    x : *const test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
    y : *const test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__std_basic_string_char____rebind_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { pub unsafe fn destructor ( &mut self , ) { assert!(( ( self . data_  ) != ( (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11)) ) ) )  ;
 assert!(( ( self . id_  ) != ( (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11)) ) ) )  ;
 if ! ( ( self . stats_  ).is_null() ) { ( * self . stats_   ) . count  .prefix_dec() ;
 } self . data_   = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11))  ;
 self . id_   = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11))  ;
 } }
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_15(self as *const test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____, other as *const test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____) }}}
impl std::cmp::Eq for test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ {}
impl Default for test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn default() -> Self { unsafe { test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { data_ : 0 , id_ : 0 , stats_ : std::ptr::null_mut() , } } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_17))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_18)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_17))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_18))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_19)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_16)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_17)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_18)) = 0_usize;
    }
}
pub static mut outstanding_bytes_16: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_17: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_18: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_19: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (i32, Vec<libc::c_char>),
}
impl min_pointer_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char_____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_20(
    mut _a0 : min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
    mut _a1 : min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____
{}
impl std::cmp::PartialEq for min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_20(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ {}
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
    {
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (6_())));
        let mut eq: ((), ()) = {
            let mut __a0 = unimplemented!();
            (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a0.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a0.1),
            )
        };
        assert!(((unimplemented!()) == (2_isize)));
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_8(_pos, _end, (1 as ()), &mut s)
        });
        {
            let mut __a1 = unimplemented!();
            eq = (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.1),
            )
        };
        assert!(((unimplemented!()) == (2_isize)));
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_8(_pos, _end, (2 as ()), &mut s)
        });
        {
            let mut __a1 = unimplemented!();
            eq = (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.1),
            )
        };
        assert!(((unimplemented!()) == (1_isize)));
        let mut i: () = eq.0.clone();
        assert!((((*i.iter_deref()).0) == (3)));
        assert!(
            (*i.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1
                )
                .to_vec()
        );
        {
            let mut __a1 = unimplemented!();
            eq = (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.1),
            )
        };
        assert!(((unimplemented!()) == (1_isize)));
        i = eq.0.clone();
        assert!((((*i.iter_deref()).0) == (4)));
        assert!(
            (*i.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!()) < (1.1920929e-07_f32)));
        assert!(((unimplemented!()) == (1_f32)));
        assert!(
            (unsafe {
                let mut _c: test_hash_int_ = test_hash_int_::new({ Some(8) });
                test_hash_int_::operator_eq(&unimplemented!(), &mut _c)
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_equal_to_int_ = unimplemented!();
                let mut _b: test_equal_to_int_ = test_equal_to_int_::new_1();
                operator_eq_10(&mut _a, &mut _b)
            })
        );
        assert!(
            (unsafe {
                let mut _x : test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  = unimplemented!()  ;
                let mut _y : test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  = <test_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ >::default() ;
                operator_eq_15(&mut _x, &mut _y)
            })
        );
    }
    {
        let mut a: [(i32, Vec<libc::c_char>); 6] = [
            (
                1,
                std::slice::from_raw_parts(
                    c"one".as_ptr(),
                    libcc2rs::c_str_len(c"one".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"two".as_ptr(),
                    libcc2rs::c_str_len(c"two".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                3,
                std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                4,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                1,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
            (
                2,
                std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1,
                )
                .to_vec(),
            ),
        ];
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (6_())));
        let mut eq: ((), ()) = {
            let mut __a0 = unimplemented!();
            (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a0.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a0.1),
            )
        };
        assert!(((unimplemented!()) == (2_isize)));
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_8(_pos, _end, (1 as ()), &mut s)
        });
        {
            let mut __a1 = unimplemented!();
            eq = (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.1),
            )
        };
        assert!(((unimplemented!()) == (2_isize)));
        unimplemented!();
        unimplemented!();
        (unsafe {
            let _pos: () = <()>::from(unimplemented!());
            let _end: () = <()>::from(unimplemented!());
            CheckConsecutiveKeys_8(_pos, _end, (2 as ()), &mut s)
        });
        {
            let mut __a1 = unimplemented!();
            eq = (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.1),
            )
        };
        assert!(((unimplemented!()) == (1_isize)));
        let mut i: () = eq.0.clone();
        assert!((((*i.iter_deref()).0) == (3)));
        assert!(
            (*i.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"three".as_ptr(),
                    libcc2rs::c_str_len(c"three".as_ptr()) + 1
                )
                .to_vec()
        );
        {
            let mut __a1 = unimplemented!();
            eq = (
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.0),
                <() as CppFromUnsafe<()>>::cpp_from_move(&mut __a1.1),
            )
        };
        assert!(((unimplemented!()) == (1_isize)));
        i = eq.0.clone();
        assert!((((*i.iter_deref()).0) == (4)));
        assert!(
            (*i.iter_deref()).1
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!() as usize) == (unimplemented!() as usize)));
        assert!(((unimplemented!()) < (1.1920929e-07_f32)));
        assert!(((unimplemented!()) == (1_f32)));
        assert!(
            (unsafe {
                let mut _c: test_hash_int_ = test_hash_int_::new({ Some(8) });
                test_hash_int_::operator_eq(&unimplemented!(), &mut _c)
            })
        );
        assert!(
            (unsafe {
                let mut _a: test_equal_to_int_ = unimplemented!();
                let mut _b: test_equal_to_int_ = test_equal_to_int_::new_1();
                operator_eq_10(&mut _a, &mut _b)
            })
        );
        assert!(
            (unsafe {
                let _arg0: min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____  = unimplemented!()  ;
                operator_eq_20 ( _arg0 , <min_allocator_std_pair_const_int__std_basic_string_char__std_char_traits_char___std_allocator_char____ >::default() , )
            })
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_11);
    std::cell::LazyCell::force(&*&raw const moved_value_12);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_16);
    std::cell::LazyCell::force(&*&raw const alloc_count_17);
    std::cell::LazyCell::force(&*&raw const dealloc_count_18);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_19);
}
