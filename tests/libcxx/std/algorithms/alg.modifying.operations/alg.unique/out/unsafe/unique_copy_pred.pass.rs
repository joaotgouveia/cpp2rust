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
pub unsafe fn operator_eq_1(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
impl std::cmp::PartialEq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
                other as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
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
pub unsafe fn operator_eq_4(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_5(
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
pub unsafe fn base_6(i: *const bidirectional_iterator_int_ptr_) -> *mut i32 {
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
pub unsafe fn operator_eq_7(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_8(
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
pub unsafe fn base_9(i: *const random_access_iterator_int_ptr_) -> *mut i32 {
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
pub unsafe fn operator_eq_10(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_11(
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
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
pub unsafe fn base_12(mut i: *mut i32) -> *mut i32 {
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
pub unsafe fn operator_eq_13(
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
            operator_eq_13(
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
pub unsafe fn operator_add_14(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_15(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_14(_i, n)
    });
}
pub unsafe fn iter_swap_16(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_17(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_18(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_19(
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
        unsafe { operator_cmp_19(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_constexpr_20() -> bool {
    let mut ia: [i32; 5] = [0, 1, 2, 2, 4];
    let mut ib: [i32; 5] = [0, 0, 0, 0, 0];
    let expected: [i32; 4] = [0, 1, 2, 4];
    let mut it: *mut i32 = (unsafe {
        let ___first: *mut i32 = (unsafe { begin_21(&mut ia) });
        let ___last: *mut i32 = (unsafe { end_22(&mut ia) });
        let ___result: *mut i32 = (unsafe { begin_21(&mut ib) });
        unique_copy_23(
            ___first,
            ___last,
            ___result,
            FnPtr::<fn(i32, i32) -> bool>::new(|mut a: i32, mut b: i32| -> bool {
                unsafe {
                    return ((a) == (b));
                }
            }),
        )
    });
    return (((it)
        == ((unsafe { begin_21(&mut ib) }).offset((unsafe { size_24(&expected) }) as isize)))
        && ((*it) == (0)))
        && (unsafe {
            let ___first2: *const i32 = (unsafe { begin_25(&expected) });
            let ___last2: *const i32 = expected.as_ptr().add(expected.len());
            equal_26((unsafe { begin_21(&mut ib) }), it, ___first2, ___last2)
        });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct count_equal {}
impl count_equal {
    pub unsafe fn operator_call_1_int(&mut self, x: *const i32, y: *const i32) -> bool {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)).prefix_inc();
        return ((*x) == (*y));
    }
}
pub static mut count_27: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
pub unsafe fn test_28() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ia.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ia.as_ptr().offset((sa) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ib.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ib.as_ptr().offset((sb) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ic.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ic.as_ptr().offset((sc) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ id.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                id.as_ptr().offset((sd) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ie.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ie.as_ptr().offset((se) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ig.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ig.as_ptr().offset((sg) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ih.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ih.as_ptr().offset((sh) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ii.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ii.as_ptr().offset((si) as isize)
            });
        unique_copy_29(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_30() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ia.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ia.as_ptr().offset((sa) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ib.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ib.as_ptr().offset((sb) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ic.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ic.as_ptr().offset((sc) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ id.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                id.as_ptr().offset((sd) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ie.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ie.as_ptr().offset((se) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ig.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ig.as_ptr().offset((sg) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ih.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ih.as_ptr().offset((sh) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ii.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ii.as_ptr().offset((si) as isize)
            });
        unique_copy_31(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_32() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ia.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ia.as_ptr().offset((sa) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ib.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ib.as_ptr().offset((sb) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ic.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ic.as_ptr().offset((sc) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ id.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                id.as_ptr().offset((sd) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ie.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ie.as_ptr().offset((se) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ig.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ig.as_ptr().offset((sg) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ih.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ih.as_ptr().offset((sh) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ii.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ii.as_ptr().offset((si) as isize)
            });
        unique_copy_33(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_34() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ia.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ia.as_ptr().offset((sa) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ib.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ib.as_ptr().offset((sb) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ic.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ic.as_ptr().offset((sc) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ id.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                id.as_ptr().offset((sd) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ie.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ie.as_ptr().offset((se) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ig.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ig.as_ptr().offset((sg) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ih.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ih.as_ptr().offset((sh) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ii.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ii.as_ptr().offset((si) as isize)
            });
        unique_copy_35(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_36() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: *mut i32 = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ia.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ia.as_ptr().offset((sa) as isize)
            });
        unique_copy_37(___first, ___last, ja.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ib.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ib.as_ptr().offset((sb) as isize)
            });
        unique_copy_37(___first, ___last, jb.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ic.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ic.as_ptr().offset((sc) as isize)
            });
        unique_copy_37(___first, ___last, jc.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ id.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                id.as_ptr().offset((sd) as isize)
            });
        unique_copy_37(___first, ___last, jd.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ie.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ie.as_ptr().offset((se) as isize)
            });
        unique_copy_37(___first, ___last, je.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ig.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ig.as_ptr().offset((sg) as isize)
            });
        unique_copy_37(___first, ___last, jg.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ih.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ih.as_ptr().offset((sh) as isize)
            });
        unique_copy_37(___first, ___last, jh.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({ ii.as_ptr() });
        let ___last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                ii.as_ptr().offset((si) as isize)
            });
        unique_copy_37(___first, ___last, ji.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_38() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_39(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_40() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_41(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_42() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_43(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_44() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_45(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_46() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: *mut i32 = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_47(___first, ___last, ja.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_47(___first, ___last, jb.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_47(___first, ___last, jc.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_47(___first, ___last, jd.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_47(___first, ___last, je.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_47(___first, ___last, jg.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_47(___first, ___last, jh.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_47(___first, ___last, ji.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_48() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_49(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_50() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_51(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_52() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_53(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_54() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_55(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_56() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: *mut i32 = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_57(___first, ___last, ja.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_57(___first, ___last, jb.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_57(___first, ___last, jc.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_57(___first, ___last, jd.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_57(___first, ___last, je.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_57(___first, ___last, jg.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_57(___first, ___last, jh.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_57(___first, ___last, ji.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_58() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_59(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_60() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_61(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_62() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_63(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_64() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_65(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_66() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: *mut i32 = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ia.as_ptr().offset((sa) as isize) });
        unique_copy_67(___first, ___last, ja.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ib.as_ptr().offset((sb) as isize) });
        unique_copy_67(___first, ___last, jb.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ic.as_ptr().offset((sc) as isize) });
        unique_copy_67(___first, ___last, jc.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ id.as_ptr().offset((sd) as isize) });
        unique_copy_67(___first, ___last, jd.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ie.as_ptr().offset((se) as isize) });
        unique_copy_67(___first, ___last, je.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ig.as_ptr().offset((sg) as isize) });
        unique_copy_67(___first, ___last, jg.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ih.as_ptr().offset((sh) as isize) });
        unique_copy_67(___first, ___last, jh.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr() });
        let ___last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ ii.as_ptr().offset((si) as isize) });
        unique_copy_67(___first, ___last, ji.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_68() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first: *const i32 = ia.as_ptr();
        let ___last: *const i32 = ia.as_ptr().offset((sa) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ib.as_ptr();
        let ___last: *const i32 = ib.as_ptr().offset((sb) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ic.as_ptr();
        let ___last: *const i32 = ic.as_ptr().offset((sc) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = id.as_ptr();
        let ___last: *const i32 = id.as_ptr().offset((sd) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ie.as_ptr();
        let ___last: *const i32 = ie.as_ptr().offset((se) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ig.as_ptr();
        let ___last: *const i32 = ig.as_ptr().offset((sg) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ih.as_ptr();
        let ___last: *const i32 = ih.as_ptr().offset((sh) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ii.as_ptr();
        let ___last: *const i32 = ii.as_ptr().offset((si) as isize);
        unique_copy_69(
            ___first,
            ___last,
            cpp17_output_iterator_int_ptr_::new({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_70() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first: *const i32 = ia.as_ptr();
        let ___last: *const i32 = ia.as_ptr().offset((sa) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ib.as_ptr();
        let ___last: *const i32 = ib.as_ptr().offset((sb) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ic.as_ptr();
        let ___last: *const i32 = ic.as_ptr().offset((sc) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = id.as_ptr();
        let ___last: *const i32 = id.as_ptr().offset((sd) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ie.as_ptr();
        let ___last: *const i32 = ie.as_ptr().offset((se) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ig.as_ptr();
        let ___last: *const i32 = ig.as_ptr().offset((sg) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ih.as_ptr();
        let ___last: *const i32 = ih.as_ptr().offset((sh) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ii.as_ptr();
        let ___last: *const i32 = ii.as_ptr().offset((si) as isize);
        unique_copy_71(
            ___first,
            ___last,
            forward_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_72() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first: *const i32 = ia.as_ptr();
        let ___last: *const i32 = ia.as_ptr().offset((sa) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ib.as_ptr();
        let ___last: *const i32 = ib.as_ptr().offset((sb) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ic.as_ptr();
        let ___last: *const i32 = ic.as_ptr().offset((sc) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = id.as_ptr();
        let ___last: *const i32 = id.as_ptr().offset((sd) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ie.as_ptr();
        let ___last: *const i32 = ie.as_ptr().offset((se) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ig.as_ptr();
        let ___last: *const i32 = ig.as_ptr().offset((sg) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ih.as_ptr();
        let ___last: *const i32 = ih.as_ptr().offset((sh) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ii.as_ptr();
        let ___last: *const i32 = ii.as_ptr().offset((si) as isize);
        unique_copy_73(
            ___first,
            ___last,
            bidirectional_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_6(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_74() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first: *const i32 = ia.as_ptr();
        let ___last: *const i32 = ia.as_ptr().offset((sa) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ja.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ib.as_ptr();
        let ___last: *const i32 = ib.as_ptr().offset((sb) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jb.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ic.as_ptr();
        let ___last: *const i32 = ic.as_ptr().offset((sc) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jc.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = id.as_ptr();
        let ___last: *const i32 = id.as_ptr().offset((sd) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jd.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ie.as_ptr();
        let ___last: *const i32 = ie.as_ptr().offset((se) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ je.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ig.as_ptr();
        let ___last: *const i32 = ig.as_ptr().offset((sg) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jg.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ih.as_ptr();
        let ___last: *const i32 = ih.as_ptr().offset((sh) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ jh.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ii.as_ptr();
        let ___last: *const i32 = ii.as_ptr().offset((si) as isize);
        unique_copy_75(
            ___first,
            ___last,
            random_access_iterator_int_ptr_::new_1({ ji.as_mut_ptr() }),
            <count_equal>::default(),
        )
    });
    assert!(((unsafe { base_9(&r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
    );
}
pub unsafe fn test_76() {
    let ia: [i32; 1] = [0];
    let sa: u32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ja: [i32; 1] = [-1_i32];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    let mut r: *mut i32 = (unsafe {
        let ___first: *const i32 = ia.as_ptr();
        let ___last: *const i32 = ia.as_ptr().offset((sa) as isize);
        unique_copy_77(___first, ___last, ja.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ja.as_mut_ptr().offset((sa) as isize))));
    assert!(((ja[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sa).wrapping_sub(1_u32)))
    );
    let ib: [i32; 2] = [0, 1];
    let sb: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jb: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ib.as_ptr();
        let ___last: *const i32 = ib.as_ptr().offset((sb) as isize);
        unique_copy_77(___first, ___last, jb.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jb.as_mut_ptr().offset((sb) as isize))));
    assert!(((jb[(0) as usize]) == (0)));
    assert!(((jb[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sb).wrapping_sub(1_u32)))
    );
    let ic: [i32; 2] = [0, 0];
    let sc: u32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jc: [i32; 2] = [-1_i32, <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ic.as_ptr();
        let ___last: *const i32 = ic.as_ptr().offset((sc) as isize);
        unique_copy_77(___first, ___last, jc.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jc.as_mut_ptr().offset((1) as isize))));
    assert!(((jc[(0) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sc).wrapping_sub(1_u32)))
    );
    let id: [i32; 3] = [0, 0, 1];
    let sd: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jd: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = id.as_ptr();
        let ___last: *const i32 = id.as_ptr().offset((sd) as isize);
        unique_copy_77(___first, ___last, jd.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jd.as_mut_ptr().offset((2) as isize))));
    assert!(((jd[(0) as usize]) == (0)));
    assert!(((jd[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sd).wrapping_sub(1_u32)))
    );
    let ie: [i32; 4] = [0, 0, 1, 0];
    let se: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut je: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ie.as_ptr();
        let ___last: *const i32 = ie.as_ptr().offset((se) as isize);
        unique_copy_77(___first, ___last, je.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (je.as_mut_ptr().offset((3) as isize))));
    assert!(((je[(0) as usize]) == (0)));
    assert!(((je[(1) as usize]) == (1)));
    assert!(((je[(2) as usize]) == (0)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((se).wrapping_sub(1_u32)))
    );
    let ig: [i32; 4] = [0, 0, 1, 1];
    let sg: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jg: [i32; 4] = [-1_i32, <i32>::default(), <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ig.as_ptr();
        let ___last: *const i32 = ig.as_ptr().offset((sg) as isize);
        unique_copy_77(___first, ___last, jg.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jg.as_mut_ptr().offset((2) as isize))));
    assert!(((jg[(0) as usize]) == (0)));
    assert!(((jg[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sg).wrapping_sub(1_u32)))
    );
    let ih: [i32; 3] = [0, 1, 1];
    let sh: u32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut jh: [i32; 3] = [-1_i32, <i32>::default(), <i32>::default()];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ih.as_ptr();
        let ___last: *const i32 = ih.as_ptr().offset((sh) as isize);
        unique_copy_77(___first, ___last, jh.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (jh.as_mut_ptr().offset((2) as isize))));
    assert!(((jh[(0) as usize]) == (0)));
    assert!(((jh[(1) as usize]) == (1)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((sh).wrapping_sub(1_u32)))
    );
    let ii: [i32; 7] = [0, 1, 1, 1, 2, 2, 2];
    let si: u32 = (((::std::mem::size_of::<[i32; 7]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ji: [i32; 7] = [
        -1_i32,
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
        <i32>::default(),
    ];
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) = 0_u32;
    r = (unsafe {
        let ___first: *const i32 = ii.as_ptr();
        let ___last: *const i32 = ii.as_ptr().offset((si) as isize);
        unique_copy_77(___first, ___last, ji.as_mut_ptr(), <count_equal>::default())
    });
    assert!(((unsafe { base_12(r,) }) == (ji.as_mut_ptr().offset((3) as isize))));
    assert!(((ji[(0) as usize]) == (0)));
    assert!(((ji[(1) as usize]) == (1)));
    assert!(((ji[(2) as usize]) == (2)));
    assert!(
        ((*std::cell::LazyCell::force_mut(&mut *&raw mut count_27)) == ((si).wrapping_sub(1_u32)))
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
    (unsafe { test_30() });
    (unsafe { test_32() });
    (unsafe { test_34() });
    (unsafe { test_36() });
    (unsafe { test_38() });
    (unsafe { test_40() });
    (unsafe { test_42() });
    (unsafe { test_44() });
    (unsafe { test_46() });
    (unsafe { test_48() });
    (unsafe { test_50() });
    (unsafe { test_52() });
    (unsafe { test_54() });
    (unsafe { test_56() });
    (unsafe { test_58() });
    (unsafe { test_60() });
    (unsafe { test_62() });
    (unsafe { test_64() });
    (unsafe { test_66() });
    (unsafe { test_68() });
    (unsafe { test_70() });
    (unsafe { test_72() });
    (unsafe { test_74() });
    (unsafe { test_76() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_27);
}
