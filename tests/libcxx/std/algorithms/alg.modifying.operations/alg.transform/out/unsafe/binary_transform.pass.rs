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
pub unsafe fn base_0(i: *const cpp17_output_iterator_int_ptr_) -> *mut i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub unsafe fn new(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
    }
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
impl UnsafeCppAdvance for cpp17_output_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
pub unsafe fn base_1(i: *const cpp17_input_iterator_int_ptr__int_ptr_) -> *mut i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn new(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
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
impl UnsafeCppAdvance for cpp17_input_iterator_int_ptr__int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
pub unsafe fn operator_ne_2(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
}
impl UnsafeCppAdvance for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn base_3(i: *const forward_iterator_int_ptr_) -> *mut i32 {
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
impl UnsafeCppAdvance for forward_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_ne_4(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_int_ptr_,
    ) -> *mut forward_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
}
impl UnsafeCppAdvance for forward_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn base_5(i: *const bidirectional_iterator_int_ptr_) -> *mut i32 {
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
impl UnsafeCppAdvance for bidirectional_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_ne_6(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_const_int_ptr_,
    ) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
}
impl UnsafeCppAdvance for bidirectional_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn base_7(i: *const random_access_iterator_int_ptr_) -> *mut i32 {
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
impl UnsafeCppAdvance for random_access_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
pub unsafe fn operator_ne_8(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_int_ptr_,
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
}
impl UnsafeCppAdvance for random_access_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
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
pub unsafe fn base_9(mut i: *mut i32) -> *mut i32 {
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
pub unsafe fn test_constexpr_17() -> bool {
    let ia: [i32; 4] = [1, 3, 6, 7];
    let ib: [i32; 4] = [2, 4, 7, 8];
    let mut ic: [i32; 5] = [0, 0, 0, 0, 0];
    let expected: [i32; 4] = [3, 7, 13, 15];
    let mut it: *mut i32 = unimplemented!();
    return (((it) == (unimplemented!().offset((unimplemented!()) as isize))) && ((*it) == (0)))
        && (unimplemented!());
}
pub unsafe fn test_18() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_19() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_20() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_21() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_22() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_23() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_24() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_25() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_26() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_27() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_28() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_29() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_30() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_31() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_32() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_33() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_34() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_35() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_36() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_37() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_38() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_39() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_40() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_41() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_42() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_43() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_44() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_45() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_46() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_47() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_48() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_49() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_50() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_51() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_52() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_53() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_54() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_55() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_56() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_57() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_58() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_59() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_60() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_61() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_62() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_63() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_64() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_65() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_66() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_67() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_68() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_69() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_70() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_71() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_72() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_73() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_74() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_75() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_76() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_77() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_78() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_79() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_80() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_81() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_82() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_83() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_84() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_85() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_86() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_87() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_88() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_89() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_90() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_91() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_92() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_93() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_94() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_95() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_96() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_97() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_98() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_99() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_100() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_101() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_102() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_103() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_104() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_105() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_106() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_107() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_108() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_109() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_110() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_111() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_112() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_113() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_114() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_115() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_116() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_117() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_118() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_119() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_120() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_121() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_122() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_123() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_124() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_125() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_126() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_127() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_128() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_129() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_130() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_131() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_132() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_133() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_134() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_135() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_136() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_137() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_138() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_139() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_140() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_141() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_142() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_143() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_144() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_145() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_146() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_147() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_148() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_149() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_150() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_151() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_152() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_153() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_154() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_155() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_156() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_157() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_158() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_159() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_160() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_161() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_162() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_163() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = unimplemented!();
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_164() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_165() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_166() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = unimplemented!();
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_167() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = unimplemented!();
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
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
    (unsafe { test_18() });
    (unsafe { test_19() });
    (unsafe { test_20() });
    (unsafe { test_21() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
    (unsafe { test_31() });
    (unsafe { test_32() });
    (unsafe { test_33() });
    (unsafe { test_34() });
    (unsafe { test_35() });
    (unsafe { test_36() });
    (unsafe { test_37() });
    (unsafe { test_38() });
    (unsafe { test_39() });
    (unsafe { test_40() });
    (unsafe { test_41() });
    (unsafe { test_42() });
    (unsafe { test_43() });
    (unsafe { test_44() });
    (unsafe { test_45() });
    (unsafe { test_46() });
    (unsafe { test_47() });
    (unsafe { test_48() });
    (unsafe { test_49() });
    (unsafe { test_50() });
    (unsafe { test_51() });
    (unsafe { test_52() });
    (unsafe { test_53() });
    (unsafe { test_54() });
    (unsafe { test_55() });
    (unsafe { test_56() });
    (unsafe { test_57() });
    (unsafe { test_58() });
    (unsafe { test_59() });
    (unsafe { test_60() });
    (unsafe { test_61() });
    (unsafe { test_62() });
    (unsafe { test_63() });
    (unsafe { test_64() });
    (unsafe { test_65() });
    (unsafe { test_66() });
    (unsafe { test_67() });
    (unsafe { test_68() });
    (unsafe { test_69() });
    (unsafe { test_70() });
    (unsafe { test_71() });
    (unsafe { test_72() });
    (unsafe { test_73() });
    (unsafe { test_74() });
    (unsafe { test_75() });
    (unsafe { test_76() });
    (unsafe { test_77() });
    (unsafe { test_78() });
    (unsafe { test_79() });
    (unsafe { test_80() });
    (unsafe { test_81() });
    (unsafe { test_82() });
    (unsafe { test_83() });
    (unsafe { test_84() });
    (unsafe { test_85() });
    (unsafe { test_86() });
    (unsafe { test_87() });
    (unsafe { test_88() });
    (unsafe { test_89() });
    (unsafe { test_90() });
    (unsafe { test_91() });
    (unsafe { test_92() });
    (unsafe { test_93() });
    (unsafe { test_94() });
    (unsafe { test_95() });
    (unsafe { test_96() });
    (unsafe { test_97() });
    (unsafe { test_98() });
    (unsafe { test_99() });
    (unsafe { test_100() });
    (unsafe { test_101() });
    (unsafe { test_102() });
    (unsafe { test_103() });
    (unsafe { test_104() });
    (unsafe { test_105() });
    (unsafe { test_106() });
    (unsafe { test_107() });
    (unsafe { test_108() });
    (unsafe { test_109() });
    (unsafe { test_110() });
    (unsafe { test_111() });
    (unsafe { test_112() });
    (unsafe { test_113() });
    (unsafe { test_114() });
    (unsafe { test_115() });
    (unsafe { test_116() });
    (unsafe { test_117() });
    (unsafe { test_118() });
    (unsafe { test_119() });
    (unsafe { test_120() });
    (unsafe { test_121() });
    (unsafe { test_122() });
    (unsafe { test_123() });
    (unsafe { test_124() });
    (unsafe { test_125() });
    (unsafe { test_126() });
    (unsafe { test_127() });
    (unsafe { test_128() });
    (unsafe { test_129() });
    (unsafe { test_130() });
    (unsafe { test_131() });
    (unsafe { test_132() });
    (unsafe { test_133() });
    (unsafe { test_134() });
    (unsafe { test_135() });
    (unsafe { test_136() });
    (unsafe { test_137() });
    (unsafe { test_138() });
    (unsafe { test_139() });
    (unsafe { test_140() });
    (unsafe { test_141() });
    (unsafe { test_142() });
    (unsafe { test_143() });
    (unsafe { test_144() });
    (unsafe { test_145() });
    (unsafe { test_146() });
    (unsafe { test_147() });
    (unsafe { test_148() });
    (unsafe { test_149() });
    (unsafe { test_150() });
    (unsafe { test_151() });
    (unsafe { test_152() });
    (unsafe { test_153() });
    (unsafe { test_154() });
    (unsafe { test_155() });
    (unsafe { test_156() });
    (unsafe { test_157() });
    (unsafe { test_158() });
    (unsafe { test_159() });
    (unsafe { test_160() });
    (unsafe { test_161() });
    (unsafe { test_162() });
    (unsafe { test_163() });
    (unsafe { test_164() });
    (unsafe { test_165() });
    (unsafe { test_166() });
    (unsafe { test_167() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
