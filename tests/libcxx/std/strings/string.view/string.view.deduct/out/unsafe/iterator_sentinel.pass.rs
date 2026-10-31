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
pub struct test_macros_detail_is_same_std_basic_string_view_char__std_char_traits_char____std_basic_string_view_char__std_char_traits_char___
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_wchar_t__std_char_traits_wchar_t____std_basic_string_view_wchar_t__std_char_traits_wchar_t___
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_char8_t__std_char_traits_char8_t____std_basic_string_view_char8_t__std_char_traits_char8_t___
{}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_char16_t__std_char_traits_char16_t____std_basic_string_view_char16_t__std_char_traits_char16_t___
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_char32_t__std_char_traits_char32_t____std_basic_string_view_char32_t__std_char_traits_char32_t___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(*const libc::c_char , to_pconstlibcc_char)]
#[cpp_into(*const i32 , to_pconsti32)]
#[cpp_into(*const u8 , to_pconstu8)]
#[cpp_into(*const u16 , to_pconstu16)]
#[cpp_into(*const u32 , to_pconstu32)]
pub struct MultiStringType {
    pub w_: *const i32,
    pub wn_: usize,
    pub u8_: *const u8,
    pub u8n_: usize,
    pub u16_: *const u16,
    pub u16n_: usize,
    pub u32_: *const u32,
    pub u32n_: usize,
    pub s_: *const libc::c_char,
    pub sn_: usize,
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const i32,
        mut wn: usize,
        mut u8: *const u8,
        mut u8n: usize,
        mut u16: *const u16,
        mut u16n: usize,
        mut u32: *const u32,
        mut u32n: usize,
        mut s: *const libc::c_char,
        mut sn: usize,
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const libc::c_char) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const i32) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const u8) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const u16) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const u32) -> *const u32 {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const libc::c_char) -> usize {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const i32) -> usize {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const u8) -> usize {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const u16) -> usize {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const u32) -> usize {
        return self.u32n_;
    }
    pub unsafe fn to_pconstlibcc_char(&self) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn to_pconsti32(&self) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn to_pconstu8(&self) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn to_pconstu16(&self) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn to_pconstu32(&self) -> *const u32 {
        return self.u32_;
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
pub unsafe fn operator_sub_5(
    mut x: contiguous_iterator_const_char_ptr_,
    mut y: contiguous_iterator_const_char_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<libc::c_char>()) as i64
        as isize);
}
pub unsafe fn base_6(i: *const contiguous_iterator_const_char_ptr_) -> *const libc::c_char {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const libc::c_char {
        return self.it_;
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_char_ptr_,
    ) -> *mut contiguous_iterator_const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_char_ptr_));
    }
}
pub unsafe fn operator_sub_7(
    mut x: contiguous_iterator_const_wchar_t_ptr_,
    mut y: contiguous_iterator_const_wchar_t_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn base_8(i: *const contiguous_iterator_const_wchar_t_ptr_) -> *const i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_wchar_t_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_wchar_t_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const i32 {
        return self.it_;
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_wchar_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_wchar_t_ptr_,
    ) -> *mut contiguous_iterator_const_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_wchar_t_ptr_));
    }
}
pub unsafe fn operator_sub_9(
    mut x: contiguous_iterator_const_char8_t_ptr_,
    mut y: contiguous_iterator_const_char8_t_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<u8>()) as i64 as isize);
}
pub unsafe fn base_10(i: *const contiguous_iterator_const_char8_t_ptr_) -> *const u8 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_char8_t_ptr_ {
    it_: *const u8,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char8_t_ptr_ {
    pub unsafe fn new_1(mut it: *const u8) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const u8 {
        return self.it_;
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_char8_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_char8_t_ptr_,
    ) -> *mut contiguous_iterator_const_char8_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_char8_t_ptr_));
    }
}
pub unsafe fn operator_sub_11(
    mut x: contiguous_iterator_const_char16_t_ptr_,
    mut y: contiguous_iterator_const_char16_t_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<u16>()) as i64 as isize);
}
pub unsafe fn base_12(i: *const contiguous_iterator_const_char16_t_ptr_) -> *const u16 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_char16_t_ptr_ {
    it_: *const u16,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char16_t_ptr_ {
    pub unsafe fn new_1(mut it: *const u16) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const u16 {
        return self.it_;
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_char16_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_char16_t_ptr_,
    ) -> *mut contiguous_iterator_const_char16_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_char16_t_ptr_));
    }
}
pub unsafe fn operator_sub_13(
    mut x: contiguous_iterator_const_char32_t_ptr_,
    mut y: contiguous_iterator_const_char32_t_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<u32>()) as i64 as isize);
}
pub unsafe fn base_14(i: *const contiguous_iterator_const_char32_t_ptr_) -> *const u32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_char32_t_ptr_ {
    it_: *const u32,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_char32_t_ptr_ {
    pub unsafe fn new_1(mut it: *const u32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const u32 {
        return self.it_;
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_char32_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_char32_t_ptr_,
    ) -> *mut contiguous_iterator_const_char32_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_char32_t_ptr_));
    }
}
pub unsafe fn base_15(mut i: *const libc::c_char) -> *const libc::c_char {
    return i;
}
pub unsafe fn base_16(mut i: *const i32) -> *const i32 {
    return i;
}
pub unsafe fn base_17(mut i: *const u8) -> *const u8 {
    return i;
}
pub unsafe fn base_18(mut i: *const u16) -> *const u16 {
    return i;
}
pub unsafe fn base_19(mut i: *const u32) -> *const u32 {
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
pub unsafe fn operator_sub_20(
    s: *const sized_sentinel_const_char_ptr_,
    i: *const *const libc::c_char,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_15((*i)) }) as usize)
        / ::std::mem::size_of::<libc::c_char>()) as i64;
}
pub unsafe fn operator_sub_21(
    i: *const *const libc::c_char,
    s: *const sized_sentinel_const_char_ptr_,
) -> i64 {
    return (((unsafe { base_15((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<libc::c_char>()) as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char_ptr_ {
    base_: *const libc::c_char,
}
impl sized_sentinel_const_char_ptr_ {
    pub unsafe fn new_1(it: *const *const libc::c_char) -> Self {
        let mut this = Self {
            base_: (unsafe { base_15((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_22(
    s: *const sized_sentinel_contiguous_iterator_const_char_ptr__,
    i: *const contiguous_iterator_const_char_ptr_,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_6(i) }) as usize)
        / ::std::mem::size_of::<libc::c_char>()) as i64;
}
pub unsafe fn operator_sub_23(
    i: *const contiguous_iterator_const_char_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char_ptr__,
) -> i64 {
    return (((unsafe { base_6(i) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<libc::c_char>()) as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char_ptr__ {
    base_: *const libc::c_char,
}
impl sized_sentinel_contiguous_iterator_const_char_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_6(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_24(
    s: *const sized_sentinel_const_wchar_t_ptr_,
    i: *const *const i32,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_16((*i)) }) as usize)
        / ::std::mem::size_of::<i32>()) as i64;
}
pub unsafe fn operator_sub_25(
    i: *const *const i32,
    s: *const sized_sentinel_const_wchar_t_ptr_,
) -> i64 {
    return (((unsafe { base_16((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<i32>()) as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_wchar_t_ptr_ {
    base_: *const i32,
}
impl sized_sentinel_const_wchar_t_ptr_ {
    pub unsafe fn new_1(it: *const *const i32) -> Self {
        let mut this = Self {
            base_: (unsafe { base_16((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_26(
    s: *const sized_sentinel_contiguous_iterator_const_wchar_t_ptr__,
    i: *const contiguous_iterator_const_wchar_t_ptr_,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_8(i) }) as usize) / ::std::mem::size_of::<i32>())
        as i64;
}
pub unsafe fn operator_sub_27(
    i: *const contiguous_iterator_const_wchar_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_wchar_t_ptr__,
) -> i64 {
    return (((unsafe { base_8(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<i32>())
        as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    base_: *const i32,
}
impl sized_sentinel_contiguous_iterator_const_wchar_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_wchar_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_8(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_28(
    s: *const sized_sentinel_const_char8_t_ptr_,
    i: *const *const u8,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_17((*i)) }) as usize)
        / ::std::mem::size_of::<u8>()) as i64;
}
pub unsafe fn operator_sub_29(
    i: *const *const u8,
    s: *const sized_sentinel_const_char8_t_ptr_,
) -> i64 {
    return (((unsafe { base_17((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<u8>()) as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char8_t_ptr_ {
    base_: *const u8,
}
impl sized_sentinel_const_char8_t_ptr_ {
    pub unsafe fn new_1(it: *const *const u8) -> Self {
        let mut this = Self {
            base_: (unsafe { base_17((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_30(
    s: *const sized_sentinel_contiguous_iterator_const_char8_t_ptr__,
    i: *const contiguous_iterator_const_char8_t_ptr_,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_10(i) }) as usize) / ::std::mem::size_of::<u8>())
        as i64;
}
pub unsafe fn operator_sub_31(
    i: *const contiguous_iterator_const_char8_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char8_t_ptr__,
) -> i64 {
    return (((unsafe { base_10(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<u8>())
        as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    base_: *const u8,
}
impl sized_sentinel_contiguous_iterator_const_char8_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char8_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_10(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_32(
    s: *const sized_sentinel_const_char16_t_ptr_,
    i: *const *const u16,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_18((*i)) }) as usize)
        / ::std::mem::size_of::<u16>()) as i64;
}
pub unsafe fn operator_sub_33(
    i: *const *const u16,
    s: *const sized_sentinel_const_char16_t_ptr_,
) -> i64 {
    return (((unsafe { base_18((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<u16>()) as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char16_t_ptr_ {
    base_: *const u16,
}
impl sized_sentinel_const_char16_t_ptr_ {
    pub unsafe fn new_1(it: *const *const u16) -> Self {
        let mut this = Self {
            base_: (unsafe { base_18((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_34(
    s: *const sized_sentinel_contiguous_iterator_const_char16_t_ptr__,
    i: *const contiguous_iterator_const_char16_t_ptr_,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_12(i) }) as usize) / ::std::mem::size_of::<u16>())
        as i64;
}
pub unsafe fn operator_sub_35(
    i: *const contiguous_iterator_const_char16_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char16_t_ptr__,
) -> i64 {
    return (((unsafe { base_12(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<u16>())
        as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    base_: *const u16,
}
impl sized_sentinel_contiguous_iterator_const_char16_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char16_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_12(it) }),
        };
        this
    }
}
pub unsafe fn operator_sub_36(
    s: *const sized_sentinel_const_char32_t_ptr_,
    i: *const *const u32,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_19((*i)) }) as usize)
        / ::std::mem::size_of::<u32>()) as i64;
}
pub unsafe fn operator_sub_37(
    i: *const *const u32,
    s: *const sized_sentinel_const_char32_t_ptr_,
) -> i64 {
    return (((unsafe { base_19((*i)) }) as usize - (*s).base_ as usize)
        / ::std::mem::size_of::<u32>()) as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_const_char32_t_ptr_ {
    base_: *const u32,
}
impl sized_sentinel_const_char32_t_ptr_ {
    pub unsafe fn new_1(it: *const *const u32) -> Self {
        let mut this = Self {
            base_: (unsafe { base_19((*it)) }),
        };
        this
    }
}
pub unsafe fn operator_sub_38(
    s: *const sized_sentinel_contiguous_iterator_const_char32_t_ptr__,
    i: *const contiguous_iterator_const_char32_t_ptr_,
) -> i64 {
    return (((*s).base_ as usize - (unsafe { base_14(i) }) as usize) / ::std::mem::size_of::<u32>())
        as i64;
}
pub unsafe fn operator_sub_39(
    i: *const contiguous_iterator_const_char32_t_ptr_,
    s: *const sized_sentinel_contiguous_iterator_const_char32_t_ptr__,
) -> i64 {
    return (((unsafe { base_14(i) }) as usize - (*s).base_ as usize) / ::std::mem::size_of::<u32>())
        as i64;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    base_: *const u32,
}
impl sized_sentinel_contiguous_iterator_const_char32_t_ptr__ {
    pub unsafe fn new_1(it: *const contiguous_iterator_const_char32_t_ptr_) -> Self {
        let mut this = Self {
            base_: (unsafe { base_14(it) }),
        };
        this
    }
}
pub unsafe fn operator_eq_40(
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
            operator_eq_40(
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
pub unsafe fn operator_add_41(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_42(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_41(_i, n)
    });
}
pub unsafe fn iter_swap_43(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_44(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_45(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_46(
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
        unsafe { operator_cmp_46(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_45(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn test_ctad_47(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_48(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_49(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_50(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_51(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_52(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_53(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_54(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_55(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_56(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_57(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_58(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_59(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_60(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_61(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_62(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_63(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_64(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_65(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_66(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_67(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_68(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_69(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_70(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_71(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_72(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_73(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_74(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_75(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_ctad_76(mut val: ()) {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_with_char_77() {
    let val: () = unimplemented!();
    (unsafe { test_ctad_47(unimplemented!()) });
    (unsafe { test_ctad_48(unimplemented!()) });
    (unsafe { test_ctad_49(unimplemented!()) });
    (unsafe { test_ctad_50(unimplemented!()) });
    (unsafe { test_ctad_51(unimplemented!()) });
    (unsafe { test_ctad_52(unimplemented!()) });
}
pub unsafe fn test_with_char_78() {
    let val: () = unimplemented!();
    (unsafe { test_ctad_53(unimplemented!()) });
    (unsafe { test_ctad_54(unimplemented!()) });
    (unsafe { test_ctad_55(unimplemented!()) });
    (unsafe { test_ctad_56(unimplemented!()) });
    (unsafe { test_ctad_57(unimplemented!()) });
    (unsafe { test_ctad_58(unimplemented!()) });
}
pub unsafe fn test_with_char_79() {
    let val: () = unimplemented!();
    (unsafe { test_ctad_59(unimplemented!()) });
    (unsafe { test_ctad_60(unimplemented!()) });
    (unsafe { test_ctad_61(unimplemented!()) });
    (unsafe { test_ctad_62(unimplemented!()) });
    (unsafe { test_ctad_63(unimplemented!()) });
    (unsafe { test_ctad_64(unimplemented!()) });
}
pub unsafe fn test_with_char_80() {
    let val: () = unimplemented!();
    (unsafe { test_ctad_65(unimplemented!()) });
    (unsafe { test_ctad_66(unimplemented!()) });
    (unsafe { test_ctad_67(unimplemented!()) });
    (unsafe { test_ctad_68(unimplemented!()) });
    (unsafe { test_ctad_69(unimplemented!()) });
    (unsafe { test_ctad_70(unimplemented!()) });
}
pub unsafe fn test_with_char_81() {
    let val: () = unimplemented!();
    (unsafe { test_ctad_71(unimplemented!()) });
    (unsafe { test_ctad_72(unimplemented!()) });
    (unsafe { test_ctad_73(unimplemented!()) });
    (unsafe { test_ctad_74(unimplemented!()) });
    (unsafe { test_ctad_75(unimplemented!()) });
    (unsafe { test_ctad_76(unimplemented!()) });
}
pub unsafe fn test_82() {
    (unsafe { test_with_char_77() });
    (unsafe { test_with_char_78() });
    (unsafe { test_with_char_79() });
    (unsafe { test_with_char_80() });
    (unsafe { test_with_char_81() });
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
    (unsafe { test_82() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
