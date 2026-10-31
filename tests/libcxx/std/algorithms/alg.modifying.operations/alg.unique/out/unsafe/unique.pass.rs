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
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_1(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_2(i: *const forward_iterator_int_ptr_) -> *mut i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_int_ptr_,
    ) -> *mut forward_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const forward_iterator_int_ptr_,
                other as *const forward_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_int_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<i32> for forward_iterator_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { forward_iterator_int_ptr_::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            forward_iterator_int_ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for forward_iterator_int_ptr_ {}
pub unsafe fn operator_eq_3(
    x: *const forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
    y: *const forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
    y: *const forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_5(
    i: *const forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> *mut Option<Box<i32>> {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    it_: *mut Option<Box<i32>>,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    pub unsafe fn new_1(mut it: *mut Option<Box<i32>>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Option<Box<i32>> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(
        &mut self,
    ) -> *mut forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_std_unique_ptr_int__do_nothing__ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
    ) -> *mut forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_std_unique_ptr_int__do_nothing__ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
                other as *const forward_iterator_std_unique_ptr_int__do_nothing__ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {}
impl UnsafeCppAdvance for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Option<Box<i32>>> for forward_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn iter_deref(&self) -> *mut Option<Box<i32>> {
        unsafe {
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::operator_deref(self)
                as *mut Option<Box<i32>>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Option<Box<i32>>>
    for forward_iterator_std_unique_ptr_int__do_nothing__ptr_
{
}
pub unsafe fn operator_eq_6(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_7(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_8(i: *const bidirectional_iterator_int_ptr_) -> *mut i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_int_ptr_,
    ) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<i32> for bidirectional_iterator_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { bidirectional_iterator_int_ptr_::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            bidirectional_iterator_int_ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for bidirectional_iterator_int_ptr_ {}
pub unsafe fn operator_eq_9(
    x: *const bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
    y: *const bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_10(
    x: *const bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
    y: *const bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_11(
    i: *const bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> *mut Option<Box<i32>> {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    it_: *mut Option<Box<i32>>,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    pub unsafe fn new_1(mut it: *mut Option<Box<i32>>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Option<Box<i32>> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(
        &mut self,
    ) -> *mut bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
    ) -> *mut bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
                other as *const bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Option<Box<i32>>>
    for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_
{
    fn iter_deref(&self) -> *mut Option<Box<i32>> {
        unsafe {
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::operator_deref(self)
                as *mut Option<Box<i32>>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Option<Box<i32>>>
    for bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_
{
}
pub unsafe fn operator_eq_12(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_13(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_14(i: *const random_access_iterator_int_ptr_) -> *mut i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_int_ptr_,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
impl UnsafeCppIterator<i32> for random_access_iterator_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { random_access_iterator_int_ptr_::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            random_access_iterator_int_ptr_::operator_inc_7(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for random_access_iterator_int_ptr_ {}
pub unsafe fn operator_eq_15(
    x: *const random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
    y: *const random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_16(
    x: *const random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
    y: *const random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_17(
    i: *const random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
) -> *mut Option<Box<i32>> {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    it_: *mut Option<Box<i32>>,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    pub unsafe fn new_1(mut it: *mut Option<Box<i32>>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Option<Box<i32>> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(
        &mut self,
    ) -> *mut random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_std_unique_ptr_int__do_nothing__ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
    ) -> *mut random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_std_unique_ptr_int__do_nothing__ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_15(
                self as *const random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
                other as *const random_access_iterator_std_unique_ptr_int__do_nothing__ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
impl UnsafeCppIterator<Option<Box<i32>>>
    for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_
{
    fn iter_deref(&self) -> *mut Option<Box<i32>> {
        unsafe {
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::operator_deref(self)
                as *mut Option<Box<i32>>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::operator_inc_7(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Option<Box<i32>>>
    for random_access_iterator_std_unique_ptr_int__do_nothing__ptr_
{
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
pub unsafe fn base_18(mut i: *mut i32) -> *mut i32 {
    return i;
}
pub unsafe fn base_19(mut i: *mut Option<Box<i32>>) -> *mut Option<Box<i32>> {
    return i;
}
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
pub unsafe fn operator_eq_20(
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
            operator_eq_20(
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
pub unsafe fn operator_add_21(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_22(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_21(_i, n)
    });
}
pub unsafe fn iter_swap_23(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_24(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_25(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_26(
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
        unsafe { operator_cmp_26(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_25(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_constexpr_27() -> bool {
    let mut ia: [i32; 5] = [0, 1, 1, 3, 4];
    let expected: [i32; 4] = [0, 1, 3, 4];
    let N: usize = 4_usize;
    let mut it: *mut i32 = {
        let __first = unimplemented!();
        let __last = (ia.as_ptr().add(ia.len()) as *mut i32);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    return ((it) == (unimplemented!().offset((N) as isize))) && (unimplemented!());
}
pub unsafe fn test_28() {
    let mut ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut r: forward_iterator_int_ptr_ = {
        let __first = forward_iterator_int_ptr_::new_1({ ia.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ ia.as_mut_ptr().offset((sa) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(((ia[(0) as usize]) == (0)));
    let mut ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr().offset((sb) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(((ib[(0) as usize]) == (0)));
    assert!(((ib[(1) as usize]) == (1)));
    let mut ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = forward_iterator_int_ptr_::new_1({ ic.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ ic.as_mut_ptr().offset((sc) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(((ic[(0) as usize]) == (0)));
    let mut id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = forward_iterator_int_ptr_::new_1({ id.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ id.as_mut_ptr().offset((sd) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(((id[(0) as usize]) == (0)));
    assert!(((id[(1) as usize]) == (1)));
    let mut ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = forward_iterator_int_ptr_::new_1({ ie.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ ie.as_mut_ptr().offset((se) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(((ie[(0) as usize]) == (0)));
    assert!(((ie[(1) as usize]) == (1)));
    assert!(((ie[(2) as usize]) == (0)));
    let mut ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = forward_iterator_int_ptr_::new_1({ ig.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ ig.as_mut_ptr().offset((sg) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(((ig[(0) as usize]) == (0)));
    assert!(((ig[(1) as usize]) == (1)));
    let mut ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = forward_iterator_int_ptr_::new_1({ ih.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ ih.as_mut_ptr().offset((sh) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(((ih[(0) as usize]) == (0)));
    assert!(((ih[(1) as usize]) == (1)));
    let mut ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = forward_iterator_int_ptr_::new_1({ ii.as_mut_ptr() });
        let __last = forward_iterator_int_ptr_::new_1({ ii.as_mut_ptr().offset((si) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_2(&r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(((ii[(0) as usize]) == (0)));
    assert!(((ii[(1) as usize]) == (1)));
    assert!(((ii[(2) as usize]) == (2)));
}
pub unsafe fn test_29() {
    let mut ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut r: bidirectional_iterator_int_ptr_ = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ ia.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ ia.as_mut_ptr().offset((sa) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(((ia[(0) as usize]) == (0)));
    let mut ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr().offset((sb) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(((ib[(0) as usize]) == (0)));
    assert!(((ib[(1) as usize]) == (1)));
    let mut ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ ic.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ ic.as_mut_ptr().offset((sc) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(((ic[(0) as usize]) == (0)));
    let mut id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ id.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ id.as_mut_ptr().offset((sd) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(((id[(0) as usize]) == (0)));
    assert!(((id[(1) as usize]) == (1)));
    let mut ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ ie.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ ie.as_mut_ptr().offset((se) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(((ie[(0) as usize]) == (0)));
    assert!(((ie[(1) as usize]) == (1)));
    assert!(((ie[(2) as usize]) == (0)));
    let mut ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ ig.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ ig.as_mut_ptr().offset((sg) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(((ig[(0) as usize]) == (0)));
    assert!(((ig[(1) as usize]) == (1)));
    let mut ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ ih.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ ih.as_mut_ptr().offset((sh) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(((ih[(0) as usize]) == (0)));
    assert!(((ih[(1) as usize]) == (1)));
    let mut ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = bidirectional_iterator_int_ptr_::new_1({ ii.as_mut_ptr() });
        let __last =
            bidirectional_iterator_int_ptr_::new_1({ ii.as_mut_ptr().offset((si) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_8(&r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(((ii[(0) as usize]) == (0)));
    assert!(((ii[(1) as usize]) == (1)));
    assert!(((ii[(2) as usize]) == (2)));
}
pub unsafe fn test_30() {
    let mut ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut r: random_access_iterator_int_ptr_ = {
        let __first = random_access_iterator_int_ptr_::new_1({ ia.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ ia.as_mut_ptr().offset((sa) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(((ia[(0) as usize]) == (0)));
    let mut ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr().offset((sb) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(((ib[(0) as usize]) == (0)));
    assert!(((ib[(1) as usize]) == (1)));
    let mut ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = random_access_iterator_int_ptr_::new_1({ ic.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ ic.as_mut_ptr().offset((sc) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(((ic[(0) as usize]) == (0)));
    let mut id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = random_access_iterator_int_ptr_::new_1({ id.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ id.as_mut_ptr().offset((sd) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(((id[(0) as usize]) == (0)));
    assert!(((id[(1) as usize]) == (1)));
    let mut ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = random_access_iterator_int_ptr_::new_1({ ie.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ ie.as_mut_ptr().offset((se) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(((ie[(0) as usize]) == (0)));
    assert!(((ie[(1) as usize]) == (1)));
    assert!(((ie[(2) as usize]) == (0)));
    let mut ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = random_access_iterator_int_ptr_::new_1({ ig.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ ig.as_mut_ptr().offset((sg) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(((ig[(0) as usize]) == (0)));
    assert!(((ig[(1) as usize]) == (1)));
    let mut ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = random_access_iterator_int_ptr_::new_1({ ih.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ ih.as_mut_ptr().offset((sh) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(((ih[(0) as usize]) == (0)));
    assert!(((ih[(1) as usize]) == (1)));
    let mut ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = random_access_iterator_int_ptr_::new_1({ ii.as_mut_ptr() });
        let __last =
            random_access_iterator_int_ptr_::new_1({ ii.as_mut_ptr().offset((si) as isize) });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_14(&r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(((ii[(0) as usize]) == (0)));
    assert!(((ii[(1) as usize]) == (1)));
    assert!(((ii[(2) as usize]) == (2)));
}
pub unsafe fn test_31() {
    let mut ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut r: *mut i32 = {
        let __first = ia.as_mut_ptr();
        let __last = ia.as_mut_ptr().offset((sa) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(((ia[(0) as usize]) == (0)));
    let mut ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = ib.as_mut_ptr();
        let __last = ib.as_mut_ptr().offset((sb) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(((ib[(0) as usize]) == (0)));
    assert!(((ib[(1) as usize]) == (1)));
    let mut ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = ic.as_mut_ptr();
        let __last = ic.as_mut_ptr().offset((sc) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(((ic[(0) as usize]) == (0)));
    let mut id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = id.as_mut_ptr();
        let __last = id.as_mut_ptr().offset((sd) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(((id[(0) as usize]) == (0)));
    assert!(((id[(1) as usize]) == (1)));
    let mut ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = ie.as_mut_ptr();
        let __last = ie.as_mut_ptr().offset((se) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(((ie[(0) as usize]) == (0)));
    assert!(((ie[(1) as usize]) == (1)));
    assert!(((ie[(2) as usize]) == (0)));
    let mut ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = ig.as_mut_ptr();
        let __last = ig.as_mut_ptr().offset((sg) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(((ig[(0) as usize]) == (0)));
    assert!(((ig[(1) as usize]) == (1)));
    let mut ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = ih.as_mut_ptr();
        let __last = ih.as_mut_ptr().offset((sh) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(((ih[(0) as usize]) == (0)));
    assert!(((ih[(1) as usize]) == (1)));
    let mut ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    r = {
        let __first = ii.as_mut_ptr();
        let __last = ii.as_mut_ptr().offset((si) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_18(r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(((ii[(0) as usize]) == (0)));
    assert!(((ii[(1) as usize]) == (1)));
    assert!(((ii[(2) as usize]) == (2)));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct do_nothing {}
impl do_nothing {
    pub unsafe fn operator_call(&self, mut _a0: *mut ::libc::c_void) {}
}
impl Callable1<*mut ::libc::c_void, ()> for do_nothing {
    fn call(&self, a1: *mut ::libc::c_void) -> () {
        unsafe { do_nothing::operator_call(self, a1) }
    }
}
pub unsafe fn test1_32() {
    let mut one: i32 = 1;
    let mut two: i32 = 2;
    let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
    let sa: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    let mut r: forward_iterator_std_unique_ptr_int__do_nothing__ptr_ = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ia.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ia.as_mut_ptr().offset((sa) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(ia[(0) as usize].is_none());
    let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ib[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sb: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ib.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ib.as_mut_ptr().offset((sb) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(ib[(0) as usize].is_none());
    assert!(
        ((*ib[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ic: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    let sc: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ic.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ic.as_mut_ptr().offset((sc) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(ic[(0) as usize].is_none());
    let mut id: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        id[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sd: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ id.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            id.as_mut_ptr().offset((sd) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(id[(0) as usize].is_none());
    assert!(
        ((*id[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ie: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ie[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let se: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ie.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ie.as_mut_ptr().offset((se) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(ie[(0) as usize].is_none());
    assert!(
        ((*ie[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(ie[(2) as usize].is_none());
    let mut ig: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sg: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ig.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ig.as_mut_ptr().offset((sg) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(ig[(0) as usize].is_none());
    assert!(
        ((*ig[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ih: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sh: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ih.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ih.as_mut_ptr().offset((sh) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(ih[(0) as usize].is_none());
    assert!(
        ((*ih[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ii: [Option<Box<i32>>; 7] = std::array::from_fn::<_, 7, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(4) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(5) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(6) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let si: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ii.as_mut_ptr() });
        let __last = forward_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ii.as_mut_ptr().offset((si) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_5(&r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(ii[(0) as usize].is_none());
    assert!(
        ((*ii[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(
        ((*ii[(2) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (2))
    );
}
pub unsafe fn test1_33() {
    let mut one: i32 = 1;
    let mut two: i32 = 2;
    let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
    let sa: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    let mut r: bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_ = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ia.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ia.as_mut_ptr().offset((sa) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(ia[(0) as usize].is_none());
    let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ib[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sb: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ib.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ib.as_mut_ptr().offset((sb) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(ib[(0) as usize].is_none());
    assert!(
        ((*ib[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ic: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    let sc: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ic.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ic.as_mut_ptr().offset((sc) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(ic[(0) as usize].is_none());
    let mut id: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        id[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sd: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ id.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            id.as_mut_ptr().offset((sd) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(id[(0) as usize].is_none());
    assert!(
        ((*id[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ie: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ie[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let se: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ie.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ie.as_mut_ptr().offset((se) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(ie[(0) as usize].is_none());
    assert!(
        ((*ie[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(ie[(2) as usize].is_none());
    let mut ig: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sg: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ig.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ig.as_mut_ptr().offset((sg) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(ig[(0) as usize].is_none());
    assert!(
        ((*ig[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ih: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sh: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ih.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ih.as_mut_ptr().offset((sh) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(ih[(0) as usize].is_none());
    assert!(
        ((*ih[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ii: [Option<Box<i32>>; 7] = std::array::from_fn::<_, 7, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(4) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(5) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(6) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let si: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ii.as_mut_ptr() });
        let __last = bidirectional_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ii.as_mut_ptr().offset((si) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_11(&r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(ii[(0) as usize].is_none());
    assert!(
        ((*ii[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(
        ((*ii[(2) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (2))
    );
}
pub unsafe fn test1_34() {
    let mut one: i32 = 1;
    let mut two: i32 = 2;
    let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
    let sa: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    let mut r: random_access_iterator_std_unique_ptr_int__do_nothing__ptr_ = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ia.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ia.as_mut_ptr().offset((sa) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(ia[(0) as usize].is_none());
    let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ib[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sb: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ib.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ib.as_mut_ptr().offset((sb) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(ib[(0) as usize].is_none());
    assert!(
        ((*ib[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ic: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    let sc: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ic.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ic.as_mut_ptr().offset((sc) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(ic[(0) as usize].is_none());
    let mut id: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        id[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sd: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ id.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            id.as_mut_ptr().offset((sd) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(id[(0) as usize].is_none());
    assert!(
        ((*id[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ie: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ie[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let se: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ie.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ie.as_mut_ptr().offset((se) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(ie[(0) as usize].is_none());
    assert!(
        ((*ie[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(ie[(2) as usize].is_none());
    let mut ig: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sg: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ig.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ig.as_mut_ptr().offset((sg) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(ig[(0) as usize].is_none());
    assert!(
        ((*ig[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ih: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sh: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ih.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ih.as_mut_ptr().offset((sh) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(ih[(0) as usize].is_none());
    assert!(
        ((*ih[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ii: [Option<Box<i32>>; 7] = std::array::from_fn::<_, 7, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(4) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(5) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(6) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let si: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first =
            random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({ ii.as_mut_ptr() });
        let __last = random_access_iterator_std_unique_ptr_int__do_nothing__ptr_::new_1({
            ii.as_mut_ptr().offset((si) as isize)
        });
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_17(&r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(ii[(0) as usize].is_none());
    assert!(
        ((*ii[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(
        ((*ii[(2) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (2))
    );
}
pub unsafe fn test1_35() {
    let mut one: i32 = 1;
    let mut two: i32 = 2;
    let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
    let sa: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    let mut r: *mut Option<Box<i32>> = {
        let __first = ia.as_mut_ptr();
        let __last = ia.as_mut_ptr().offset((sa) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
    assert!(ia[(0) as usize].is_none());
    let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ib[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sb: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first = ib.as_mut_ptr();
        let __last = ib.as_mut_ptr().offset((sb) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
    assert!(ib[(0) as usize].is_none());
    assert!(
        ((*ib[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ic: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
    let sc: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first = ic.as_mut_ptr();
        let __last = ic.as_mut_ptr().offset((sc) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
    assert!(ic[(0) as usize].is_none());
    let mut id: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        id[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sd: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first = id.as_mut_ptr();
        let __last = id.as_mut_ptr().offset((sd) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (id.as_mut_ptr().offset((2) as isize))));
    assert!(id[(0) as usize].is_none());
    assert!(
        ((*id[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ie: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ie[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let se: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first = ie.as_mut_ptr();
        let __last = ie.as_mut_ptr().offset((se) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
    assert!(ie[(0) as usize].is_none());
    assert!(
        ((*ie[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(ie[(2) as usize].is_none());
    let mut ig: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ig[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sg: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first = ig.as_mut_ptr();
        let __last = ig.as_mut_ptr().offset((sg) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
    assert!(ig[(0) as usize].is_none());
    assert!(
        ((*ig[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ih: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ih[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let sh: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first = ih.as_mut_ptr();
        let __last = ih.as_mut_ptr().offset((sh) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (ih.as_mut_ptr().offset((2) as isize))));
    assert!(ih[(0) as usize].is_none());
    assert!(
        ((*ih[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    let mut ii: [Option<Box<i32>>; 7] = std::array::from_fn::<_, 7, _>(|_| None);
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(1) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(2) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut one as *mut i32);
        ii[(3) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(4) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(5) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    {
        let _a0: *mut i32 = (&mut two as *mut i32);
        ii[(6) as usize] = if _a0.is_null() {
            None
        } else {
            Some(Box::from_raw(_a0))
        }
    };
    let si: u32 = (((::std::mem::size_of::<[Option<Box<i32>>; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
        as u32);
    r = {
        let __first = ii.as_mut_ptr();
        let __last = ii.as_mut_ptr().offset((si) as isize);
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    assert!(((unsafe { base_19(r,) }) == (ii.as_mut_ptr().offset((3) as isize))));
    assert!(ii[(0) as usize].is_none());
    assert!(
        ((*ii[(1) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (1))
    );
    assert!(
        ((*ii[(2) as usize]
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
            == (2))
    );
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
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
    (unsafe { test_31() });
    (unsafe { test1_32() });
    (unsafe { test1_33() });
    (unsafe { test1_34() });
    (unsafe { test1_35() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
