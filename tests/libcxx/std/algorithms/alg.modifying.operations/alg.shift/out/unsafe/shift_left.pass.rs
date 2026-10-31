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
pub unsafe fn operator_eq_2(
    x: *const forward_iterator_MoveOnly_ptr_,
    y: *const forward_iterator_MoveOnly_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_3(
    x: *const forward_iterator_MoveOnly_ptr_,
    y: *const forward_iterator_MoveOnly_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_MoveOnly_ptr_ {
    it_: *mut MoveOnly,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_MoveOnly_ptr_ {
    pub unsafe fn new_1(mut it: *mut MoveOnly) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut MoveOnly {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_MoveOnly_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_MoveOnly_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_MoveOnly_ptr_,
    ) -> *mut forward_iterator_MoveOnly_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_MoveOnly_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const forward_iterator_MoveOnly_ptr_,
                other as *const forward_iterator_MoveOnly_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_MoveOnly_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_MoveOnly_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_4(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_5(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
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
    pub unsafe fn operator_dec_7(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_dec();
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
            operator_eq_4(
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
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_7();
    }
}
pub unsafe fn operator_eq_6(
    x: *const bidirectional_iterator_MoveOnly_ptr_,
    y: *const bidirectional_iterator_MoveOnly_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_7(
    x: *const bidirectional_iterator_MoveOnly_ptr_,
    y: *const bidirectional_iterator_MoveOnly_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_MoveOnly_ptr_ {
    it_: *mut MoveOnly,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_MoveOnly_ptr_ {
    pub unsafe fn new_1(mut it: *mut MoveOnly) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut MoveOnly {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_MoveOnly_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_dec_7(&mut self) -> *mut bidirectional_iterator_MoveOnly_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut bidirectional_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_MoveOnly_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_MoveOnly_ptr_,
    ) -> *mut bidirectional_iterator_MoveOnly_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_MoveOnly_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const bidirectional_iterator_MoveOnly_ptr_,
                other as *const bidirectional_iterator_MoveOnly_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_MoveOnly_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_MoveOnly_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_7();
    }
}
pub unsafe fn operator_sub_8(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_9(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_10(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
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
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
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
            operator_eq_9(
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
pub unsafe fn operator_sub_11(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut y: random_access_iterator_MoveOnly_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_12(
    x: *const random_access_iterator_MoveOnly_ptr_,
    y: *const random_access_iterator_MoveOnly_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_13(
    x: *const random_access_iterator_MoveOnly_ptr_,
    y: *const random_access_iterator_MoveOnly_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_MoveOnly_ptr_ {
    it_: *mut MoveOnly,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_MoveOnly_ptr_ {
    pub unsafe fn new_1(mut it: *mut MoveOnly) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut MoveOnly {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_MoveOnly_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_MoveOnly_ptr_,
    ) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const random_access_iterator_MoveOnly_ptr_,
                other as *const random_access_iterator_MoveOnly_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_MoveOnly_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_MoveOnly_ptr_ {
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
pub unsafe fn operator_eq_14(
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
            operator_eq_14(
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
pub unsafe fn operator_add_15(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_16(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_15(_i, n)
    });
}
pub unsafe fn iter_swap_17(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_18(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_19(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_20(
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
        unsafe { operator_cmp_20(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_19(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn operator_eq_21(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_22(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_23(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_24(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_25(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_26(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_27(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_27(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_21(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_28(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_29(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_30(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_31(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_32(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_33(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_34(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_34(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_28(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
pub unsafe fn test_35() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() = (*__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: forward_iterator_int_ptr_ = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    (unsafe {
                        let _x: *const forward_iterator_int_ptr_ = &it;
                        let mut _y: forward_iterator_int_ptr_ = forward_iterator_int_ptr_::new_1({
                            work.as_mut_ptr()
                                .offset((n) as isize)
                                .offset(-((k) as isize))
                        });
                        operator_eq_0(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            } else {
                assert!(
                    (unsafe {
                        let _x: *const forward_iterator_int_ptr_ = &it;
                        let mut _y: forward_iterator_int_ptr_ =
                            forward_iterator_int_ptr_::new_1({ work.as_mut_ptr() });
                        operator_eq_0(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const forward_iterator_int_ptr_ = &it;
                operator_eq_0(_x, &e)
            })
        );
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 2] = [1, 2];
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 6] = [3, 4, 5, 6, 7, 8];
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 2] = [7, 8];
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const forward_iterator_int_ptr_ = &it;
                operator_eq_0(_x, &b)
            })
        );
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_int_ptr_ =
            forward_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const forward_iterator_int_ptr_ = &it;
                operator_eq_0(_x, &b)
            })
        );
    }
    return true;
}
pub unsafe fn test_36() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() = (*__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: bidirectional_iterator_int_ptr_ = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    (unsafe {
                        let _x: *const bidirectional_iterator_int_ptr_ = &it;
                        let mut _y: bidirectional_iterator_int_ptr_ =
                            bidirectional_iterator_int_ptr_::new_1({
                                work.as_mut_ptr()
                                    .offset((n) as isize)
                                    .offset(-((k) as isize))
                            });
                        operator_eq_4(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            } else {
                assert!(
                    (unsafe {
                        let _x: *const bidirectional_iterator_int_ptr_ = &it;
                        let mut _y: bidirectional_iterator_int_ptr_ =
                            bidirectional_iterator_int_ptr_::new_1({ work.as_mut_ptr() });
                        operator_eq_4(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_int_ptr_ = &it;
                operator_eq_4(_x, &e)
            })
        );
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 2] = [1, 2];
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 6] = [3, 4, 5, 6, 7, 8];
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 2] = [7, 8];
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_int_ptr_ = &it;
                operator_eq_4(_x, &b)
            })
        );
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_int_ptr_ =
            bidirectional_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_int_ptr_ = &it;
                operator_eq_4(_x, &b)
            })
        );
    }
    return true;
}
pub unsafe fn test_37() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() = (*__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: random_access_iterator_int_ptr_ = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    (unsafe {
                        let _x: *const random_access_iterator_int_ptr_ = &it;
                        let mut _y: random_access_iterator_int_ptr_ =
                            random_access_iterator_int_ptr_::new_1({
                                work.as_mut_ptr()
                                    .offset((n) as isize)
                                    .offset(-((k) as isize))
                            });
                        operator_eq_9(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            } else {
                assert!(
                    (unsafe {
                        let _x: *const random_access_iterator_int_ptr_ = &it;
                        let mut _y: random_access_iterator_int_ptr_ =
                            random_access_iterator_int_ptr_::new_1({ work.as_mut_ptr() });
                        operator_eq_9(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_int_ptr_ = &it;
                operator_eq_9(_x, &e)
            })
        );
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 2] = [1, 2];
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 6] = [3, 4, 5, 6, 7, 8];
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 2] = [7, 8];
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_int_ptr_ = &it;
                operator_eq_9(_x, &b)
            })
        );
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_int_ptr_ =
            random_access_iterator_int_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_int_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_int_ptr_ = &it;
                operator_eq_9(_x, &b)
            })
        );
    }
    return true;
}
pub unsafe fn test_38() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() = (*__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: *mut i32 = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    ((it)
                        == (work
                            .as_mut_ptr()
                            .offset((n) as isize)
                            .offset(-((k) as isize))))
                );
                assert!(unimplemented!());
            } else {
                assert!(((it) == (work.as_mut_ptr())));
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: *mut i32 = unimplemented!();
        let mut e: *mut i32 = input.as_ptr().add(input.len());
        let mut it: *mut i32 = unimplemented!();
        assert!(unimplemented!());
        assert!(((it) == (e)));
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 2] = [1, 2];
        let mut b: *mut i32 = unimplemented!();
        let mut e: *mut i32 = input.as_ptr().add(input.len());
        let mut it: *mut i32 = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 6] = [3, 4, 5, 6, 7, 8];
        let mut b: *mut i32 = unimplemented!();
        let mut e: *mut i32 = input.as_ptr().add(input.len());
        let mut it: *mut i32 = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected: [i32; 2] = [7, 8];
        let mut b: *mut i32 = unimplemented!();
        let mut e: *mut i32 = input.as_ptr().add(input.len());
        let mut it: *mut i32 = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: *mut i32 = unimplemented!();
        let mut e: *mut i32 = input.as_ptr().add(input.len());
        let mut it: *mut i32 = unimplemented!();
        assert!(unimplemented!());
        assert!(((it) == (b)));
    }
    {
        let mut input: [i32; 3] = [0, 1, 2];
        let expected: [i32; 3] = [0, 1, 2];
        let mut b: *mut i32 = unimplemented!();
        let mut e: *mut i32 = input.as_ptr().add(input.len());
        let mut it: *mut i32 = unimplemented!();
        assert!(unimplemented!());
        assert!(((it) == (b)));
    }
    return true;
}
pub unsafe fn test_39() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [MoveOnly; 15] = [
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() =
                        <MoveOnly as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: forward_iterator_MoveOnly_ptr_ = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    (unsafe {
                        let _x: *const forward_iterator_MoveOnly_ptr_ = &it;
                        let mut _y: forward_iterator_MoveOnly_ptr_ =
                            forward_iterator_MoveOnly_ptr_::new_1({
                                work.as_mut_ptr()
                                    .offset((n) as isize)
                                    .offset(-((k) as isize))
                            });
                        operator_eq_2(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            } else {
                assert!(
                    (unsafe {
                        let _x: *const forward_iterator_MoveOnly_ptr_ = &it;
                        let mut _y: forward_iterator_MoveOnly_ptr_ =
                            forward_iterator_MoveOnly_ptr_::new_1({ work.as_mut_ptr() });
                        operator_eq_2(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const forward_iterator_MoveOnly_ptr_ = &it;
                operator_eq_2(_x, &e)
            })
        );
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(1) }), MoveOnly::new({ Some(2) })];
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 6] = [
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(7) }), MoveOnly::new({ Some(8) })];
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const forward_iterator_MoveOnly_ptr_ = &it;
                operator_eq_2(_x, &b)
            })
        );
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: forward_iterator_MoveOnly_ptr_ =
            forward_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: forward_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const forward_iterator_MoveOnly_ptr_ = &it;
                operator_eq_2(_x, &b)
            })
        );
    }
    return true;
}
pub unsafe fn test_40() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [MoveOnly; 15] = [
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() =
                        <MoveOnly as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: bidirectional_iterator_MoveOnly_ptr_ = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    (unsafe {
                        let _x: *const bidirectional_iterator_MoveOnly_ptr_ = &it;
                        let mut _y: bidirectional_iterator_MoveOnly_ptr_ =
                            bidirectional_iterator_MoveOnly_ptr_::new_1({
                                work.as_mut_ptr()
                                    .offset((n) as isize)
                                    .offset(-((k) as isize))
                            });
                        operator_eq_6(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            } else {
                assert!(
                    (unsafe {
                        let _x: *const bidirectional_iterator_MoveOnly_ptr_ = &it;
                        let mut _y: bidirectional_iterator_MoveOnly_ptr_ =
                            bidirectional_iterator_MoveOnly_ptr_::new_1({ work.as_mut_ptr() });
                        operator_eq_6(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_MoveOnly_ptr_ = &it;
                operator_eq_6(_x, &e)
            })
        );
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(1) }), MoveOnly::new({ Some(2) })];
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 6] = [
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(7) }), MoveOnly::new({ Some(8) })];
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_MoveOnly_ptr_ = &it;
                operator_eq_6(_x, &b)
            })
        );
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: bidirectional_iterator_MoveOnly_ptr_ =
            bidirectional_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: bidirectional_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_MoveOnly_ptr_ = &it;
                operator_eq_6(_x, &b)
            })
        );
    }
    return true;
}
pub unsafe fn test_41() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [MoveOnly; 15] = [
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() =
                        <MoveOnly as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: random_access_iterator_MoveOnly_ptr_ = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    (unsafe {
                        let _x: *const random_access_iterator_MoveOnly_ptr_ = &it;
                        let mut _y: random_access_iterator_MoveOnly_ptr_ =
                            random_access_iterator_MoveOnly_ptr_::new_1({
                                work.as_mut_ptr()
                                    .offset((n) as isize)
                                    .offset(-((k) as isize))
                            });
                        operator_eq_12(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            } else {
                assert!(
                    (unsafe {
                        let _x: *const random_access_iterator_MoveOnly_ptr_ = &it;
                        let mut _y: random_access_iterator_MoveOnly_ptr_ =
                            random_access_iterator_MoveOnly_ptr_::new_1({ work.as_mut_ptr() });
                        operator_eq_12(_x, &mut _y)
                    })
                );
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_MoveOnly_ptr_ = &it;
                operator_eq_12(_x, &e)
            })
        );
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(1) }), MoveOnly::new({ Some(2) })];
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 6] = [
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(7) }), MoveOnly::new({ Some(8) })];
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_MoveOnly_ptr_ = &it;
                operator_eq_12(_x, &b)
            })
        );
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ unimplemented!() });
        let mut e: random_access_iterator_MoveOnly_ptr_ =
            random_access_iterator_MoveOnly_ptr_::new_1({ input.as_ptr().add(input.len()) });
        let mut it: random_access_iterator_MoveOnly_ptr_ = unimplemented!();
        assert!(unimplemented!());
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_MoveOnly_ptr_ = &it;
                operator_eq_12(_x, &b)
            })
        );
    }
    return true;
}
pub unsafe fn test_42() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [MoveOnly; 15] = [
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ];
    let mut n: i32 = 0;
    'loop_: while ((n) <= (15)) {
        let mut k: i32 = 0;
        'loop_: while ((k) <= ((n) + (2))) {
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((n) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() =
                        <MoveOnly as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            let mut it: *mut MoveOnly = unimplemented!();
            if ((0) <= (k)) && ((k) < (n)) {
                assert!(
                    ((it)
                        == (work
                            .as_mut_ptr()
                            .offset((n) as isize)
                            .offset(-((k) as isize))))
                );
                assert!(unimplemented!());
            } else {
                assert!(((it) == (work.as_mut_ptr())));
                assert!(unimplemented!());
            }
            k.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: *mut MoveOnly = unimplemented!();
        let mut e: *mut MoveOnly = input.as_ptr().add(input.len());
        let mut it: *mut MoveOnly = unimplemented!();
        assert!(unimplemented!());
        assert!(((it) == (e)));
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(1) }), MoveOnly::new({ Some(2) })];
        let mut b: *mut MoveOnly = unimplemented!();
        let mut e: *mut MoveOnly = input.as_ptr().add(input.len());
        let mut it: *mut MoveOnly = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 6] = [
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let mut b: *mut MoveOnly = unimplemented!();
        let mut e: *mut MoveOnly = input.as_ptr().add(input.len());
        let mut it: *mut MoveOnly = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 8] = [
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(6) }),
            MoveOnly::new({ Some(7) }),
            MoveOnly::new({ Some(8) }),
        ];
        let expected: [MoveOnly; 2] = [MoveOnly::new({ Some(7) }), MoveOnly::new({ Some(8) })];
        let mut b: *mut MoveOnly = unimplemented!();
        let mut e: *mut MoveOnly = input.as_ptr().add(input.len());
        let mut it: *mut MoveOnly = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: *mut MoveOnly = unimplemented!();
        let mut e: *mut MoveOnly = input.as_ptr().add(input.len());
        let mut it: *mut MoveOnly = unimplemented!();
        assert!(unimplemented!());
        assert!(((it) == (b)));
    }
    {
        let mut input: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let expected: [MoveOnly; 3] = [
            MoveOnly::new({ Some(0) }),
            MoveOnly::new({ Some(1) }),
            MoveOnly::new({ Some(2) }),
        ];
        let mut b: *mut MoveOnly = unimplemented!();
        let mut e: *mut MoveOnly = input.as_ptr().add(input.len());
        let mut it: *mut MoveOnly = unimplemented!();
        assert!(unimplemented!());
        assert!(((it) == (b)));
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
    (unsafe { test_36() });
    (unsafe { test_37() });
    (unsafe { test_38() });
    (unsafe { test_39() });
    (unsafe { test_40() });
    (unsafe { test_41() });
    (unsafe { test_42() });
    const _: () = assert!(true, "test<int, forward_iterator<int*>>()");;
    const _: () = assert!(true, "test<int, bidirectional_iterator<int*>>()");;
    const _: () = assert!(true, "test<int, random_access_iterator<int*>>()");;
    const _: () = assert!(true, "test<int, int*>()");;
    const _: () = assert!(true, "test<MoveOnly, forward_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<MoveOnly, bidirectional_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<MoveOnly, random_access_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<MoveOnly, MoveOnly*>()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
