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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_0(
    x: *const random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__,
    y: *const random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__,
) -> bool {
    return (*x).it_ == (*y).it_;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
    pub unsafe fn new_1(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_dec_8(
        &mut self,
    ) -> *mut random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
        self.it_.iter_dec();
        return &mut (*(self
            as *mut random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__));
    }
    pub unsafe fn move_from(
        _a0: *mut random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_.clone(),
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__,
    ) -> *mut random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
        self.it_ = (*_a0).it_.clone();
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self
            as *mut random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__));
    }
}
impl std::cmp::PartialEq for random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__,
                other as *const random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ {}
pub unsafe fn operator_sub_1(
    mut x: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    mut n: isize,
) -> random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    (unsafe {
        random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: operator_sub_assign ( &mut x  , n , )
    });
    return random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ :: move_from ( { & mut x } , )   ;
}
pub unsafe fn operator_sub_2(
    mut x: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    mut y: random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
) -> isize {
    return x.it_.iter_distance(&(y.it_).clone().into()) as i64;
}
pub unsafe fn operator_ne_3(
    x: *const random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    y: *const random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
) -> bool {
    return (*x).it_ != (*y).it_;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
    pub unsafe fn new_1(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_dec_8(
        &mut self,
    ) -> *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
        self.it_.iter_dec();
        return & mut ( * ( self as *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__  )  )  ;
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
        self.it_.iter_add_assign(-n);
        return & mut ( * ( self as *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__  )  )  ;
    }
    pub unsafe fn move_from(
        _a0: *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_.clone(),
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__,
    ) -> *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__ {
        self.it_ = (*_a0).it_.clone();
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return & mut ( * ( self as *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__const_int_ptr__  )  )  ;
    }
}
pub unsafe fn operator_eq_4(
    x : *const random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____,
    y : *const random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____,
) -> bool {
    return (*x).it_ == (*y).it_;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { pub unsafe fn new_1 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *mut i32 { return & mut ( * self . it_  )  ;
 } pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { self . it_  .iter_dec()  ;
 return & mut ( * ( self as *mut random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  .clone() , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ , ) -> *mut random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { self . it_   = ( * _a0 ) . it_  .clone()  ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  )  )  ;
 } }
impl std::cmp::PartialEq for random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_4(self as *const random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____, other as *const random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____) }}}
impl std::cmp::Eq for random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ {}
pub unsafe fn operator_ne_5(
    x : *const random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____,
    y : *const random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____,
) -> bool {
    return (*x).it_ != (*y).it_;
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ { pub unsafe fn new_1 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const i32 { return & ( * self . it_  )  ;
 } pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ { self . it_  .iter_dec()  ;
 return & mut ( * ( self as *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  .clone() , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ , ) -> *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____ { self . it_   = ( * _a0 ) . it_  .clone()  ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut random_access_iterator_std__Deque_iterator_int__const_int_ref__min_pointer_const_int__std_integral_constant_unsigned_long__0____  )  )  ;
 } }
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
pub unsafe fn operator_eq_6(
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
pub unsafe fn operator_add_7(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_8(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_7(_i, n)
    });
}
pub unsafe fn iter_swap_9(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_10(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_11(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_12(
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
impl UnsafeCppAdvance for adl_Iterator {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_14();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_16();
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_14))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_15)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_14))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_15))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_16)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_13)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_14)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_15)) = 0_usize;
    }
}
pub static mut outstanding_bytes_13: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_14: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_15: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_16: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_17(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_18(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_19(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_18(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut i32 {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const i32,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_20(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as i64 as isize);
}
pub unsafe fn operator_lt_21(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_22(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_23(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_22(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_int__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn to_bool ( &self , ) -> bool { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : isize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : isize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::Ord for  min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { if operator_lt_21(self.clone(), other.clone()) { std::cmp::Ordering::Less } else if operator_lt_21(other.clone(), self.clone()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal } }}}
impl std::cmp::PartialOrd for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_22(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_8(); } unsafe fn cpp_dec(&mut self) { self.operator_dec_10(); } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn make_24(mut size: i32, mut start: Option<i32>) -> Vec<i32> {
    let mut start: i32 = start.unwrap_or_else(|| unsafe { 0 });
    let b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if ((start) > (0)) {
        init = ((((start) + (1)) / (b)) + (((((start) + (1)) % (b)) != (0)) as i32));
        init *= b;
        init.prefix_dec();
    }
    let mut c: Vec<i32> = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        c.push(i);
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (start)) {
        c.remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn make_25(mut size: i32, mut start: Option<i32>) -> Vec<i32> {
    let mut start: i32 = start.unwrap_or_else(|| unsafe { 0 });
    let b: i32 = (((4096_usize).wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
    let mut init: i32 = 0;
    if ((start) > (0)) {
        init = ((((start) + (1)) / (b)) + (((((start) + (1)) % (b)) != (0)) as i32));
        init *= b;
        init.prefix_dec();
    }
    let mut c: Vec<i32> = unimplemented!();
    let mut i: i32 = 0;
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        c.push(i);
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (start)) {
        c.remove(0);
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn testN_26(mut start: i32, mut N: i32) {
    let mut c1: Vec<i32> = (unsafe { make_24(N, Some(start)) });
    let mut c2: Vec<i32> = (unsafe { make_24(N, None) });
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ =
                unimplemented!();
            let mut _y: random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ =
                random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__::new_1({
                    unimplemented!()
                });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ =
                unimplemented!();
            let mut _y: random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__ =
                random_access_iterator_std__Deque_iterator_int__int_ref__int_ptr__::new_1({
                    unimplemented!()
                });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn testN_27(mut start: i32, mut N: i32) {
    let mut c1: Vec<i32> = (unsafe { make_25(N, Some(start)) });
    let mut c2: Vec<i32> = (unsafe { make_25(N, None) });
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        (unsafe {
            let mut _x : random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  = unimplemented!()  ;
            let mut _y : random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  = random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(
        (unsafe {
            let mut _x : random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  = unimplemented!()  ;
            let mut _y : random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____  = random_access_iterator_std__Deque_iterator_int__int_ref__min_pointer_int__std_integral_constant_unsigned_long__0____ :: new_1 ( { unimplemented!()  } , )  ;
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_26(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [i32; 10] = [0, 1, 2, 3, 1023, 1024, 1025, 2047, 2048, 2049];
        let N: i32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut j: i32 = 0;
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: i32 = rng[(i) as usize];
                    let _N: i32 = rng[(j) as usize];
                    testN_27(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_13);
    std::cell::LazyCell::force(&*&raw const alloc_count_14);
    std::cell::LazyCell::force(&*&raw const dealloc_count_15);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_16);
}
