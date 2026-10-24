extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_double_ended_contiguous_container_asan_correct_0(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_1(_a0: *const ()) -> () {
    return true;
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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_2(
    x : *const random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__,
    y : *const random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__ {
    pub unsafe fn new_1(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*unimplemented!());
    }    pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__{
        unimplemented!();
        return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__  )  )  ;
    }
    pub unsafe fn move_from(
        _a0 : *mut random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }    pub unsafe fn move_assign ( &mut self , _a0 : *mut random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__ , ) -> *mut random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__{
        unimplemented!();
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__  )  )  ;
    }
}
impl std::cmp::PartialEq
    for random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(self as *const random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__, other as *const random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__)
        }
    }
}
impl std::cmp::Eq
    for random_access_iterator_std___deque_iterator_int__int_ptr__int_ref__int_ptrptr__long__1024__
{
}pub unsafe fn operator_sub_3 ( mut x : random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ , mut n : () , ) -> random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__{
    (unsafe {
        random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ :: operator_sub_assign ( &mut x  , n , )
    });
    return random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ :: move_from ( { & mut x } , )   ;
}
pub unsafe fn operator_sub_4(
    mut x : random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__,
    mut y : random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_5(
    x : *const random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__,
    y : *const random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_6(
    x : *const random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__,
    y : *const random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__,
) -> () {
    return !(unimplemented!());
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ { pub unsafe fn new_1 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const () { return & ( * unimplemented!()  )   ;
 } pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ { unimplemented!()  ;
 return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ { unimplemented!()  ;
 return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } }
impl std::cmp::PartialEq for random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_5(self as *const random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__, other as *const random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__) }}}
impl std::cmp::Eq for random_access_iterator_std___deque_iterator_int__const_int_ptr__const_int_ref__const_int_ptrconst_ptr__long__1024__ {}
pub unsafe fn operator_eq_7(
    x : *const random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
    y : *const random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { pub unsafe fn new_1 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *mut () { return & mut ( * unimplemented!()  )   ;
 } pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { unimplemented!()  ;
 return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ , ) -> *mut random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { unimplemented!()  ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__  )  )  ;
 } }
impl std::cmp::PartialEq for random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_7(self as *const random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__, other as *const random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__) }}}
impl std::cmp::Eq for random_access_iterator_std___deque_iterator_int__min_pointer_int__std_integral_constant_unsigned_long__0____int_ref__min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ {}pub unsafe fn operator_sub_8 ( mut x : random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ , mut n : () , ) -> random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__{
    (unsafe {
        random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ :: operator_sub_assign ( &mut x  , n , )
    });
    return random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ :: move_from ( { & mut x } , )   ;
}
pub unsafe fn operator_sub_9(
    mut x : random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
    mut y : random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_10(
    x : *const random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
    y : *const random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_11(
    x : *const random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
    y : *const random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__,
) -> () {
    return !(unimplemented!());
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { pub unsafe fn new_1 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const () { return & ( * unimplemented!()  )   ;
 } pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { unimplemented!()  ;
 return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { unimplemented!()  ;
 return & mut ( * ( self as *mut random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } }
impl std::cmp::PartialEq for random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_10(self as *const random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__, other as *const random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__) }}}
impl std::cmp::Eq for random_access_iterator_std___deque_iterator_int__min_pointer_const_int__std_integral_constant_unsigned_long__0____const_int_ref__min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__1024__ {}
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
pub unsafe fn operator_eq_12(
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
            operator_eq_12(
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
pub unsafe fn operator_add_13(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_14(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_13(_i, n)
    });
}
pub unsafe fn iter_swap_15(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_16(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn operator_eq_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_18(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_18(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const adl_Iterator, other as *const adl_Iterator) }
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
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_20))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_21))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_22)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_19)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_20)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_21)) = 0_();
    }
}
pub static mut outstanding_bytes_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_23(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_24(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_25(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_24(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
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
        mut n: (),
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
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
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_24(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_26(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_27(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_28(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_dec_11(
        &mut self,
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_29(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as () as ());
}
pub unsafe fn operator_eq_30(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_31(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_30(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_int__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return self . ptr_   ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_30(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_32(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as () as ());
}
pub unsafe fn operator_eq_33(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_34(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *const min_pointer_int__std_integral_constant_unsigned_long__0__ { return & ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_9 ( &mut self , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_11 ( &mut self , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_33(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_const_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_const_int__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn make_35(mut size: (), mut start: Option<()>) -> () {
    let mut start: () = start.unwrap_or_else(|| unsafe { 0_() });
    let b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if ((start) > (0_())) {
        init = ((((start) + (1_())) / (b)) + ((((start) + (1_())) % (b)) != (0_())));
        init *= b;
        init.prefix_dec();
    }
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (start)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn make_36(mut size: (), mut start: Option<()>) -> () {
    let mut start: () = start.unwrap_or_else(|| unsafe { 0_() });
    let b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if ((start) > (0_())) {
        init = ((((start) + (1_())) / (b)) + ((((start) + (1_())) % (b)) != (0_())));
        init *= b;
        init.prefix_dec();
    }
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (start)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn testN_37(mut start: (), mut N: ()) {
    let mut c1: () = (unsafe { make_35(N, Some(start)) });
    let mut c2: () = (unsafe { make_35(N, None) });
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testN_38(mut start: (), mut N: ()) {
    let mut c1: () = (unsafe { make_36(N, Some(start)) });
    let mut c2: () = (unsafe { make_36(N, None) });
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            let mut j: () = 0_();
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_37(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            let mut j: () = 0_();
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_38(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_19);
    std::cell::LazyCell::force(&*&raw const alloc_count_20);
    std::cell::LazyCell::force(&*&raw const dealloc_count_21);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_22);
}
