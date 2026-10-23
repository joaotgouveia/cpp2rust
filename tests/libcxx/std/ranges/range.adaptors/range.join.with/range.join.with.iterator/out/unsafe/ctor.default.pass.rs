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
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct forward_iterator_std___wrap_iter_const_char_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_const_char_ptr__ {
    pub unsafe fn new_1(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
}
pub unsafe fn operator_eq_0(
    x: *const forward_iterator_std___wrap_iter_int_ptr__,
    y: *const forward_iterator_std___wrap_iter_int_ptr__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_std___wrap_iter_int_ptr__,
    y: *const forward_iterator_std___wrap_iter_int_ptr__,
) -> () {
    return !(unimplemented!());
}
pub unsafe fn base_2(i: *const forward_iterator_std___wrap_iter_int_ptr__) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct forward_iterator_std___wrap_iter_int_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_int_ptr__ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: unimplemented!(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_std___wrap_iter_int_ptr__) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_std___wrap_iter_int_ptr__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const forward_iterator_std___wrap_iter_int_ptr__,
                other as *const forward_iterator_std___wrap_iter_int_ptr__,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_std___wrap_iter_int_ptr__ {}
impl Default for forward_iterator_std___wrap_iter_int_ptr__ {
    fn default() -> Self {
        unsafe { forward_iterator_std___wrap_iter_int_ptr__::new_1() }
    }
}
pub unsafe fn operator_eq_3(
    x : *const forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
    y : *const forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_4(
    x : *const forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
    y : *const forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
) -> () {
    return !(unimplemented!());
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { pub unsafe fn new_1 ( ) -> Self { let mut this = Self { it_ : unimplemented!() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn new_2 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> () { return & mut ( * self . it_  )  ;
 } pub unsafe fn operator_inc_6 ( &mut self , ) -> *mut forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { unimplemented!()  ;
 return & mut ( * ( self as *mut forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } }
impl std::cmp::PartialEq for forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_3(self as *const forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__, other as *const forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__) }}}
impl std::cmp::Eq for forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ {}
impl Default for forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { fn default() -> Self { unsafe { forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ :: new_1 ( )  } } }
impl forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { pub unsafe fn new_1 ( other : *mut forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ , ) -> Self { let mut this = Self { it_ : std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr_ :: new_1 ( { & ( * other ) . it_  } , )  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * other ) . tracker_  } , )  , } ;
 unimplemented!()  ;
 this } }
pub unsafe fn operator_eq_5(
    x : *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
    y : *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_6(
    x : *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
    y : *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__,
) -> () {
    return !(unimplemented!());
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { pub unsafe fn new_2 ( ) -> Self { let mut this = Self { it_ : unimplemented!() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn new_3 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> () { return & ( * self . it_  )  ;
 } pub unsafe fn operator_inc_6 ( &mut self , ) -> *mut forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { unimplemented!()  ;
 return & mut ( * ( self as *mut forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } }
impl std::cmp::PartialEq for forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_5(self as *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__, other as *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__) }}}
impl std::cmp::Eq for forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ {}
impl Default for forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { fn default() -> Self { unsafe { forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ :: new_2 ( )  } } }
impl forward_iterator_std___wrap_iter_const_int_ptr__ {
    pub unsafe fn new_1(other: *mut forward_iterator_std___wrap_iter_int_ptr__) -> Self {
        let mut this = Self {
            it_: std___wrap_iter_const_int_ptr_::new_1({ &(*other).it_ }),
            tracker_: support_double_move_tracker::move_from({ &mut (*other).tracker_ }),
        };
        unimplemented!();
        this
    }
}
pub unsafe fn operator_eq_7(
    x: *const forward_iterator_std___wrap_iter_const_int_ptr__,
    y: *const forward_iterator_std___wrap_iter_const_int_ptr__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_8(
    x: *const forward_iterator_std___wrap_iter_const_int_ptr__,
    y: *const forward_iterator_std___wrap_iter_const_int_ptr__,
) -> () {
    return !(unimplemented!());
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct forward_iterator_std___wrap_iter_const_int_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_const_int_ptr__ {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {
            it_: unimplemented!(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_3(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &(*self.it_);
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_std___wrap_iter_const_int_ptr__) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_std___wrap_iter_const_int_ptr__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const forward_iterator_std___wrap_iter_const_int_ptr__,
                other as *const forward_iterator_std___wrap_iter_const_int_ptr__,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_std___wrap_iter_const_int_ptr__ {}
impl Default for forward_iterator_std___wrap_iter_const_int_ptr__ {
    fn default() -> Self {
        unsafe { forward_iterator_std___wrap_iter_const_int_ptr__::new_2() }
    }
}
pub unsafe fn operator_eq_9(
    x : *const forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__,
    y : *const forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__,
) -> () {
    return unimplemented!();
}
pub unsafe fn base_10(
    i : *const forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__,
) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ { pub unsafe fn new_1 ( ) -> Self { let mut this = Self { it_ : unimplemented!() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn new_2 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> () { return & mut ( * self . it_  )  ;
 } pub unsafe fn operator_inc_6 ( &mut self , ) -> *mut forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ { unimplemented!()  ;
 return & mut ( * ( self as *mut forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__  )  )  ;
 } pub unsafe fn move_from ( _a0 : *mut forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } }
impl std::cmp::PartialEq for forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_9(self as *const forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__, other as *const forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__) }}}
impl std::cmp::Eq for forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ {}
impl Default for forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ { fn default() -> Self { unsafe { forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ :: new_1 ( )  } } }
pub unsafe fn operator_eq_11(
    x: *const forward_iterator_std___wrap_iter_char_ptr__,
    y: *const forward_iterator_std___wrap_iter_char_ptr__,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_ne_12(
    x: *const forward_iterator_std___wrap_iter_char_ptr__,
    y: *const forward_iterator_std___wrap_iter_char_ptr__,
) -> () {
    return !(unimplemented!());
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct forward_iterator_std___wrap_iter_char_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_char_ptr__ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: unimplemented!(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
        return &mut (*self.it_);
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_std___wrap_iter_char_ptr__) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
}
impl std::cmp::PartialEq for forward_iterator_std___wrap_iter_char_ptr__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_11(
                self as *const forward_iterator_std___wrap_iter_char_ptr__,
                other as *const forward_iterator_std___wrap_iter_char_ptr__,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_std___wrap_iter_char_ptr__ {}
impl Default for forward_iterator_std___wrap_iter_char_ptr__ {
    fn default() -> Self {
        unsafe { forward_iterator_std___wrap_iter_char_ptr__::new_1() }
    }
}
pub unsafe fn base_13(
    i : *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__,
) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ { pub unsafe fn new_1 ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } }
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
pub unsafe fn base_14(
    i : *const cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__,
) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ { pub unsafe fn new ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn move_from ( _a0 : *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ , ) -> *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ { unimplemented!()  ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__  )  )  ;
 } pub unsafe fn operator_deref ( &self , ) -> std_basic_string_char__std_char_traits_char___std_allocator_char__ { return ( * self . it_  )  ;
 } }
pub unsafe fn base_15(
    i : *const cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__,
) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__
{
    it_: (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ { pub unsafe fn new ( mut it : () , ) -> Self { let mut this = Self { it_ : it .clone() , tracker_ : support_double_move_tracker :: new ( )  , } ;
 this } pub unsafe fn move_from ( _a0 : *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ , ) -> Self { let mut this = Self { it_ : ( * _a0 ) . it_  , tracker_ : support_double_move_tracker :: move_from ( { & mut ( * _a0 ) . tracker_  } , )  , } ;
 this } pub unsafe fn move_assign ( &mut self , _a0 : *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ , ) -> *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ { unimplemented!()  ;
 ( unsafe { let _other: *mut support_double_move_tracker  = & mut ( * _a0 ) . tracker_  ; support_double_move_tracker :: move_assign ( &mut self . tracker_   , _other , ) } )  ;
 return & mut ( * ( self as *mut cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__  )  )  ;
 } pub unsafe fn operator_deref ( &self , ) -> std_basic_string_char__std_char_traits_char___std_allocator_char__ { return ( * self . it_  )  ;
 } }
pub unsafe fn base_16(i: *const cpp20_input_iterator_std___wrap_iter_int_ptr__) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_std___wrap_iter_int_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_std___wrap_iter_int_ptr__ {
    pub unsafe fn new(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_std___wrap_iter_int_ptr__) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_std___wrap_iter_int_ptr__,
    ) -> *mut cpp20_input_iterator_std___wrap_iter_int_ptr__ {
        unimplemented!();
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_std___wrap_iter_int_ptr__));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
pub unsafe fn base_17(i: *const cpp20_input_iterator_std___wrap_iter_const_int_ptr__) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_std___wrap_iter_const_int_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_std___wrap_iter_const_int_ptr__ {
    pub unsafe fn new(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(
        _a0: *mut cpp20_input_iterator_std___wrap_iter_const_int_ptr__,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_std___wrap_iter_const_int_ptr__,
    ) -> *mut cpp20_input_iterator_std___wrap_iter_const_int_ptr__ {
        unimplemented!();
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_std___wrap_iter_const_int_ptr__));
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
pub unsafe fn base_18(i: *const cpp20_output_iterator_std___wrap_iter_int_ptr__) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_std___wrap_iter_int_ptr__ {
    it_: (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_std___wrap_iter_int_ptr__ {
    pub unsafe fn new(mut it: ()) -> Self {
        let mut this = Self {
            it_: it.clone(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_std___wrap_iter_int_ptr__) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_std___wrap_iter_int_ptr__,
    ) -> *mut cpp20_output_iterator_std___wrap_iter_int_ptr__ {
        unimplemented!();
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_std___wrap_iter_int_ptr__));
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
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false___
{
    base_: (),
}
impl sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false___ { pub unsafe fn new_1 ( it : *const cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ , ) -> Self { let mut this = Self { base_ : ( unsafe { base_14 ( it , ) } )  , } ;
 this } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true___
{
    base_: (),
}
impl sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true___ { pub unsafe fn new_1 ( it : *const cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ , ) -> Self { let mut this = Self { base_ : ( unsafe { base_15 ( it , ) } )  , } ;
 this } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_int_ptr___ {
    base_: (),
}
impl sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_int_ptr___ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_std___wrap_iter_int_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_16(it) }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_output_iterator_std___wrap_iter_int_ptr___ {
    base_: (),
}
impl sentinel_wrapper_cpp20_output_iterator_std___wrap_iter_int_ptr___ {
    pub unsafe fn new_1(it: *const cpp20_output_iterator_std___wrap_iter_int_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_18(it) }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_std___wrap_iter_int_ptr___ {
    base_: (),
}
impl sentinel_wrapper_forward_iterator_std___wrap_iter_int_ptr___ {
    pub unsafe fn new_1(it: *const forward_iterator_std___wrap_iter_int_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_2(it) }),
        };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_const_int_ptr___ {
    base_: (),
}
impl sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_const_int_ptr___ {
    pub unsafe fn new_1(it: *const cpp20_input_iterator_std___wrap_iter_const_int_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_17(it) }),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_const_int_ptr_ {
    base_: *const (),
}
impl sentinel_wrapper_const_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_int_ptr_ {
    base_: *mut (),
}
impl sentinel_wrapper_int_ptr_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___
{
    base_: (),
}
impl sentinel_wrapper_forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___ { pub unsafe fn new_1 ( it : *const forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ , ) -> Self { let mut this = Self { base_ : ( unsafe { base_10 ( it , ) } )  , } ;
 this } pub unsafe fn operator_eq ( &self , other : *const forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ , ) -> () { { } return unimplemented!()   ;
 } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_EqComparableInputIter_std___wrap_iter_char_ptr___ {
    base_: (),
}
impl sentinel_wrapper_EqComparableInputIter_std___wrap_iter_char_ptr___ {
    pub unsafe fn new_1(it: *const EqComparableInputIter_std___wrap_iter_char_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_19(it) }),
        };
        this
    }
    pub unsafe fn operator_eq(
        &self,
        other: *const EqComparableInputIter_std___wrap_iter_char_ptr__,
    ) -> () {
        {}
        return unimplemented!();
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___
{
    base_: (),
}
impl sentinel_wrapper_forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___ { pub unsafe fn new_1 ( it : *const forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ , ) -> Self { let mut this = Self { base_ : ( unsafe { base_13 ( it , ) } )  , } ;
 this } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct sentinel_wrapper_EqComparableInputIter_std___wrap_iter_const_char_ptr___ {
    base_: (),
}
impl sentinel_wrapper_EqComparableInputIter_std___wrap_iter_const_char_ptr___ {
    pub unsafe fn new_1(it: *const EqComparableInputIter_std___wrap_iter_const_char_ptr__) -> Self {
        let mut this = Self {
            base_: (unsafe { base_20(it) }),
        };
        this
    }
}
pub unsafe fn operator_eq_21(
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
            operator_eq_21(
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
pub unsafe fn operator_add_22(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_23(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_22(_i, n)
    });
}
pub unsafe fn iter_swap_24(mut a: adl_Iterator, mut b: adl_Iterator) {
    (unsafe {
        let ___x: *mut *mut () = &mut a.ptr_;
        let ___y: *mut *mut () = &mut b.ptr_;
        swap_25(___x, ___y)
    });
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_26(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_27(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_28(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
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
        unsafe { operator_cmp_28(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_27(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub static mut enable_borrowed_range_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedRange<>>");
const _: () = assert!(true, "std::ranges::view<BorrowedView>");
const _: () = assert!(true, "std::ranges::borrowed_range<BorrowedView>");
const _: () = assert!(true, "std::ranges::view<NonBorrowedView>");
const _: () = assert!(!(false), "!std::ranges::borrowed_range<NonBorrowedView>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ViewProperties {
    pub simple: (),
    pub common: (),
}
impl Default for ViewProperties {
    fn default() -> Self {
        unsafe {
            ViewProperties {
                simple: false,
                common: true,
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct BasicView_std_string__ViewProperties_true__true___forward_iterator__forward_iterator_ {
    data_: (),
}
impl BasicView_std_string__ViewProperties_true__true___forward_iterator__forward_iterator_ {
    pub unsafe fn begin_6(&self) -> forward_iterator_std___wrap_iter_const_char_ptr__ {
        return forward_iterator_std___wrap_iter_const_char_ptr__::new_1({ unimplemented!() });
    }
    pub unsafe fn end_7(&self) -> forward_iterator_std___wrap_iter_const_char_ptr__ {
        {
            return forward_iterator_std___wrap_iter_const_char_ptr__::new_1({ unimplemented!() });
        }
        panic!("ub: non-void function does not return a value")
    }
}
impl Default
    for BasicView_std_string__ViewProperties_true__true___forward_iterator__forward_iterator_
{
    fn default() -> Self {
        unsafe {
            BasicView_std_string__ViewProperties_true__true___forward_iterator__forward_iterator_ {
                data_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct BasicView_RvalueVector_std_string___ViewProperties_false__false___cpp20_input_iterator__cpp20_input_iterator_
{
    data_: RvalueVector_std_string_,
}
impl BasicView_RvalueVector_std_string___ViewProperties_false__false___cpp20_input_iterator__cpp20_input_iterator_ { pub unsafe fn begin_4 ( &mut self , ) -> cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ { return cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ :: new ( { ( unsafe { RvalueVector_std_string_ :: begin_4 ( &mut self . data_   , ) } )  } , )   ;
 } pub unsafe fn end_5 ( &mut self , ) -> sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false___ { { return { let mut __tmp_0 : cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__  = cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false__ :: new ( { ( unsafe { RvalueVector_std_string_ :: end_5 ( &mut self . data_   , ) } )  } , )  ; sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_false___ :: new_1 ( { & mut __tmp_0 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } pub unsafe fn begin_6 ( &self , ) -> cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ { return cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ :: new ( { ( unsafe { RvalueVector_std_string_ :: begin_6 ( & self . data_   , ) } )  } , )   ;
 } pub unsafe fn end_7 ( &self , ) -> sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true___ { { return { let mut __tmp_1 : cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__  = cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true__ :: new ( { ( unsafe { RvalueVector_std_string_ :: end_7 ( & self . data_   , ) } )  } , )  ; sentinel_wrapper_cpp20_input_iterator_std_ranges_transform_view_std_ranges_owning_view_std_vector_std_string__std_allocator_std_string_____AsPrvalue____iterator_true___ :: new_1 ( { & mut __tmp_1 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct BasicView_std_vector_int__std_allocator_int____ViewProperties_false__false___forward_iterator__cpp20_input_iterator_
{
    data_: (),
}
impl BasicView_std_vector_int__std_allocator_int____ViewProperties_false__false___forward_iterator__cpp20_input_iterator_ { pub unsafe fn begin_4 ( &mut self , ) -> forward_iterator_std___wrap_iter_int_ptr__ { return forward_iterator_std___wrap_iter_int_ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_5 ( &mut self , ) -> sentinel_wrapper_forward_iterator_std___wrap_iter_int_ptr___ { { return { let mut __tmp_2 : forward_iterator_std___wrap_iter_int_ptr__  = forward_iterator_std___wrap_iter_int_ptr__ :: new_2 ( { unimplemented!()  } , )  ; sentinel_wrapper_forward_iterator_std___wrap_iter_int_ptr___ :: new_1 ( { & mut __tmp_2 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } pub unsafe fn begin_6 ( &self , ) -> cpp20_input_iterator_std___wrap_iter_const_int_ptr__ { return cpp20_input_iterator_std___wrap_iter_const_int_ptr__ :: new ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_7 ( &self , ) -> sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_const_int_ptr___ { { return { let mut __tmp_3 : cpp20_input_iterator_std___wrap_iter_const_int_ptr__  = cpp20_input_iterator_std___wrap_iter_const_int_ptr__ :: new ( { unimplemented!()  } , )  ; sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_const_int_ptr___ :: new_1 ( { & mut __tmp_3 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator_
{
    data_: (),
}
impl BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator_ { pub unsafe fn new_1 ( mut il : () , ) -> Self { let mut this = Self { data_ : unimplemented!()  , } ;
 this } pub unsafe fn begin_4 ( &mut self , ) -> forward_iterator_std___wrap_iter_int_ptr__ { return forward_iterator_std___wrap_iter_int_ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_5 ( &mut self , ) -> forward_iterator_std___wrap_iter_int_ptr__ { { return forward_iterator_std___wrap_iter_int_ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } panic!("ub: non-void function does not return a value") } pub unsafe fn begin_6 ( &self , ) -> forward_iterator_std___wrap_iter_const_int_ptr__ { return forward_iterator_std___wrap_iter_const_int_ptr__ :: new_3 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_7 ( &self , ) -> forward_iterator_std___wrap_iter_const_int_ptr__ { { return forward_iterator_std___wrap_iter_const_int_ptr__ :: new_3 ( { unimplemented!()  } , )   ;
 } panic!("ub: non-void function does not return a value") } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct BasicView_std_vector_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator___std_allocator_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator_____ViewProperties_false__true___forward_iterator__forward_iterator_
{
    data_: (),
}
impl BasicView_std_vector_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator___std_allocator_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator_____ViewProperties_false__true___forward_iterator__forward_iterator_ { pub unsafe fn new_1 ( mut il : () , ) -> Self { let mut this = Self { data_ : unimplemented!()  , } ;
 this } pub unsafe fn begin_4 ( &mut self , ) -> forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { return forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_5 ( &mut self , ) -> forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { { return forward_iterator_std___wrap_iter_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } panic!("ub: non-void function does not return a value") } pub unsafe fn begin_6 ( &self , ) -> forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { return forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ :: new_3 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_7 ( &self , ) -> forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ { { return forward_iterator_std___wrap_iter_const_BasicView_std_vector_int__std_allocator_int____ViewProperties_false__true___forward_iterator__forward_iterator__ptr__ :: new_3 ( { unimplemented!()  } , )   ;
 } panic!("ub: non-void function does not return a value") } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter_
{
    data_: (),
}
impl BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter_ { pub unsafe fn new_1 ( mut il : () , ) -> Self { let mut this = Self { data_ : unimplemented!()  , } ;
 this } pub unsafe fn begin_4 ( &mut self , ) -> EqComparableInputIter_std___wrap_iter_char_ptr__ { return EqComparableInputIter_std___wrap_iter_char_ptr__ :: new ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_5 ( &mut self , ) -> sentinel_wrapper_EqComparableInputIter_std___wrap_iter_char_ptr___ { { return { let mut __tmp_4 : EqComparableInputIter_std___wrap_iter_char_ptr__  = EqComparableInputIter_std___wrap_iter_char_ptr__ :: new ( { unimplemented!()  } , )  ; sentinel_wrapper_EqComparableInputIter_std___wrap_iter_char_ptr___ :: new_1 ( { & mut __tmp_4 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } pub unsafe fn begin_6 ( &self , ) -> EqComparableInputIter_std___wrap_iter_const_char_ptr__ { return EqComparableInputIter_std___wrap_iter_const_char_ptr__ :: new ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_7 ( &self , ) -> sentinel_wrapper_EqComparableInputIter_std___wrap_iter_const_char_ptr___ { { return { let mut __tmp_5 : EqComparableInputIter_std___wrap_iter_const_char_ptr__  = EqComparableInputIter_std___wrap_iter_const_char_ptr__ :: new ( { unimplemented!()  } , )  ; sentinel_wrapper_EqComparableInputIter_std___wrap_iter_const_char_ptr___ :: new_1 ( { & mut __tmp_5 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct BasicView_std_vector_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter___std_allocator_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter_____ViewProperties_false__false___forward_iterator__forward_iterator_
{
    data_: (),
}
impl BasicView_std_vector_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter___std_allocator_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter_____ViewProperties_false__false___forward_iterator__forward_iterator_ { pub unsafe fn new_1 ( mut il : () , ) -> Self { let mut this = Self { data_ : unimplemented!()  , } ;
 this } pub unsafe fn begin_4 ( &mut self , ) -> forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ { return forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_5 ( &mut self , ) -> sentinel_wrapper_forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___ { { return { let mut __tmp_6 : forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__  = forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ :: new_2 ( { unimplemented!()  } , )  ; sentinel_wrapper_forward_iterator_std___wrap_iter_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___ :: new_1 ( { & mut __tmp_6 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } pub unsafe fn begin_6 ( &self , ) -> forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ { return forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_7 ( &self , ) -> sentinel_wrapper_forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___ { { return { let mut __tmp_7 : forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__  = forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr__ :: new_1 ( { unimplemented!()  } , )  ; sentinel_wrapper_forward_iterator_std___wrap_iter_const_BasicView_std_vector_char__std_allocator_char____ViewProperties_false__false___EqComparableInputIter__EqComparableInputIter__ptr___ :: new_1 ( { & mut __tmp_7 } , )  }  ;
 } panic!("ub: non-void function does not return a value") } }
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct BasicView_std_vector_char__std_allocator_char____ViewProperties_false__true___forward_iterator__forward_iterator_
{
    data_: (),
}
impl BasicView_std_vector_char__std_allocator_char____ViewProperties_false__true___forward_iterator__forward_iterator_ { pub unsafe fn new_1 ( mut il : () , ) -> Self { let mut this = Self { data_ : unimplemented!()  , } ;
 this } pub unsafe fn begin_4 ( &mut self , ) -> forward_iterator_std___wrap_iter_char_ptr__ { return forward_iterator_std___wrap_iter_char_ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_5 ( &mut self , ) -> forward_iterator_std___wrap_iter_char_ptr__ { { return forward_iterator_std___wrap_iter_char_ptr__ :: new_2 ( { unimplemented!()  } , )   ;
 } panic!("ub: non-void function does not return a value") } pub unsafe fn begin_6 ( &self , ) -> forward_iterator_std___wrap_iter_const_char_ptr__ { return forward_iterator_std___wrap_iter_const_char_ptr__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn end_7 ( &self , ) -> forward_iterator_std___wrap_iter_const_char_ptr__ { { return forward_iterator_std___wrap_iter_const_char_ptr__ :: new_1 ( { unimplemented!()  } , )   ;
 } panic!("ub: non-void function does not return a value") } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AsPrvalue {}
impl AsPrvalue {
    pub unsafe fn operator_call_1_std_string_ref(&self, t: *mut ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_call_1_const_std_string_ref(&self, t: *const ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_call_1_int_ref(&self, t: *mut ()) -> () {
        return (*t);
    }
    pub unsafe fn operator_call_1_const_int_ref(&self, t: *const ()) -> () {
        return (*t);
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct RvalueVector_std_string_ {
    range_: (),
}
impl RvalueVector_std_string_ {
    pub unsafe fn begin_4(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end_5(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn begin_6(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end_7(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn move_from(_a0: *mut RvalueVector_std_string_) -> Self {
        let mut this = Self {
            range_: (*_a0).range_,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut RvalueVector_std_string_,
    ) -> *mut RvalueVector_std_string_ {
        self.range_ = ((*_a0).range_).clone();
        return &mut (*(self as *mut RvalueVector_std_string_));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct RvalueVector_int_ {
    range_: (),
}
impl RvalueVector_int_ {
    pub unsafe fn begin_4(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end_5(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn begin_6(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end_7(&self) -> () {
        return unimplemented!();
    }
    pub unsafe fn move_from(_a0: *mut RvalueVector_int_) -> Self {
        let mut this = Self {
            range_: (*_a0).range_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut RvalueVector_int_) -> *mut RvalueVector_int_ {
        self.range_ = ((*_a0).range_).clone();
        return &mut (*(self as *mut RvalueVector_int_));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DefaultCtorInputIter_int_ptr_ {
    it_: *mut (),
}
impl DefaultCtorInputIter_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_eq(&self, _a0: *const DefaultCtorInputIter_int_ptr_) -> () {
        return (((*(self as *const DefaultCtorInputIter_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::PartialEq for DefaultCtorInputIter_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            DefaultCtorInputIter_int_ptr_::operator_eq(
                self,
                other as *const DefaultCtorInputIter_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for DefaultCtorInputIter_int_ptr_ {}
impl Default for DefaultCtorInputIter_int_ptr_ {
    fn default() -> Self {
        unsafe {
            DefaultCtorInputIter_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct InputRangeButOutputWhenConst_int_ {
    range_: (),
}
impl InputRangeButOutputWhenConst_int_ {
    pub unsafe fn begin_2(&mut self) -> cpp20_input_iterator_std___wrap_iter_int_ptr__ {
        return cpp20_input_iterator_std___wrap_iter_int_ptr__::new({ unimplemented!() });
    }
    pub unsafe fn end_3(
        &mut self,
    ) -> sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_int_ptr___ {
        return {
            let mut __tmp_8: cpp20_input_iterator_std___wrap_iter_int_ptr__ =
                cpp20_input_iterator_std___wrap_iter_int_ptr__::new({ unimplemented!() });
            sentinel_wrapper_cpp20_input_iterator_std___wrap_iter_int_ptr___::new_1({
                &mut __tmp_8
            })
        };
    }
    pub unsafe fn begin_4(&self) -> cpp20_output_iterator_std___wrap_iter_int_ptr__ {
        return cpp20_output_iterator_std___wrap_iter_int_ptr__::new({ unimplemented!() });
    }
    pub unsafe fn end_5(
        &self,
    ) -> sentinel_wrapper_cpp20_output_iterator_std___wrap_iter_int_ptr___ {
        return {
            let mut __tmp_9: cpp20_output_iterator_std___wrap_iter_int_ptr__ =
                cpp20_output_iterator_std___wrap_iter_int_ptr__::new({ unimplemented!() });
            sentinel_wrapper_cpp20_output_iterator_std___wrap_iter_int_ptr___::new_1({
                &mut __tmp_9
            })
        };
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ForwardIteratorWithInputCategory_long_ptr_ {
    it_: *mut (),
}
impl ForwardIteratorWithInputCategory_long_ptr_ {}
impl Default for ForwardIteratorWithInputCategory_long_ptr_ {
    fn default() -> Self {
        unsafe {
            ForwardIteratorWithInputCategory_long_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EqComparableInputIter_int_ptr_ {
    it_: *mut (),
}
impl EqComparableInputIter_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
pub unsafe fn base_19(i: *const EqComparableInputIter_std___wrap_iter_char_ptr__) -> () {
    return (*i).it_.clone();
}
pub unsafe fn operator_eq_30(
    left: *const EqComparableInputIter_std___wrap_iter_char_ptr__,
    right: *const EqComparableInputIter_std___wrap_iter_char_ptr__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct EqComparableInputIter_std___wrap_iter_char_ptr__ {
    it_: (),
}
impl EqComparableInputIter_std___wrap_iter_char_ptr__ {
    pub unsafe fn new(mut it: ()) -> Self {
        let mut this = Self { it_: it.clone() };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
impl std::cmp::PartialEq for EqComparableInputIter_std___wrap_iter_char_ptr__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_30(
                self as *const EqComparableInputIter_std___wrap_iter_char_ptr__,
                other as *const EqComparableInputIter_std___wrap_iter_char_ptr__,
            )
        }
    }
}
impl std::cmp::Eq for EqComparableInputIter_std___wrap_iter_char_ptr__ {}
pub unsafe fn base_20(i: *const EqComparableInputIter_std___wrap_iter_const_char_ptr__) -> () {
    return (*i).it_.clone();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct EqComparableInputIter_std___wrap_iter_const_char_ptr__ {
    it_: (),
}
impl EqComparableInputIter_std___wrap_iter_const_char_ptr__ {
    pub unsafe fn new(mut it: ()) -> Self {
        let mut this = Self { it_: it.clone() };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.it_);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConstOppositeView_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct lwg4074_Value {}
impl lwg4074_Value {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct lwg4074_Reference {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct lwg4074_CommonReference {}
impl lwg4074_CommonReference {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct lwg4074_Iter {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct lwg4074_PatternWithProxyConstAccess {}
const _: () = assert!(true, "std::ranges::forward_range<BV1>");
const _: () = assert!(!(false), "!std::ranges::bidirectional_range<BV1>");
const _: () = assert!(true, "std::ranges::common_range<BV1>");
const _: () = assert!(true, "simple_view<BV1>");
const _: () = assert!(true, "std::ranges::input_range<BV2>");
const _: () = assert!(!(false), "!std::ranges::forward_range<BV2>");
const _: () = assert!(!(false), "!std::ranges::common_range<BV2>");
const _: () = assert!(
    true,
    "!std::is_reference_v<std::ranges::range_reference_t<BV2>>"
);
const _: () = assert!(!(false), "!simple_view<BV2>");
const _: () = assert!(true, "std::movable<RV>");
const _: () = assert!(true, "std::ranges::random_access_range<RV>");
const _: () = assert!(true, "std::ranges::random_access_range<const RV>");
const _: () = assert!(
    true,
    "!std::is_reference_v<std::ranges::range_reference_t<RV>>"
);
const _: () = assert!(
    true,
    "!std::is_reference_v<std::ranges::range_reference_t<const RV>>"
);
const _: () = assert!(true, "std::default_initializable<DCII>");
const _: () = assert!(true, "std::sentinel_for<DCII, DCII>");
const _: () = assert!(true, "std::input_iterator<DCII>");
const _: () = assert!(!(false), "!std::forward_iterator<DCII>");
const _: () = assert!(true, "std::ranges::input_range<IRBOWC>");
const _: () = assert!(true, "std::ranges::output_range<const IRBOWC&, int>");
const _: () = assert!(true, "std::default_initializable<FVBIWC>");
const _: () = assert!(true, "std::ranges::view<FVBIWC>");
const _: () = assert!(true, "std::ranges::forward_range<FVBIWC>");
const _: () = assert!(!(false), "!std::ranges::common_range<FVBIWC>");
const _: () = assert!(true, "std::ranges::input_range<const FVBIWC&>");
const _: () = assert!(!(false), "!std::ranges::forward_range<const FVBIWC&>");
const _: () = assert!(!(false), "!std::ranges::common_range<const FVBIWC&>");
const _: () = assert!(true, "std::forward_iterator<FIWIC>");
const _: () = assert!(
    true,
    "std::same_as<FIWIC::iterator_category, std::input_iterator_tag>"
);
const _: () = assert!(
    true,
    "std::same_as<FIWIC::iterator_category, std::iterator_traits<FIWIC>::iterator_category>"
);
const _: () = assert!(true, "std::input_iterator<ECII>");
const _: () = assert!(!(false), "!std::forward_iterator<ECII>");
const _: () = assert!(true, "std::equality_comparable<ECII>");
const _: () = assert!(true, "std::ranges::view<COV>");
const _: () = assert!(true, "std::ranges::range<const COV>");
const _: () = assert!(!(false), "!std::ranges::common_range<COV>");
const _: () = assert!(!(false), "!std::ranges::common_range<const COV>");
const _: () = assert!(
    true,
    "std::convertible_to<std::ranges::iterator_t<const COV>, std::ranges::iterator_t<COV>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::iterator_t<COV>, std::ranges::iterator_t<const COV>>"
);
const _: () = assert!(true, "std::common_with<lwg4074::Value, int>");
const _: () = assert!(true, "std::common_with<lwg4074::Value, lwg4074::Reference>");
const _: () = assert!(true, "std::common_reference_with<lwg4074::Reference, int&>");
const _: () = assert!(
    true,
    "std::common_reference_with<lwg4074::Reference, lwg4074::CommonReference>"
);
const _: () = assert!(true, "std::forward_iterator<lwg4074::Iter>");
const _: () = assert!(
    true,
    "std::ranges::forward_range<lwg4074::PatternWithProxyConstAccess>"
);
const _: () = assert!(
    true,
    "std::ranges::forward_range<const lwg4074::PatternWithProxyConstAccess>"
);
pub unsafe fn testEquality_31(t1: *const (), t2: *const (), mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testEquality_32(t1: *const (), t2: *const (), mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testEquality_33(t1: *const (), t2: *const (), mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testEquality_34(t1: *const (), t2: *const (), mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testEquality_35(t1: *const (), t2: *const (), mut isEqual: ()) -> () {
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
    }
    return true;
}
pub unsafe fn operator_lt_36(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_37(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LessAndEqComp {
    pub value: (),
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_36(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_36(other as *const LessAndEqComp, self as *const LessAndEqComp) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_37(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_38(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrongOrder {
    pub value: (),
}
impl StrongOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_38(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_38(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_39(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let mut cmp: () = unimplemented!();
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WeakOrder {
    pub value: (),
}
impl WeakOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_39(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_39(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_40(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
        return unimplemented!();
    }
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_41(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PartialOrder {
    pub value: (),
}
impl PartialOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_40(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_41(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub unsafe fn operator_eq_42(_a0: *const EqualityComparable, _a1: *const EqualityComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EqualityComparable {
    value_: (),
}
impl EqualityComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_42(
                self as *const EqualityComparable,
                other as *const EqualityComparable,
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub unsafe fn operator_eq_43(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_44(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ThreeWayComparable {
    value_: (),
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_44(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_43(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E1 {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool {
    pub val: (),
}
impl ImplicitBool {
    pub unsafe fn to__1(&self) -> () {
        return self.val;
    }
}
pub unsafe fn operator_eq_45(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
pub unsafe fn test_46() -> () {
    {
        let mut iter: () = unimplemented!();
        unimplemented!();
        let mut citer: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut jwv: () = unimplemented!();
        let mut jwv_iter: () = unimplemented!();
        let mut jwv_citer: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut iter: () = unimplemented!();
        unimplemented!();
        let mut jwv: () = unimplemented!();
        let mut jwv_iter: () = unimplemented!();
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
    (unsafe { test_46() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_29);
}
