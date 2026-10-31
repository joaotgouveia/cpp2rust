extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_char__signed_char_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_char__signed_char_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub unsafe fn allocate(&mut self, mut n: u8) -> *mut u64 {
        if ((n as i32)
            > ((unsafe {
                sized_allocator_unsigned_long__unsigned_char__signed_char_::max_size(self)
            }) as i32))
        {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u8) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u8 {
        return (((u8::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize))) as u8);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_short__short_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_short__short_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_short__short_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub unsafe fn allocate(&mut self, mut n: u16) -> *mut u64 {
        if ((n as i32)
            > ((unsafe { sized_allocator_unsigned_long__unsigned_short__short_::max_size(self) })
                as i32))
        {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u16) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u16 {
        return (((u16::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u16);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_int__int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_int__int_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_int__int_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub unsafe fn allocate(&mut self, mut n: u32) -> *mut u64 {
        if ((n) > (unsafe { sized_allocator_unsigned_long__unsigned_int__int_::max_size(self) })) {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u32) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return (((u32::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_long__long_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_long__long_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn allocate(&mut self, mut n: u64) -> *mut u64 {
        if ((n) > (unsafe { sized_allocator_unsigned_long__unsigned_long__long_::max_size(self) }))
        {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u64) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u64 {
        return (u64::MAX as u64).wrapping_div((::std::mem::size_of::<u64>() as u64));
    }
}
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
pub struct types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr___forward_iterator_char_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__
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
pub struct types_concatenate_types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr____types_type_list_bidirectional_iterator_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr____types_type_list_forward_iterator_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr___forward_iterator_char_ptr__,
    mut f: Test_char_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { Test_char_::operator_call_1_char_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe { Test_char_::operator_call_1_class_contiguous_iterator_char_ptr_(&mut f) });
            0
        };
        let _arg2: i32 = {
            (unsafe { Test_char_::operator_call_1_class_random_access_iterator_char_ptr_(&mut f) });
            0
        };
        let _arg3: i32 = {
            (unsafe { Test_char_::operator_call_1_class_bidirectional_iterator_char_ptr_(&mut f) });
            0
        };
        let _arg4: i32 = {
            (unsafe { Test_char_::operator_call_1_class_forward_iterator_char_ptr_(&mut f) });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: Test_int_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { Test_int_::operator_call_1_int_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe { Test_int_::operator_call_1_class_contiguous_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg2: i32 = {
            (unsafe { Test_int_::operator_call_1_class_random_access_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg3: i32 = {
            (unsafe { Test_int_::operator_call_1_class_bidirectional_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg4: i32 = {
            (unsafe { Test_int_::operator_call_1_class_forward_iterator_int_ptr_(&mut f) });
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
pub unsafe fn operator_ne_3(
    x: *const forward_iterator_char_ptr_,
    y: *const forward_iterator_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_char_ptr_ {
    it_: *mut libc::c_char,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut libc::c_char {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_char_ptr_,
    ) -> *mut forward_iterator_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_char_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_3(
                self as *const forward_iterator_char_ptr_,
                other as *const forward_iterator_char_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for forward_iterator_char_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<libc::c_char> for forward_iterator_char_ptr_ {
    fn iter_deref(&self) -> *mut libc::c_char {
        unsafe { forward_iterator_char_ptr_::operator_deref(self) as *mut libc::c_char }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            forward_iterator_char_ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<libc::c_char> for forward_iterator_char_ptr_ {}
pub unsafe fn operator_ne_4(
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
            !(operator_ne_4(
                self as *const forward_iterator_int_ptr_,
                other as *const forward_iterator_int_ptr_,
            ))
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
pub unsafe fn operator_ne_5(
    x: *const bidirectional_iterator_char_ptr_,
    y: *const bidirectional_iterator_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_char_ptr_ {
    it_: *mut libc::c_char,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut libc::c_char {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_char_ptr_,
    ) -> *mut bidirectional_iterator_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_char_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_5(
                self as *const bidirectional_iterator_char_ptr_,
                other as *const bidirectional_iterator_char_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_char_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<libc::c_char> for bidirectional_iterator_char_ptr_ {
    fn iter_deref(&self) -> *mut libc::c_char {
        unsafe { bidirectional_iterator_char_ptr_::operator_deref(self) as *mut libc::c_char }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            bidirectional_iterator_char_ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<libc::c_char> for bidirectional_iterator_char_ptr_ {}
pub unsafe fn operator_ne_6(
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
            !(operator_ne_6(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            ))
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
pub unsafe fn operator_ne_7(
    x: *const random_access_iterator_char_ptr_,
    y: *const random_access_iterator_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_char_ptr_ {
    it_: *mut libc::c_char,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut libc::c_char {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_char_ptr_,
    ) -> *mut random_access_iterator_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_char_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_7(
                self as *const random_access_iterator_char_ptr_,
                other as *const random_access_iterator_char_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for random_access_iterator_char_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
impl UnsafeCppIterator<libc::c_char> for random_access_iterator_char_ptr_ {
    fn iter_deref(&self) -> *mut libc::c_char {
        unsafe { random_access_iterator_char_ptr_::operator_deref(self) as *mut libc::c_char }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            random_access_iterator_char_ptr_::operator_inc_7(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<libc::c_char> for random_access_iterator_char_ptr_ {}
pub unsafe fn operator_ne_8(
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
            !(operator_ne_8(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            ))
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
pub unsafe fn operator_ne_9(
    x: *const contiguous_iterator_char_ptr_,
    y: *const contiguous_iterator_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_char_ptr_ {
    it_: *mut libc::c_char,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut libc::c_char {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_char_ptr_,
    ) -> *mut contiguous_iterator_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_char_ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_9(
                self as *const contiguous_iterator_char_ptr_,
                other as *const contiguous_iterator_char_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_char_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
impl UnsafeCppIterator<libc::c_char> for contiguous_iterator_char_ptr_ {
    fn iter_deref(&self) -> *mut libc::c_char {
        unsafe { contiguous_iterator_char_ptr_::operator_deref(self) as *mut libc::c_char }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            contiguous_iterator_char_ptr_::operator_inc_9(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<libc::c_char> for contiguous_iterator_char_ptr_ {}
pub unsafe fn operator_ne_10(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
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
            !(operator_ne_10(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
impl UnsafeCppIterator<i32> for contiguous_iterator_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { contiguous_iterator_int_ptr_::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            contiguous_iterator_int_ptr_::operator_inc_9(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for contiguous_iterator_int_ptr_ {}
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
pub unsafe fn operator_eq_11(
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
            operator_eq_11(
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
pub unsafe fn operator_add_12(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_13(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_12(_i, n)
    });
}
pub unsafe fn iter_swap_14(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_15(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_16(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_17(
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
        unsafe { operator_cmp_17(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_16(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_18(
    mut in_: Vec<libc::c_char>,
    mut from: usize,
    mut to: usize,
    mut value: libc::c_char,
    mut expected: Vec<libc::c_char>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_19(
    mut in_: Vec<libc::c_char>,
    mut from: usize,
    mut to: usize,
    mut value: libc::c_char,
    mut expected: Vec<libc::c_char>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_20(
    mut in_: Vec<libc::c_char>,
    mut from: usize,
    mut to: usize,
    mut value: libc::c_char,
    mut expected: Vec<libc::c_char>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_21(
    mut in_: Vec<libc::c_char>,
    mut from: usize,
    mut to: usize,
    mut value: libc::c_char,
    mut expected: Vec<libc::c_char>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_22(
    mut in_: Vec<libc::c_char>,
    mut from: usize,
    mut to: usize,
    mut value: libc::c_char,
    mut expected: Vec<libc::c_char>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_23(
    mut in_: Vec<i32>,
    mut from: usize,
    mut to: usize,
    mut value: i32,
    mut expected: Vec<i32>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_24(
    mut in_: Vec<i32>,
    mut from: usize,
    mut to: usize,
    mut value: i32,
    mut expected: Vec<i32>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_25(
    mut in_: Vec<i32>,
    mut from: usize,
    mut to: usize,
    mut value: i32,
    mut expected: Vec<i32>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_26(
    mut in_: Vec<i32>,
    mut from: usize,
    mut to: usize,
    mut value: i32,
    mut expected: Vec<i32>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_27(
    mut in_: Vec<i32>,
    mut from: usize,
    mut to: usize,
    mut value: i32,
    mut expected: Vec<i32>,
) {
    unimplemented!();
    assert!(unimplemented!());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_char_ {}
impl Test_char_ {
    pub unsafe fn operator_call_1_char_ptr(&mut self) {
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
            ]);
            (unsafe {
                test_18(
                    in_.clone(),
                    0_usize,
                    4_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (4 as libc::c_char),
            ]);
            (unsafe {
                test_18(
                    in_.clone(),
                    1_usize,
                    3_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_char_ptr_(&mut self) {
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
            ]);
            (unsafe {
                test_19(
                    in_.clone(),
                    0_usize,
                    4_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (4 as libc::c_char),
            ]);
            (unsafe {
                test_19(
                    in_.clone(),
                    1_usize,
                    3_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_char_ptr_(&mut self) {
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
            ]);
            (unsafe {
                test_20(
                    in_.clone(),
                    0_usize,
                    4_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (4 as libc::c_char),
            ]);
            (unsafe {
                test_20(
                    in_.clone(),
                    1_usize,
                    3_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_char_ptr_(&mut self) {
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
            ]);
            (unsafe {
                test_21(
                    in_.clone(),
                    0_usize,
                    4_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (4 as libc::c_char),
            ]);
            (unsafe {
                test_21(
                    in_.clone(),
                    1_usize,
                    3_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
    }
    pub unsafe fn operator_call_1_class_forward_iterator_char_ptr_(&mut self) {
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
            ]);
            (unsafe {
                test_22(
                    in_.clone(),
                    0_usize,
                    4_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
        {
            let mut in_: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (2 as libc::c_char),
                (3 as libc::c_char),
                (4 as libc::c_char),
            ]);
            let mut expected: Vec<libc::c_char> = Vec::from([
                (1 as libc::c_char),
                (5 as libc::c_char),
                (5 as libc::c_char),
                (4 as libc::c_char),
            ]);
            (unsafe {
                test_22(
                    in_.clone(),
                    1_usize,
                    3_usize,
                    (5 as libc::c_char),
                    expected.clone(),
                )
            });
        }
    }
}
impl Callable0<()> for Test_char_ {
    fn call(&self) -> () {
        unsafe { Test_char_::operator_call_1_char_ptr(&mut *(&raw const *self).cast_mut()) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_int_ {}
impl Test_int_ {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([5, 5, 5, 5]);
            (unsafe { test_23(in_.clone(), 0_usize, 4_usize, 5, expected.clone()) });
        }
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([1, 5, 5, 4]);
            (unsafe { test_23(in_.clone(), 1_usize, 3_usize, 5, expected.clone()) });
        }
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([5, 5, 5, 5]);
            (unsafe { test_24(in_.clone(), 0_usize, 4_usize, 5, expected.clone()) });
        }
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([1, 5, 5, 4]);
            (unsafe { test_24(in_.clone(), 1_usize, 3_usize, 5, expected.clone()) });
        }
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([5, 5, 5, 5]);
            (unsafe { test_25(in_.clone(), 0_usize, 4_usize, 5, expected.clone()) });
        }
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([1, 5, 5, 4]);
            (unsafe { test_25(in_.clone(), 1_usize, 3_usize, 5, expected.clone()) });
        }
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([5, 5, 5, 5]);
            (unsafe { test_26(in_.clone(), 0_usize, 4_usize, 5, expected.clone()) });
        }
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([1, 5, 5, 4]);
            (unsafe { test_26(in_.clone(), 1_usize, 3_usize, 5, expected.clone()) });
        }
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([5, 5, 5, 5]);
            (unsafe { test_27(in_.clone(), 0_usize, 4_usize, 5, expected.clone()) });
        }
        {
            let mut in_: Vec<i32> = Vec::from([1, 2, 3, 4]);
            let mut expected: Vec<i32> = Vec::from([1, 5, 5, 4]);
            (unsafe { test_27(in_.clone(), 1_usize, 3_usize, 5, expected.clone()) });
        }
    }
}
impl Callable0<()> for Test_int_ {
    fn call(&self) -> () {
        unsafe { Test_int_::operator_call_1_int_ptr(&mut *(&raw const *self).cast_mut()) }
    }
}
pub unsafe fn test_vector_bool_28(mut N: usize) -> bool {
    {
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            {
                *(&mut expected).as_mut_ptr().add(0_usize as usize) = *{
                    *(&mut expected).as_mut_ptr().add(1_usize as usize) = false;
                    (&mut expected).as_mut_ptr().add(1_usize as usize)
                };
                (&mut expected).as_mut_ptr().add(0_usize as usize)
            };
            unimplemented!();
            assert!(unimplemented!());
        }
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            {
                *(&mut expected)
                    .as_mut_ptr()
                    .add((N).wrapping_sub(1_usize) as usize) = *{
                    *(&mut expected)
                        .as_mut_ptr()
                        .add((N).wrapping_sub(2_usize) as usize) = false;
                    (&mut expected)
                        .as_mut_ptr()
                        .add((N).wrapping_sub(2_usize) as usize)
                };
                (&mut expected)
                    .as_mut_ptr()
                    .add((N).wrapping_sub(1_usize) as usize)
            };
            unimplemented!();
            assert!(unimplemented!());
        }
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            {
                *(&mut expected).as_mut_ptr().add(0_usize as usize) = *{
                    *(&mut expected).as_mut_ptr().add(1_usize as usize) = *{
                        *(&mut expected)
                            .as_mut_ptr()
                            .add((N).wrapping_sub(1_usize) as usize) = *{
                            *(&mut expected)
                                .as_mut_ptr()
                                .add((N).wrapping_sub(2_usize) as usize) = false;
                            (&mut expected)
                                .as_mut_ptr()
                                .add((N).wrapping_sub(2_usize) as usize)
                        };
                        (&mut expected)
                            .as_mut_ptr()
                            .add((N).wrapping_sub(1_usize) as usize)
                    };
                    (&mut expected).as_mut_ptr().add(1_usize as usize)
                };
                (&mut expected).as_mut_ptr().add(0_usize as usize)
            };
            unimplemented!();
            assert!(unimplemented!());
        }
    }
    {
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            unimplemented!();
            assert!(unimplemented!());
        }
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            assert!(unimplemented!());
        }
    }
    return true;
}
pub unsafe fn test_deque_29() {
    let mut in_: Vec<i32> = unimplemented!();
    let mut expected: Vec<i32> = unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_30() -> bool {
    (unsafe {
        for_each_0 ( <types_type_list_char_ptr__contiguous_iterator_char_ptr___random_access_iterator_char_ptr___bidirectional_iterator_char_ptr___forward_iterator_char_ptr__ >::default() , <Test_char_ >::default() , )
    });
    (unsafe {
        for_each_2 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <Test_int_ >::default() , )
    });
    {
        assert!((unsafe { test_vector_bool_28(8_usize,) }));
        assert!((unsafe { test_vector_bool_28(19_usize,) }));
        assert!((unsafe { test_vector_bool_28(32_usize,) }));
        assert!((unsafe { test_vector_bool_28(49_usize,) }));
        assert!((unsafe { test_vector_bool_28(64_usize,) }));
        assert!((unsafe { test_vector_bool_28(199_usize,) }));
        assert!((unsafe { test_vector_bool_28(256_usize,) }));
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            unimplemented!();
            assert!(unimplemented!());
        }
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            unimplemented!();
            assert!(unimplemented!());
        }
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            unimplemented!();
            assert!(unimplemented!());
        }
        {
            let mut in_: Vec<bool> = unimplemented!();
            let mut expected: Vec<bool> = unimplemented!();
            unimplemented!();
            assert!(unimplemented!());
        }
    }
    {
        (unsafe { test_deque_29() });
    }
    {
        let mut v: Vec<Vec<i32>> = Vec::from([
            Vec::from([1, 2]),
            Vec::from([1, 2, 3]),
            Vec::new(),
            Vec::from([3, 4, 5]),
            Vec::from([6]),
            Vec::from([7, 8, 9, 6]),
            Vec::from([0, 1, 2, 3, 0, 1, 2]),
        ]);
        let mut jv: () = unimplemented!();
        unimplemented!();
        'loop_: for vec_ in IterRange::new(v.begin(), v.end()) {
            let vec_: *const Vec<i32> = &(*vec_.iter_deref());
            'loop_: for n in IterRange::new((*vec_).begin(), (*vec_).end()) {
                let mut n: i32 = (*n.iter_deref());
                assert!(((n) == (42)));
            }
        }
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
    (unsafe { test_30() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
