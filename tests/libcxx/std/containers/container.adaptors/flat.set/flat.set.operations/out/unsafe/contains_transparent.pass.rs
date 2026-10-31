extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct three_way_random_access_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_const_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut three_way_random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut three_way_random_access_iterator_const_int_ptr_,
    ) -> *mut three_way_random_access_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut three_way_random_access_iterator_const_int_ptr_));
    }
}pub unsafe fn operator_add_1 ( mut x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , mut n : isize , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    (unsafe {
        three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: operator_add_assign ( &mut x  , n , )
    });
    return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { & mut x } , )   ;
}pub unsafe fn operator_add_2 ( mut n : isize , mut x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    (unsafe {
        three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: operator_add_assign ( &mut x  , n , )
    });
    return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { & mut x } , )   ;
}pub unsafe fn operator_sub_3 ( mut x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , mut n : isize , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    (unsafe {
        three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: operator_sub_assign ( &mut x  , n , )
    });
    return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { & mut x } , )   ;
}
pub unsafe fn operator_sub_4(
    mut x : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    mut y : three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<Vec<libc::c_char>>()) as i64
        as isize);
}
pub unsafe fn operator_eq_5(
    x : *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    y : *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_lt_6(
    x : *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    y : *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
) -> bool {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_
{
    it_: *mut Vec<libc::c_char>,
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { pub unsafe fn new_1 ( mut it : *mut Vec<libc::c_char > , ) -> Self { let mut this = Self { it_ : it , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *mut Vec<libc::c_char >  { return & mut ( * self . it_   )  ;
 } pub unsafe fn operator_inc_7 ( &mut self , ) -> *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_  .prefix_inc() ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_  .prefix_dec() ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_  = ( self . it_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : isize , ) -> *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_  = ( self . it_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_   = ( * _a0 ) . it_   ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } }
impl std::cmp::Ord for  three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { if operator_lt_6(self as *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_, other as *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_) { std::cmp::Ordering::Less } else if operator_lt_6(other as *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_, self as *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal } }}}
impl std::cmp::PartialOrd for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_5(self as *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_, other as *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_) }}}
impl std::cmp::Eq for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ {}
impl UnsafeCppAdvance for three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { unsafe fn cpp_inc(&mut self) { self.operator_inc_7(); } unsafe fn cpp_dec(&mut self) { self.operator_dec_8(); } }
impl three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { pub unsafe fn new_1 ( u : *const three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> Self { let mut this = Self { it_ : ( ( * u ) . it_  ) .cast_const() , tracker_ : ( * u ) . tracker_  , } ;
 this } }
impl three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { pub unsafe fn new_2 ( u : *mut three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> Self { let mut this = Self { it_ : ( ( * u ) . it_  ) .cast_const() , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * u ) . tracker_  } , )  , } ;
 ( * u ) . it_   = std::ptr::null_mut()  ;
 this } }pub unsafe fn operator_add_7 ( mut x : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , mut n : isize , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_{
    (unsafe {
        three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: operator_add_assign ( &mut x  , n , )
    });
    return three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: move_from ( { & mut x } , )   ;
}
pub unsafe fn operator_sub_8(
    mut x : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    mut y : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<Vec<libc::c_char>>()) as i64
        as isize);
}
pub unsafe fn operator_eq_9(
    x : *const three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
    y : *const three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(
    Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default,
)]
#[cpp_from(three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , new_1, ptr)]
#[cpp_from_move(three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , new_2, ptr)]
pub struct three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_
{
    it_: *const Vec<libc::c_char>,
    tracker_: support_double_move_tracker,
}
impl three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { pub unsafe fn new_3 ( mut it : *const Vec<libc::c_char > , ) -> Self { let mut this = Self { it_ : it , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const Vec<libc::c_char >  { return & ( * self . it_   )  ;
 } pub unsafe fn operator_inc_7 ( &mut self , ) -> *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_  .prefix_inc() ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } pub unsafe fn operator_dec_8 ( &mut self , ) -> *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_  .prefix_dec() ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_  = ( self . it_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { self . it_   = ( * _a0 ) . it_   ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  )  )  ;
 } }
