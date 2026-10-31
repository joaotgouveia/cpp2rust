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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{}
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
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr____types_type_list_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr____types_type_list_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: TestIter,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestIter::operator_call_1_int_ptr(&f) });
            0
        };
        let _arg1: i32 = {
            (unsafe { TestIter::operator_call_1_class_contiguous_iterator_int_ptr_(&f) });
            0
        };
        let _arg2: i32 = {
            (unsafe { TestIter::operator_call_1_class_random_access_iterator_int_ptr_(&f) });
            0
        };
        let _arg3: i32 = {
            (unsafe { TestIter::operator_call_1_class_bidirectional_iterator_int_ptr_(&f) });
            0
        };
        let _arg4: i32 = {
            (unsafe { TestIter::operator_call_1_class_forward_iterator_int_ptr_(&f) });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
    mut f: TestUniquePtr,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestUniquePtr::operator_call_1_class_std_unique_ptr_int__ptr(&f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestUniquePtr :: operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_ ( & f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestUniquePtr :: operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_ ( & f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestUniquePtr :: operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_ ( & f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestUniquePtr::operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(
                    &f,
                )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn swallow_1(mut _a0: i32, mut _a1: i32, mut _a2: i32, mut _a3: i32, mut _a4: i32) {}
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
pub unsafe fn operator_eq_3(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_5(i: *const forward_iterator_int_ptr_) -> *mut i32 {
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
            operator_eq_3(
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
pub unsafe fn operator_eq_6(
    x: *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_7(
    x: *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_8(
    i: *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> *mut Option<Box<i32>> {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    it_: *mut Option<Box<i32>>,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
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
    ) -> *mut forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self
            as *mut forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> *mut forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self
            as *mut forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
                other as *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_9(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_10(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_11(i: *const bidirectional_iterator_int_ptr_) -> *mut i32 {
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
            operator_eq_9(
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
pub unsafe fn operator_eq_12(
    x: *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_13(
    x: *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_14(
    i: *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> *mut Option<Box<i32>> {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    it_: *mut Option<Box<i32>>,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
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
    ) -> *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self
            as *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn operator_dec_7(
        &mut self,
    ) -> *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self
            as *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self
            as *mut bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
}
impl std::cmp::PartialEq
    for bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(self as *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_, other as *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_)
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_7();
    }
}
pub unsafe fn operator_add_15(
    mut x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_16(
    mut x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_sub_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_17(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_18(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_19(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_20(i: *const random_access_iterator_int_ptr_) -> *mut i32 {
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
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
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
            operator_eq_18(
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
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_8();
    }
}
pub unsafe fn operator_add_21(
    mut x: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    mut n: isize,
) -> random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    (unsafe {
        random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ :: operator_add_assign ( &mut x  , n , )
    });
    return random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::move_from({
        &mut x
    });
}
pub unsafe fn operator_sub_22(
    mut x: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    mut n: isize,
) -> random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    (unsafe {
        random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ :: operator_sub_assign ( &mut x  , n , )
    });
    return random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::move_from({
        &mut x
    });
}
pub unsafe fn operator_sub_23(
    mut x: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    mut y: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<Option<Box<i32>>>()) as i64
        as isize);
}
pub unsafe fn operator_eq_24(
    x: *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_25(
    x: *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_26(
    i: *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> *mut Option<Box<i32>> {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    it_: *mut Option<Box<i32>>,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
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
    ) -> *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self
            as *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn operator_dec_8(
        &mut self,
    ) -> *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self
            as *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self
            as *mut random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
}
impl std::cmp::PartialEq
    for random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_24(self as *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_, other as *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_)
        }
    }
}
impl std::cmp::Eq for random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_8();
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
pub unsafe fn operator_add_27(
    mut x: contiguous_iterator_int_ptr_,
    mut n: isize,
) -> contiguous_iterator_int_ptr_ {
    (unsafe { contiguous_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_28(
    mut x: contiguous_iterator_int_ptr_,
    mut n: isize,
) -> contiguous_iterator_int_ptr_ {
    (unsafe { contiguous_iterator_int_ptr_::operator_sub_assign(&mut x, n) });
    return contiguous_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_29(
    mut x: contiguous_iterator_int_ptr_,
    mut y: contiguous_iterator_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_30(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_31(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_32(i: *const contiguous_iterator_int_ptr_) -> *mut i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
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
    pub unsafe fn operator_arrow(&self) -> *mut i32 {
        return self.it_;
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn operator_dec_10(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_int_ptr_,
    ) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_30(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
pub unsafe fn operator_add_33(
    mut x: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    mut n: isize,
) -> contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    (unsafe {
        contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_add_assign(
            &mut x, n,
        )
    });
    return contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::move_from({
        &mut x
    });
}
pub unsafe fn operator_sub_34(
    mut x: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    mut n: isize,
) -> contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    (unsafe {
        contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_sub_assign(
            &mut x, n,
        )
    });
    return contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::move_from({
        &mut x
    });
}
pub unsafe fn operator_sub_35(
    mut x: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    mut y: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<Option<Box<i32>>>()) as i64
        as isize);
}
pub unsafe fn operator_eq_36(
    x: *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_37(
    x: *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_38(
    i: *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> *mut Option<Box<i32>> {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    it_: *mut Option<Box<i32>>,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
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
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self
            as *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self
            as *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (self.it_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    ) -> *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self
            as *mut contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_36(
                self
                    as *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
                other
                    as *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
pub unsafe fn base_39(mut i: *mut i32) -> *mut i32 {
    return i;
}
pub unsafe fn base_40(mut i: *mut Option<Box<i32>>) -> *mut Option<Box<i32>> {
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
pub unsafe fn operator_eq_41(
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
            operator_eq_41(
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
pub unsafe fn operator_add_42(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_43(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_42(_i, n)
    });
}
pub unsafe fn iter_swap_44(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_45(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_46(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_47(
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
        unsafe { operator_cmp_47(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_46(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct TestIter {}
impl TestIter {
    pub unsafe fn operator_call_1_int_ptr(&self) {
        let mut ia: [i32; 1] = [0];
        let sa: i32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut r: *mut i32 = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        let mut ib: [i32; 2] = [0, 1];
        let sb: i32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(((ib[(0) as usize]) == (0)));
        assert!(((ib[(1) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ib.as_mut_ptr())));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        let mut ic: [i32; 3] = [0, 1, 2];
        let sc: i32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(((ic[(0) as usize]) == (1)));
        assert!(((ic[(1) as usize]) == (2)));
        assert!(((ic[(2) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ic.as_mut_ptr())));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        let mut id: [i32; 4] = [0, 1, 2, 3];
        let sd: i32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(((id[(0) as usize]) == (0)));
        assert!(((id[(1) as usize]) == (1)));
        assert!(((id[(2) as usize]) == (2)));
        assert!(((id[(3) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(((id[(0) as usize]) == (1)));
        assert!(((id[(1) as usize]) == (2)));
        assert!(((id[(2) as usize]) == (3)));
        assert!(((id[(3) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(((id[(0) as usize]) == (3)));
        assert!(((id[(1) as usize]) == (0)));
        assert!(((id[(2) as usize]) == (1)));
        assert!(((id[(3) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (id.as_mut_ptr())));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        let mut ie: [i32; 5] = [0, 1, 2, 3, 4];
        let se: i32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(((ie[(0) as usize]) == (3)));
        assert!(((ie[(1) as usize]) == (4)));
        assert!(((ie[(2) as usize]) == (0)));
        assert!(((ie[(3) as usize]) == (1)));
        assert!(((ie[(4) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ie.as_mut_ptr())));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        let mut ig: [i32; 6] = [0, 1, 2, 3, 4, 5];
        let sg: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(((ig[(0) as usize]) == (1)));
        assert!(((ig[(1) as usize]) == (2)));
        assert!(((ig[(2) as usize]) == (3)));
        assert!(((ig[(3) as usize]) == (4)));
        assert!(((ig[(4) as usize]) == (5)));
        assert!(((ig[(5) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(((ig[(0) as usize]) == (4)));
        assert!(((ig[(1) as usize]) == (5)));
        assert!(((ig[(2) as usize]) == (0)));
        assert!(((ig[(3) as usize]) == (1)));
        assert!(((ig[(4) as usize]) == (2)));
        assert!(((ig[(5) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_39(r,) }) == (ig.as_mut_ptr())));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&self) {
        let mut ia: [i32; 1] = [0];
        let sa: i32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        let mut ib: [i32; 2] = [0, 1];
        let sb: i32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(((ib[(0) as usize]) == (0)));
        assert!(((ib[(1) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ib.as_mut_ptr())));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        let mut ic: [i32; 3] = [0, 1, 2];
        let sc: i32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(((ic[(0) as usize]) == (1)));
        assert!(((ic[(1) as usize]) == (2)));
        assert!(((ic[(2) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ic.as_mut_ptr())));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        let mut id: [i32; 4] = [0, 1, 2, 3];
        let sd: i32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(((id[(0) as usize]) == (0)));
        assert!(((id[(1) as usize]) == (1)));
        assert!(((id[(2) as usize]) == (2)));
        assert!(((id[(3) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(((id[(0) as usize]) == (1)));
        assert!(((id[(1) as usize]) == (2)));
        assert!(((id[(2) as usize]) == (3)));
        assert!(((id[(3) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(((id[(0) as usize]) == (3)));
        assert!(((id[(1) as usize]) == (0)));
        assert!(((id[(2) as usize]) == (1)));
        assert!(((id[(3) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (id.as_mut_ptr())));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        let mut ie: [i32; 5] = [0, 1, 2, 3, 4];
        let se: i32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(((ie[(0) as usize]) == (3)));
        assert!(((ie[(1) as usize]) == (4)));
        assert!(((ie[(2) as usize]) == (0)));
        assert!(((ie[(3) as usize]) == (1)));
        assert!(((ie[(4) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ie.as_mut_ptr())));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        let mut ig: [i32; 6] = [0, 1, 2, 3, 4, 5];
        let sg: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(((ig[(0) as usize]) == (1)));
        assert!(((ig[(1) as usize]) == (2)));
        assert!(((ig[(2) as usize]) == (3)));
        assert!(((ig[(3) as usize]) == (4)));
        assert!(((ig[(4) as usize]) == (5)));
        assert!(((ig[(5) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(((ig[(0) as usize]) == (4)));
        assert!(((ig[(1) as usize]) == (5)));
        assert!(((ig[(2) as usize]) == (0)));
        assert!(((ig[(3) as usize]) == (1)));
        assert!(((ig[(4) as usize]) == (2)));
        assert!(((ig[(5) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_32(&r,) }) == (ig.as_mut_ptr())));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&self) {
        let mut ia: [i32; 1] = [0];
        let sa: i32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut r: random_access_iterator_int_ptr_ = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        let mut ib: [i32; 2] = [0, 1];
        let sb: i32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(((ib[(0) as usize]) == (0)));
        assert!(((ib[(1) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ib.as_mut_ptr())));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        let mut ic: [i32; 3] = [0, 1, 2];
        let sc: i32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(((ic[(0) as usize]) == (1)));
        assert!(((ic[(1) as usize]) == (2)));
        assert!(((ic[(2) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ic.as_mut_ptr())));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        let mut id: [i32; 4] = [0, 1, 2, 3];
        let sd: i32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(((id[(0) as usize]) == (0)));
        assert!(((id[(1) as usize]) == (1)));
        assert!(((id[(2) as usize]) == (2)));
        assert!(((id[(3) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(((id[(0) as usize]) == (1)));
        assert!(((id[(1) as usize]) == (2)));
        assert!(((id[(2) as usize]) == (3)));
        assert!(((id[(3) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(((id[(0) as usize]) == (3)));
        assert!(((id[(1) as usize]) == (0)));
        assert!(((id[(2) as usize]) == (1)));
        assert!(((id[(3) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (id.as_mut_ptr())));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        let mut ie: [i32; 5] = [0, 1, 2, 3, 4];
        let se: i32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(((ie[(0) as usize]) == (3)));
        assert!(((ie[(1) as usize]) == (4)));
        assert!(((ie[(2) as usize]) == (0)));
        assert!(((ie[(3) as usize]) == (1)));
        assert!(((ie[(4) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ie.as_mut_ptr())));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        let mut ig: [i32; 6] = [0, 1, 2, 3, 4, 5];
        let sg: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(((ig[(0) as usize]) == (1)));
        assert!(((ig[(1) as usize]) == (2)));
        assert!(((ig[(2) as usize]) == (3)));
        assert!(((ig[(3) as usize]) == (4)));
        assert!(((ig[(4) as usize]) == (5)));
        assert!(((ig[(5) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(((ig[(0) as usize]) == (4)));
        assert!(((ig[(1) as usize]) == (5)));
        assert!(((ig[(2) as usize]) == (0)));
        assert!(((ig[(3) as usize]) == (1)));
        assert!(((ig[(4) as usize]) == (2)));
        assert!(((ig[(5) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_20(&r,) }) == (ig.as_mut_ptr())));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&self) {
        let mut ia: [i32; 1] = [0];
        let sa: i32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        let mut ib: [i32; 2] = [0, 1];
        let sb: i32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(((ib[(0) as usize]) == (0)));
        assert!(((ib[(1) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ib.as_mut_ptr())));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        let mut ic: [i32; 3] = [0, 1, 2];
        let sc: i32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(((ic[(0) as usize]) == (1)));
        assert!(((ic[(1) as usize]) == (2)));
        assert!(((ic[(2) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ic.as_mut_ptr())));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        let mut id: [i32; 4] = [0, 1, 2, 3];
        let sd: i32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(((id[(0) as usize]) == (0)));
        assert!(((id[(1) as usize]) == (1)));
        assert!(((id[(2) as usize]) == (2)));
        assert!(((id[(3) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(((id[(0) as usize]) == (1)));
        assert!(((id[(1) as usize]) == (2)));
        assert!(((id[(2) as usize]) == (3)));
        assert!(((id[(3) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(((id[(0) as usize]) == (3)));
        assert!(((id[(1) as usize]) == (0)));
        assert!(((id[(2) as usize]) == (1)));
        assert!(((id[(3) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (id.as_mut_ptr())));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        let mut ie: [i32; 5] = [0, 1, 2, 3, 4];
        let se: i32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(((ie[(0) as usize]) == (3)));
        assert!(((ie[(1) as usize]) == (4)));
        assert!(((ie[(2) as usize]) == (0)));
        assert!(((ie[(3) as usize]) == (1)));
        assert!(((ie[(4) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ie.as_mut_ptr())));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        let mut ig: [i32; 6] = [0, 1, 2, 3, 4, 5];
        let sg: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(((ig[(0) as usize]) == (1)));
        assert!(((ig[(1) as usize]) == (2)));
        assert!(((ig[(2) as usize]) == (3)));
        assert!(((ig[(3) as usize]) == (4)));
        assert!(((ig[(4) as usize]) == (5)));
        assert!(((ig[(5) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(((ig[(0) as usize]) == (4)));
        assert!(((ig[(1) as usize]) == (5)));
        assert!(((ig[(2) as usize]) == (0)));
        assert!(((ig[(3) as usize]) == (1)));
        assert!(((ig[(4) as usize]) == (2)));
        assert!(((ig[(5) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_11(&r,) }) == (ig.as_mut_ptr())));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&self) {
        let mut ia: [i32; 1] = [0];
        let sa: i32 = (((::std::mem::size_of::<[i32; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        let mut r: forward_iterator_int_ptr_ = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(((ia[(0) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ia.as_mut_ptr())));
        assert!(((ia[(0) as usize]) == (0)));
        let mut ib: [i32; 2] = [0, 1];
        let sb: i32 = (((::std::mem::size_of::<[i32; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(((ib[(0) as usize]) == (0)));
        assert!(((ib[(1) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ib.as_mut_ptr())));
        assert!(((ib[(0) as usize]) == (1)));
        assert!(((ib[(1) as usize]) == (0)));
        let mut ic: [i32; 3] = [0, 1, 2];
        let sc: i32 = (((::std::mem::size_of::<[i32; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(((ic[(0) as usize]) == (1)));
        assert!(((ic[(1) as usize]) == (2)));
        assert!(((ic[(2) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ic.as_mut_ptr())));
        assert!(((ic[(0) as usize]) == (0)));
        assert!(((ic[(1) as usize]) == (1)));
        assert!(((ic[(2) as usize]) == (2)));
        let mut id: [i32; 4] = [0, 1, 2, 3];
        let sd: i32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(((id[(0) as usize]) == (0)));
        assert!(((id[(1) as usize]) == (1)));
        assert!(((id[(2) as usize]) == (2)));
        assert!(((id[(3) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(((id[(0) as usize]) == (1)));
        assert!(((id[(1) as usize]) == (2)));
        assert!(((id[(2) as usize]) == (3)));
        assert!(((id[(3) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(((id[(0) as usize]) == (3)));
        assert!(((id[(1) as usize]) == (0)));
        assert!(((id[(2) as usize]) == (1)));
        assert!(((id[(3) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (id.as_mut_ptr())));
        assert!(((id[(0) as usize]) == (2)));
        assert!(((id[(1) as usize]) == (3)));
        assert!(((id[(2) as usize]) == (0)));
        assert!(((id[(3) as usize]) == (1)));
        let mut ie: [i32; 5] = [0, 1, 2, 3, 4];
        let se: i32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(((ie[(0) as usize]) == (3)));
        assert!(((ie[(1) as usize]) == (4)));
        assert!(((ie[(2) as usize]) == (0)));
        assert!(((ie[(3) as usize]) == (1)));
        assert!(((ie[(4) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(((ie[(0) as usize]) == (1)));
        assert!(((ie[(1) as usize]) == (2)));
        assert!(((ie[(2) as usize]) == (3)));
        assert!(((ie[(3) as usize]) == (4)));
        assert!(((ie[(4) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ie.as_mut_ptr())));
        assert!(((ie[(0) as usize]) == (0)));
        assert!(((ie[(1) as usize]) == (1)));
        assert!(((ie[(2) as usize]) == (2)));
        assert!(((ie[(3) as usize]) == (3)));
        assert!(((ie[(4) as usize]) == (4)));
        let mut ig: [i32; 6] = [0, 1, 2, 3, 4, 5];
        let sg: i32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as i32);
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(((ig[(0) as usize]) == (1)));
        assert!(((ig[(1) as usize]) == (2)));
        assert!(((ig[(2) as usize]) == (3)));
        assert!(((ig[(3) as usize]) == (4)));
        assert!(((ig[(4) as usize]) == (5)));
        assert!(((ig[(5) as usize]) == (0)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(((ig[(0) as usize]) == (0)));
        assert!(((ig[(1) as usize]) == (1)));
        assert!(((ig[(2) as usize]) == (2)));
        assert!(((ig[(3) as usize]) == (3)));
        assert!(((ig[(4) as usize]) == (4)));
        assert!(((ig[(5) as usize]) == (5)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(((ig[(0) as usize]) == (4)));
        assert!(((ig[(1) as usize]) == (5)));
        assert!(((ig[(2) as usize]) == (0)));
        assert!(((ig[(3) as usize]) == (1)));
        assert!(((ig[(4) as usize]) == (2)));
        assert!(((ig[(5) as usize]) == (3)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
        r = unimplemented!();
        assert!(((unsafe { base_5(&r,) }) == (ig.as_mut_ptr())));
        assert!(((ig[(0) as usize]) == (3)));
        assert!(((ig[(1) as usize]) == (4)));
        assert!(((ig[(2) as usize]) == (5)));
        assert!(((ig[(3) as usize]) == (0)));
        assert!(((ig[(4) as usize]) == (1)));
        assert!(((ig[(5) as usize]) == (2)));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestUniquePtr {}
impl TestUniquePtr {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int__ptr(&self) {
        let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
        let sa: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sa)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ia[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        let mut r: *mut Option<Box<i32>> = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
        let sb: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sb)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ib[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ib.as_mut_ptr())));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ic: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let sc: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sc)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ic[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ic.as_mut_ptr())));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        let mut id: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
        let sd: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sd)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                id[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (id.as_mut_ptr())));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        let mut ie: [Option<Box<i32>>; 5] = std::array::from_fn::<_, 5, _>(|_| None);
        let se: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (se)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ie[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ie.as_mut_ptr())));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        let mut ig: [Option<Box<i32>>; 6] = std::array::from_fn::<_, 6, _>(|_| None);
        let sg: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sg)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ig[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_40(r,) }) == (ig.as_mut_ptr())));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_(&self) {
        let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
        let sa: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sa)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ia[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        let mut r: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
        let sb: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sb)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ib[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ib.as_mut_ptr())));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ic: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let sc: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sc)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ic[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ic.as_mut_ptr())));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        let mut id: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
        let sd: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sd)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                id[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (id.as_mut_ptr())));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        let mut ie: [Option<Box<i32>>; 5] = std::array::from_fn::<_, 5, _>(|_| None);
        let se: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (se)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ie[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ie.as_mut_ptr())));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        let mut ig: [Option<Box<i32>>; 6] = std::array::from_fn::<_, 6, _>(|_| None);
        let sg: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sg)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ig[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_38(&r,) }) == (ig.as_mut_ptr())));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_(
        &self,
    ) {
        let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
        let sa: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sa)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ia[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        let mut r: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
        let sb: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sb)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ib[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ib.as_mut_ptr())));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ic: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let sc: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sc)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ic[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ic.as_mut_ptr())));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        let mut id: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
        let sd: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sd)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                id[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (id.as_mut_ptr())));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        let mut ie: [Option<Box<i32>>; 5] = std::array::from_fn::<_, 5, _>(|_| None);
        let se: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (se)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ie[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ie.as_mut_ptr())));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        let mut ig: [Option<Box<i32>>; 6] = std::array::from_fn::<_, 6, _>(|_| None);
        let sg: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sg)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ig[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_26(&r,) }) == (ig.as_mut_ptr())));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_(
        &self,
    ) {
        let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
        let sa: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sa)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ia[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        let mut r: bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
        let sb: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sb)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ib[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ib.as_mut_ptr())));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ic: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let sc: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sc)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ic[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ic.as_mut_ptr())));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        let mut id: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
        let sd: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sd)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                id[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (id.as_mut_ptr())));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        let mut ie: [Option<Box<i32>>; 5] = std::array::from_fn::<_, 5, _>(|_| None);
        let se: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (se)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ie[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ie.as_mut_ptr())));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        let mut ig: [Option<Box<i32>>; 6] = std::array::from_fn::<_, 6, _>(|_| None);
        let sg: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sg)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ig[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_14(&r,) }) == (ig.as_mut_ptr())));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
    }
    pub unsafe fn operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(&self) {
        let mut ia: [Option<Box<i32>>; 1] = std::array::from_fn::<_, 1, _>(|_| None);
        let sa: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 1]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sa)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ia[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        let mut r: forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ia.as_mut_ptr().offset((sa) as isize))));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ia.as_mut_ptr())));
        assert!(
            ((*ia[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ib: [Option<Box<i32>>; 2] = std::array::from_fn::<_, 2, _>(|_| None);
        let sb: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 2]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sb)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ib[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ib.as_mut_ptr().offset((sb) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ib.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ib.as_mut_ptr())));
        assert!(
            ((*ib[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ib[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        let mut ic: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let sc: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 3]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sc)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ic[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ic.as_mut_ptr().offset((sc) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ic.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ic.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ic.as_mut_ptr())));
        assert!(
            ((*ic[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ic[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ic[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        let mut id: [Option<Box<i32>>; 4] = std::array::from_fn::<_, 4, _>(|_| None);
        let sd: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sd)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                id[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (id.as_mut_ptr().offset((sd) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (id.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (id.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (id.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (id.as_mut_ptr())));
        assert!(
            ((*id[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*id[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*id[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*id[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        let mut ie: [Option<Box<i32>>; 5] = std::array::from_fn::<_, 5, _>(|_| None);
        let se: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (se)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ie[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ie.as_mut_ptr().offset((se) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ie.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ie.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ie.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ie.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ie.as_mut_ptr())));
        assert!(
            ((*ie[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ie[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ie[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ie[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ie[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        let mut ig: [Option<Box<i32>>; 6] = std::array::from_fn::<_, 6, _>(|_| None);
        let sg: i32 = (((::std::mem::size_of::<[Option<Box<i32>>; 6]>() as usize)
            .wrapping_div((::std::mem::size_of::<Option<Box<i32>>>() as usize)))
            as i32);
        let mut i: i32 = 0;
        'loop_: while ((i) < (sg)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(i)) as *mut i32);
                ig[(i) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            i.prefix_inc();
        }
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr().offset((sg) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr().offset((5) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr().offset((4) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr().offset((3) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr().offset((2) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr().offset((1) as isize))));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        r = unimplemented!();
        assert!(((unsafe { base_8(&r,) }) == (ig.as_mut_ptr())));
        assert!(
            ((*ig[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(
            ((*ig[(1) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(
            ((*ig[(2) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (5))
        );
        assert!(
            ((*ig[(3) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
        assert!(
            ((*ig[(4) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        assert!(
            ((*ig[(5) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
    }
}
pub unsafe fn test_vector_bool_48() -> bool {
    let mut offset: i32 = -4_i32;
    'loop_: while ((offset) <= (4)) {
        let mut a: Vec<bool> = unimplemented!();
        let mut mid: usize =
            (((8_u64 as u64).wrapping_div(2_u64) as u64).wrapping_add((offset as u64)) as usize);
        let mut i: usize = mid;
        'loop_: while ((i) < (8_u64 as usize)) {
            {
                *(&mut a).as_mut_ptr().add(i as usize) = true;
                (&mut a).as_mut_ptr().add(i as usize)
            };
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (8_u64 as usize)) {
            assert!(
                ((*(&mut a).as_mut_ptr().add(i as usize) as i32)
                    == (((i) < ((8_u64 as u64).wrapping_sub((mid as u64)) as usize)) as i32))
            );
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_49() -> bool {
    let mut offset: i32 = -4_i32;
    'loop_: while ((offset) <= (4)) {
        let mut a: Vec<bool> = unimplemented!();
        let mut mid: usize =
            (((19_u64 as u64).wrapping_div(2_u64) as u64).wrapping_add((offset as u64)) as usize);
        let mut i: usize = mid;
        'loop_: while ((i) < (19_u64 as usize)) {
            {
                *(&mut a).as_mut_ptr().add(i as usize) = true;
                (&mut a).as_mut_ptr().add(i as usize)
            };
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (19_u64 as usize)) {
            assert!(
                ((*(&mut a).as_mut_ptr().add(i as usize) as i32)
                    == (((i) < ((19_u64 as u64).wrapping_sub((mid as u64)) as usize)) as i32))
            );
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_50() -> bool {
    let mut offset: i32 = -4_i32;
    'loop_: while ((offset) <= (4)) {
        let mut a: Vec<bool> = unimplemented!();
        let mut mid: usize =
            (((32_u64 as u64).wrapping_div(2_u64) as u64).wrapping_add((offset as u64)) as usize);
        let mut i: usize = mid;
        'loop_: while ((i) < (32_u64 as usize)) {
            {
                *(&mut a).as_mut_ptr().add(i as usize) = true;
                (&mut a).as_mut_ptr().add(i as usize)
            };
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (32_u64 as usize)) {
            assert!(
                ((*(&mut a).as_mut_ptr().add(i as usize) as i32)
                    == (((i) < ((32_u64 as u64).wrapping_sub((mid as u64)) as usize)) as i32))
            );
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_51() -> bool {
    let mut offset: i32 = -4_i32;
    'loop_: while ((offset) <= (4)) {
        let mut a: Vec<bool> = unimplemented!();
        let mut mid: usize =
            (((49_u64 as u64).wrapping_div(2_u64) as u64).wrapping_add((offset as u64)) as usize);
        let mut i: usize = mid;
        'loop_: while ((i) < (49_u64 as usize)) {
            {
                *(&mut a).as_mut_ptr().add(i as usize) = true;
                (&mut a).as_mut_ptr().add(i as usize)
            };
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (49_u64 as usize)) {
            assert!(
                ((*(&mut a).as_mut_ptr().add(i as usize) as i32)
                    == (((i) < ((49_u64 as u64).wrapping_sub((mid as u64)) as usize)) as i32))
            );
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_52() -> bool {
    let mut offset: i32 = -4_i32;
    'loop_: while ((offset) <= (4)) {
        let mut a: Vec<bool> = unimplemented!();
        let mut mid: usize =
            (((64_u64 as u64).wrapping_div(2_u64) as u64).wrapping_add((offset as u64)) as usize);
        let mut i: usize = mid;
        'loop_: while ((i) < (64_u64 as usize)) {
            {
                *(&mut a).as_mut_ptr().add(i as usize) = true;
                (&mut a).as_mut_ptr().add(i as usize)
            };
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (64_u64 as usize)) {
            assert!(
                ((*(&mut a).as_mut_ptr().add(i as usize) as i32)
                    == (((i) < ((64_u64 as u64).wrapping_sub((mid as u64)) as usize)) as i32))
            );
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_53() -> bool {
    let mut offset: i32 = -4_i32;
    'loop_: while ((offset) <= (4)) {
        let mut a: Vec<bool> = unimplemented!();
        let mut mid: usize =
            (((199_u64 as u64).wrapping_div(2_u64) as u64).wrapping_add((offset as u64)) as usize);
        let mut i: usize = mid;
        'loop_: while ((i) < (199_u64 as usize)) {
            {
                *(&mut a).as_mut_ptr().add(i as usize) = true;
                (&mut a).as_mut_ptr().add(i as usize)
            };
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (199_u64 as usize)) {
            assert!(
                ((*(&mut a).as_mut_ptr().add(i as usize) as i32)
                    == (((i) < ((199_u64 as u64).wrapping_sub((mid as u64)) as usize)) as i32))
            );
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_vector_bool_54() -> bool {
    let mut offset: i32 = -4_i32;
    'loop_: while ((offset) <= (4)) {
        let mut a: Vec<bool> = unimplemented!();
        let mut mid: usize =
            (((256_u64 as u64).wrapping_div(2_u64) as u64).wrapping_add((offset as u64)) as usize);
        let mut i: usize = mid;
        'loop_: while ((i) < (256_u64 as usize)) {
            {
                *(&mut a).as_mut_ptr().add(i as usize) = true;
                (&mut a).as_mut_ptr().add(i as usize)
            };
            i.prefix_inc();
        }
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (256_u64 as usize)) {
            assert!(
                ((*(&mut a).as_mut_ptr().add(i as usize) as i32)
                    == (((i) < ((256_u64 as u64).wrapping_sub((mid as u64)) as usize)) as i32))
            );
            i.prefix_inc();
        }
        offset.prefix_inc();
    }
    return true;
}
pub unsafe fn test_55() -> bool {
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <TestIter >::default() , )
    });
    {
        (unsafe {
            for_each_2 ( <types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , <TestUniquePtr >::default() , )
        });
    }
    (unsafe { test_vector_bool_48() });
    (unsafe { test_vector_bool_49() });
    (unsafe { test_vector_bool_50() });
    (unsafe { test_vector_bool_51() });
    (unsafe { test_vector_bool_52() });
    (unsafe { test_vector_bool_53() });
    (unsafe { test_vector_bool_54() });
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
    (unsafe { test_55() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
