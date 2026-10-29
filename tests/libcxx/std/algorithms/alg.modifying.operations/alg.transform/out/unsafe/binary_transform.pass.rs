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
pub unsafe fn test_constexpr_17() -> bool {
    let ia: [i32; 4] = [1, 3, 6, 7];
    let ib: [i32; 4] = [2, 4, 7, 8];
    let mut ic: [i32; 5] = [0, 0, 0, 0, 0];
    let expected: [i32; 4] = [3, 7, 13, 15];
    let mut it: *mut i32 = (unsafe {
        let ___first1: *const i32 = (unsafe { begin_18(&ia) });
        let ___last1: *const i32 = ia.as_ptr().add(ia.len());
        let ___first2: *const i32 = (unsafe { begin_18(&ib) });
        transform_19(
            ___first1,
            ___last1,
            ___first2,
            (unsafe { begin_20(&mut ic) }),
            unsafe { std::mem::zeroed::<std_plus_int_>() },
        )
    });
    return (((it) == ((unsafe { begin_20(&mut ic) }).offset((unsafe { size_21(&ia) }) as isize)))
        && ((*it) == (0)))
        && (unsafe {
            let ___first1: *const i32 = (unsafe { begin_18(&expected) });
            let ___last1: *const i32 = expected.as_ptr().add(expected.len());
            equal_22(___first1, ___last1, (unsafe { begin_20(&mut ic) }), it)
        });
}
pub unsafe fn test_23() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_24(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_26(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_28(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_30(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_32(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_34(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_36(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_38(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_40(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_42(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_44(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_46(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_48(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_50(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_52(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_54(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_56(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_58(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_60(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_62(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_64(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_66(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_68(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_70(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_72(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_74(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_76(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_78(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_80(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_82(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_84(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_86(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_88(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_90(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_92(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_94(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_96(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_98(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_100(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_102(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_104(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_106(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_108(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_110(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_112(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_114(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_116(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_118(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_120(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_122(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_124(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_126(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_128(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_130(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_132(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_134(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_136(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_138(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_140(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
        });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_142(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_144(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_146(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_148(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_150(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_152(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_154(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_156(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
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
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_158(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_160(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_162(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_164(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: *mut i32 = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_166(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
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
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_168(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_169() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_170(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_171() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_172(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_173() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_174(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_175() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_176(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_177() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_178(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_179() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_180(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_181() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_182(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_183() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_184(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_185() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_186(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_187() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_188(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_189() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_190(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_191() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_192(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_193() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_194(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_195() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_196(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_197() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_198(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_199() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_200(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_201() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_202(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_203() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_204(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_205() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_206(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_207() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_208(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_209() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_210(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_211() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_212(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_213() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_214(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_215() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_216(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_217() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_218(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_219() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_220(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_221() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_222(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_223() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_224(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_225() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_226(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_227() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_228(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_229() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_230(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_231() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_232(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_233() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_234(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_235() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_236(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_237() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_238(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_239() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_240(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_241() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_242(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_243() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_244(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_245() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_246(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_247() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_248(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_249() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_250(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_251() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_252(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_253() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_254(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_255() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_256(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_257() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_258(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_259() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_260(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_261() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_262(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_263() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_264(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_265() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_266(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_267() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_268(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_269() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_270(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_271() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_272(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_273() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_274(
            ___first1,
            ___last1,
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (ia.as_mut_ptr()).cast_const()
            }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_275() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_276(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_277() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_278(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_279() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_280(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_281() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_282(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_283() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_284(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_285() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_286(
            ___first1,
            ___last1,
            forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_287() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_288(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_289() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_290(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_291() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_292(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_293() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_294(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_295() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_296(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_297() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_298(
            ___first1,
            ___last1,
            bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_299() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_300(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_301() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_302(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_303() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_304(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_305() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_306(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_307() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_308(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_309() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_310(
            ___first1,
            ___last1,
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() }),
            ___result,
            unsafe { std::mem::zeroed::<std_minus_int_>() },
        )
    });
    assert!(((unsafe { base_9(r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_311() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_output_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_output_iterator_int_ptr_ =
            cpp17_output_iterator_int_ptr_::new({ ib.as_mut_ptr() });
        transform_312(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_0(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_313() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: cpp17_input_iterator_int_ptr__int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: cpp17_input_iterator_int_ptr__int_ptr_ =
            cpp17_input_iterator_int_ptr__int_ptr_::new({ ib.as_mut_ptr() });
        transform_314(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_1(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_315() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: forward_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_316(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_3(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_317() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: bidirectional_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_318(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_319() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: random_access_iterator_int_ptr_ = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ ib.as_mut_ptr() });
        transform_320(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
    assert!(((unsafe { base_7(&r,) }) == (ib.as_mut_ptr().offset((sa) as isize))));
    assert!(((ib[(0) as usize]) == (1)));
    assert!(((ib[(1) as usize]) == (1)));
    assert!(((ib[(2) as usize]) == (1)));
    assert!(((ib[(3) as usize]) == (1)));
    assert!(((ib[(4) as usize]) == (1)));
}
pub unsafe fn test_321() {
    let mut ia: [i32; 5] = [0, 1, 2, 3, 4];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ib: [i32; 5] = [1, 2, 3, 4, 5];
    let mut r: *mut i32 = (unsafe {
        let ___first1: *const i32 = ib.as_mut_ptr();
        let ___last1: *const i32 = ib.as_mut_ptr().offset((sa) as isize);
        let ___result: *mut i32 = ib.as_mut_ptr();
        transform_322(___first1, ___last1, ia.as_mut_ptr(), ___result, unsafe {
            std::mem::zeroed::<std_minus_int_>()
        })
    });
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
    (unsafe { test_23() });
    (unsafe { test_25() });
    (unsafe { test_27() });
    (unsafe { test_29() });
    (unsafe { test_31() });
    (unsafe { test_33() });
    (unsafe { test_35() });
    (unsafe { test_37() });
    (unsafe { test_39() });
    (unsafe { test_41() });
    (unsafe { test_43() });
    (unsafe { test_45() });
    (unsafe { test_47() });
    (unsafe { test_49() });
    (unsafe { test_51() });
    (unsafe { test_53() });
    (unsafe { test_55() });
    (unsafe { test_57() });
    (unsafe { test_59() });
    (unsafe { test_61() });
    (unsafe { test_63() });
    (unsafe { test_65() });
    (unsafe { test_67() });
    (unsafe { test_69() });
    (unsafe { test_71() });
    (unsafe { test_73() });
    (unsafe { test_75() });
    (unsafe { test_77() });
    (unsafe { test_79() });
    (unsafe { test_81() });
    (unsafe { test_83() });
    (unsafe { test_85() });
    (unsafe { test_87() });
    (unsafe { test_89() });
    (unsafe { test_91() });
    (unsafe { test_93() });
    (unsafe { test_95() });
    (unsafe { test_97() });
    (unsafe { test_99() });
    (unsafe { test_101() });
    (unsafe { test_103() });
    (unsafe { test_105() });
    (unsafe { test_107() });
    (unsafe { test_109() });
    (unsafe { test_111() });
    (unsafe { test_113() });
    (unsafe { test_115() });
    (unsafe { test_117() });
    (unsafe { test_119() });
    (unsafe { test_121() });
    (unsafe { test_123() });
    (unsafe { test_125() });
    (unsafe { test_127() });
    (unsafe { test_129() });
    (unsafe { test_131() });
    (unsafe { test_133() });
    (unsafe { test_135() });
    (unsafe { test_137() });
    (unsafe { test_139() });
    (unsafe { test_141() });
    (unsafe { test_143() });
    (unsafe { test_145() });
    (unsafe { test_147() });
    (unsafe { test_149() });
    (unsafe { test_151() });
    (unsafe { test_153() });
    (unsafe { test_155() });
    (unsafe { test_157() });
    (unsafe { test_159() });
    (unsafe { test_161() });
    (unsafe { test_163() });
    (unsafe { test_165() });
    (unsafe { test_167() });
    (unsafe { test_169() });
    (unsafe { test_171() });
    (unsafe { test_173() });
    (unsafe { test_175() });
    (unsafe { test_177() });
    (unsafe { test_179() });
    (unsafe { test_181() });
    (unsafe { test_183() });
    (unsafe { test_185() });
    (unsafe { test_187() });
    (unsafe { test_189() });
    (unsafe { test_191() });
    (unsafe { test_193() });
    (unsafe { test_195() });
    (unsafe { test_197() });
    (unsafe { test_199() });
    (unsafe { test_201() });
    (unsafe { test_203() });
    (unsafe { test_205() });
    (unsafe { test_207() });
    (unsafe { test_209() });
    (unsafe { test_211() });
    (unsafe { test_213() });
    (unsafe { test_215() });
    (unsafe { test_217() });
    (unsafe { test_219() });
    (unsafe { test_221() });
    (unsafe { test_223() });
    (unsafe { test_225() });
    (unsafe { test_227() });
    (unsafe { test_229() });
    (unsafe { test_231() });
    (unsafe { test_233() });
    (unsafe { test_235() });
    (unsafe { test_237() });
    (unsafe { test_239() });
    (unsafe { test_241() });
    (unsafe { test_243() });
    (unsafe { test_245() });
    (unsafe { test_247() });
    (unsafe { test_249() });
    (unsafe { test_251() });
    (unsafe { test_253() });
    (unsafe { test_255() });
    (unsafe { test_257() });
    (unsafe { test_259() });
    (unsafe { test_261() });
    (unsafe { test_263() });
    (unsafe { test_265() });
    (unsafe { test_267() });
    (unsafe { test_269() });
    (unsafe { test_271() });
    (unsafe { test_273() });
    (unsafe { test_275() });
    (unsafe { test_277() });
    (unsafe { test_279() });
    (unsafe { test_281() });
    (unsafe { test_283() });
    (unsafe { test_285() });
    (unsafe { test_287() });
    (unsafe { test_289() });
    (unsafe { test_291() });
    (unsafe { test_293() });
    (unsafe { test_295() });
    (unsafe { test_297() });
    (unsafe { test_299() });
    (unsafe { test_301() });
    (unsafe { test_303() });
    (unsafe { test_305() });
    (unsafe { test_307() });
    (unsafe { test_309() });
    (unsafe { test_311() });
    (unsafe { test_313() });
    (unsafe { test_315() });
    (unsafe { test_317() });
    (unsafe { test_319() });
    (unsafe { test_321() });
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