impl std::cmp::PartialEq for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_9(self as *const three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_, other as *const three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_) }}}
impl std::cmp::Eq for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ {}
impl UnsafeCppAdvance for three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { unsafe fn cpp_inc(&mut self) { self.operator_inc_7(); } unsafe fn cpp_dec(&mut self) { self.operator_dec_8(); } }
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
pub unsafe fn operator_eq_10(
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
            operator_eq_10(
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
pub unsafe fn operator_add_11(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_12(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_11(_i, n)
    });
}
pub unsafe fn iter_swap_13(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_14(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_15(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_16(
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
        unsafe { operator_cmp_16(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_15(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__
{
    data_: Vec<i32>,
}
impl MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { pub unsafe fn move_from ( _a0 : *mut MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ , ) -> Self { let mut this = Self { data_ : std::mem::take(&mut ( * _a0 ) . data_  ) , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ , ) -> *mut MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__ { self . data_   = std::mem::take(&mut ( * _a0 ) . data_  )  ;
 return & mut ( * ( self as *mut MinSequenceContainer_int__three_way_random_access_iterator_int_ptr___three_way_random_access_iterator_const_int_ptr__  )  )  ;
 } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__
{
    data_: Vec<Vec<libc::c_char>>,
}
impl MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ { pub unsafe fn begin_11 ( &mut self , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: new_1 ( { self . data_  .as_mut_ptr()  } , )   ;
 } pub unsafe fn begin_12 ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: new_3 ( { self . data_  .as_ptr()  } , )   ;
 } pub unsafe fn cbegin ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ :: new_3 ( { self . data_  .as_ptr()  } , )   ;
 } pub unsafe fn end_14 ( &mut self , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return ( unsafe { let _x: three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  = ( unsafe { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: begin_11 ( self , ) } )  ; let _n: isize  = ( ( unsafe { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: size ( self , ) } )  as isize ) ; operator_add_1 ( _x , _n , ) } )   ;
 } pub unsafe fn end_15 ( &self , ) -> three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return ( unsafe { let _x: three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  = ( unsafe { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: begin_12 ( self , ) } )  ; let _n: isize  = ( ( unsafe { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: size ( self , ) } )  as isize ) ; operator_add_7 ( _x , _n , ) } )   ;
 } pub unsafe fn size ( &self , ) -> u32 { return ( self . data_  .len()  as u32 )  ;
 } pub unsafe fn clear ( &mut self , ) { self . data_  .clear()  ;
 } pub unsafe fn erase_25 ( &mut self , mut first : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , mut last : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return ( unsafe { let _it: UnsafeRandomAccessIterator<Vec<libc::c_char > >  = unimplemented!()  ; MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: from_vector_iterator ( self , _it , ) } )   ;
 } unsafe fn to_vector_iterator ( &self , mut cit : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , ) -> UnsafeRandomAccessIterator<Vec<libc::c_char > > { return self . data_  .begin()  .iter_add(( unsafe { let _x: three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  = cit ; operator_sub_8 ( _x , ( unsafe { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: cbegin ( self , ) } )  , ) } )  )   ;
 } unsafe fn from_vector_iterator ( &mut self , mut it : UnsafeRandomAccessIterator<Vec<libc::c_char > > , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return ( unsafe { let _n: isize  = it .iter_distance(&self . data_  .begin()  .into()) as i64  ; let _x: three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_  = ( unsafe { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: begin_11 ( self , ) } )  ; operator_add_2 ( _n , _x , ) } )   ;
 } pub unsafe fn move_from ( _a0 : *mut MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ , ) -> Self { let mut this = Self { data_ : std::mem::take(&mut ( * _a0 ) . data_  ) , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ , ) -> *mut MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ { self . data_   = std::mem::take(&mut ( * _a0 ) . data_  )  ;
 return & mut ( * ( self as *mut MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__  )  )  ;
 } pub unsafe fn insert_20_const_class_std_basic_string_char__ptr ( &mut self , mut p : three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ , mut first : *const Vec<libc::c_char > , mut last : *const Vec<libc::c_char > , ) -> three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr_ { return ( unsafe { let _it: UnsafeRandomAccessIterator<Vec<libc::c_char > >  = {let __off = ( unsafe { MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: to_vector_iterator ( self , p , ) } )  .iter_distance(&UnsafeRandomAccessIterator::Vec(self . data_  .as_mut_ptr())) as usize;
    let mut __tmp = Vec::new();
    let mut __it = first ;
    while __it != last  {
        __tmp.push(<Vec<libc::c_char >  as CppFromUnsafe<Vec<libc::c_char > >>::cpp_from(__it.iter_deref()));
        __it.iter_inc();
    }
    self . data_  .splice(__off..__off, __tmp);
    UnsafeRandomAccessIterator::Vec(self . data_  .as_mut_ptr().add(__off))}  ; MinSequenceContainer_std_basic_string_char__std_char_traits_char___std_allocator_char____three_way_random_access_iterator_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr___three_way_random_access_iterator_const_std_basic_string_char__std_char_traits_char___std_allocator_char___ptr__ :: from_vector_iterator ( self , _it , ) } )   ;
 } }
const _: () = assert!(!(false), "!HasAllocatorType<MinSequenceContainer<int>>");
const _: () = assert!(!(false), "!HasEmplace<MinSequenceContainer<int>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Transparent_int__false_ {
    pub t: i32,
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ {
    pub t: Vec<libc::c_char>,
}
impl Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ {
    pub unsafe fn move_from(
        _a0 : *mut Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
    ) -> Self {
        let mut this = Self {
            t: std::mem::take(&mut (*_a0).t),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0 : *mut Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
    ) -> *mut Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_
    {
        self.t = std::mem::take(&mut (*_a0).t);
        return & mut ( * ( self as *mut Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_  )  )  ;
    }
}
impl Default
    for Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_
{
    fn default() -> Self {
        unsafe {
            Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_ {
                t: vec![libc::c_char::default()],
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(bool, new_1, mut_ptr)]
pub struct TransparentComparator {
    pub transparent_used: *mut bool,
}
impl TransparentComparator {
    pub unsafe fn new_1(used: *mut bool) -> Self {
        let mut this = Self {
            transparent_used: (used),
        };
        this
    }
    pub unsafe fn operator_call_3_class_std_basic_string_char__0(
        &self,
        t: *const Vec<libc::c_char>,
        transparent : *const Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
    ) -> bool {
        if !((self.transparent_used).is_null()) {
            (*self.transparent_used) = true;
        }
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    pub unsafe fn operator_call_3_int_0(
        &self,
        t: *const i32,
        transparent: *const Transparent_int__false_,
    ) -> bool {
        if !((self.transparent_used).is_null()) {
            (*self.transparent_used) = true;
        }
        return ((*t) < ((*transparent).t));
    }
    pub unsafe fn operator_call_4_class_std_basic_string_char__0(
        &self,
        transparent : *const Transparent_std_basic_string_char__std_char_traits_char___std_allocator_char____false_,
        t: *const Vec<libc::c_char>,
    ) -> bool {
        if !((self.transparent_used).is_null()) {
            (*self.transparent_used) = true;
        }
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    pub unsafe fn operator_call_4_int_0(
        &self,
        transparent: *const Transparent_int__false_,
        t: *const i32,
    ) -> bool {
        if !((self.transparent_used).is_null()) {
            (*self.transparent_used) = true;
        }
        return (((*transparent).t) < (*t));
    }
    pub unsafe fn operator_call_5_class_std_basic_string_char_(
        &self,
        t1: *const Vec<libc::c_char>,
        t2: *const Vec<libc::c_char>,
    ) -> bool {
        return unimplemented!() == std::cmp::Ordering::Less;
    }
    pub unsafe fn operator_call_5_int(&self, t1: *const i32, t2: *const i32) -> bool {
        return ((*t1) < (*t2));
    }
}
impl Default for TransparentComparator {
    fn default() -> Self {
        unsafe {
            TransparentComparator {
                transparent_used: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonTransparentComparator {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoDefaultCtr {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Moveable {
    int_: i32,
    double_: f64,
}
impl Moveable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            int_: 0,
            double_: 0_f64,
        };
        this
    }
    pub unsafe fn new_2(mut i: i32, mut d: f64) -> Self {
        let mut this = Self {
            int_: i,
            double_: d,
        };
        this
    }
    pub unsafe fn move_from(x: *mut Moveable) -> Self {
        let mut this = Self {
            int_: (*x).int_,
            double_: (*x).double_,
        };
        (*x).int_ = -1_i32;
        (*x).double_ = (-1_i32 as f64);
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut Moveable) -> *mut Moveable {
        self.int_ = (*x).int_;
        (*x).int_ = -1_i32;
        self.double_ = (*x).double_;
        (*x).double_ = (-1_i32 as f64);
        return &mut (*(self as *mut Moveable));
    }
    pub unsafe fn operator_eq(&self, x: *const Moveable) -> bool {
        return ((self.int_) == ((*x).int_)) && ((self.double_) == ((*x).double_));
    }
    pub unsafe fn operator_lt(&self, x: *const Moveable) -> bool {
        return ((self.int_) < ((*x).int_))
            || (((self.int_) == ((*x).int_)) && ((self.double_) < ((*x).double_)));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.int_;
    }
    pub unsafe fn moved(&self) -> bool {
        return ((self.int_) == (-1_i32));
    }
}
impl std::cmp::Ord for Moveable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Moveable::operator_lt(self, other as *const Moveable) {
                std::cmp::Ordering::Less
            } else if Moveable::operator_lt(other, self as *const Moveable) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Moveable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Moveable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Moveable::operator_eq(self, other as *const Moveable) }
    }
}
impl std::cmp::Eq for Moveable {}
impl Default for Moveable {
    fn default() -> Self {
        unsafe { Moveable::new_1() }
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
pub static mut destructed_value_17: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_18: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_19(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_20(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_19(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_17));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_17));
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
            operator_eq_19(
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
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_22))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_23)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_22))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_23))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_24)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_21)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_22)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_23)) = 0_usize;
    }
}
pub static mut outstanding_bytes_21: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_22: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_23: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_24: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_25(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<Vec<libc::c_char>>())
        as i64 as isize);
}
pub unsafe fn operator_lt_26(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_27(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_28(
    mut x : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_27(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut Vec<libc::c_char>,
}
impl min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut Vec<libc::c_char > , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn to_bool ( &self , ) -> bool { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut Vec<libc::c_char > { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : isize , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : isize , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::Ord for  min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { if operator_lt_26(self.clone(), other.clone()) { std::cmp::Ordering::Less } else if operator_lt_26(other.clone(), self.clone()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal } }}}
impl std::cmp::PartialOrd for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_27(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_8(); } unsafe fn cpp_dec(&mut self) { self.operator_dec_10(); } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_29(
    mut x : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<Vec<libc::c_char>>())
        as i64 as isize);
}
pub unsafe fn operator_eq_30(
    mut x : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ , new_1, value)]
pub struct min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__
{
    ptr_: *const Vec<libc::c_char>,
}
impl min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const Vec<libc::c_char > { return & ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_9 ( &mut self , ) -> *mut min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_11 ( &mut self , ) -> *mut min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__  )  )  ;
 } }
impl std::cmp::PartialEq for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_30(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_const_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_9(); } unsafe fn cpp_dec(&mut self) { self.operator_dec_11(); } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char___ {}
impl min_allocator_std_basic_string_char__std_char_traits_char___std_allocator_char___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__{
        return min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_std_basic_string_char__std_char_traits_char___std_allocator_char____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
const _: () = assert!(true, "CanContains<TransparentSet>");
const _: () = assert!(true, "CanContains<const TransparentSet>");
const _: () = assert!(!(false), "!CanContains<NonTransparentSet>");
const _: () = assert!(!(false), "!CanContains<const NonTransparentSet>");
pub unsafe fn test_one_31() {
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_one_32() {
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_one_33() {
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_one_34() {
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (true as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
    {
        let mut m: () = unimplemented!();
        assert!(((unimplemented!() as i32) == (false as i32)));
        assert!(((unimplemented!() as i32) == (false as i32)));
    }
}
pub unsafe fn test_35() -> bool {
    (unsafe { test_one_31() });
    {
        (unsafe { test_one_32() });
    }
    (unsafe { test_one_33() });
    (unsafe { test_one_34() });
    {
        let mut transparent_used: bool = false;
        let mut c: TransparentComparator = TransparentComparator::new_1({ &mut transparent_used });
        let mut m: () = unimplemented!();
        assert!(!(transparent_used));
        let mut b: bool = unimplemented!();
        assert!((b as bool));
        assert!(transparent_used);
    }
    {
        let mut m: () = unimplemented!();
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
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
    (unsafe { test_35() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_17);
    std::cell::LazyCell::force(&*&raw const moved_value_18);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_21);
    std::cell::LazyCell::force(&*&raw const alloc_count_22);
    std::cell::LazyCell::force(&*&raw const dealloc_count_23);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_24);
}
