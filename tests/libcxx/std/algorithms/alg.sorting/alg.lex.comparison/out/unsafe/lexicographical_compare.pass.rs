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
pub struct types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__
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
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr____types_type_list_bidirectional_iterator_const_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr____types_type_list_forward_iterator_const_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr____types_type_list_cpp17_input_iterator_const_int_ptr__const_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr____types_type_list_bidirectional_iterator_const_wchar_t_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr____types_type_list_forward_iterator_const_wchar_t_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr____types_type_list_cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr____types_type_list_bidirectional_iterator_const_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr____types_type_list_forward_iterator_const_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr____types_type_list_cpp17_input_iterator_const_char_ptr__const_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr____types_type_list_bidirectional_iterator_unsigned_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr____types_type_list_forward_iterator_unsigned_char_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr____types_type_list_cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: TestIter_const_int_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestIter_const_int_::operator_call_1_const_int_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestIter_const_int_::operator_call_1_class_contiguous_iterator_const_int_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestIter_const_int_::operator_call_1_class_random_access_iterator_const_int_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestIter_const_int_::operator_call_1_class_bidirectional_iterator_const_int_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestIter_const_int_::operator_call_1_class_forward_iterator_const_int_ptr_(&mut f)
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                TestIter_const_int_::operator_call_1_class_cpp17_input_iterator_const_int_ptr_(
                    &mut f,
                )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: Test_const_int__const_int_ptr_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { Test_const_int__const_int_ptr_::operator_call_1_const_int_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_int__const_int_ptr_ :: operator_call_1_class_contiguous_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_int__const_int_ptr_ :: operator_call_1_class_random_access_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_int__const_int_ptr_ :: operator_call_1_class_bidirectional_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_int__const_int_ptr_ :: operator_call_1_class_forward_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_int__const_int_ptr_ :: operator_call_1_class_cpp17_input_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_3(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: Test_const_int__contiguous_iterator_const_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_int__contiguous_iterator_const_int_ptr__::operator_call_1_const_int_ptr(
                    &mut f,
                )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_int__contiguous_iterator_const_int_ptr__ :: operator_call_1_class_contiguous_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_int__contiguous_iterator_const_int_ptr__ :: operator_call_1_class_random_access_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_int__contiguous_iterator_const_int_ptr__ :: operator_call_1_class_bidirectional_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_int__contiguous_iterator_const_int_ptr__ :: operator_call_1_class_forward_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_int__contiguous_iterator_const_int_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_4(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: Test_const_int__random_access_iterator_const_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_int__random_access_iterator_const_int_ptr__ :: operator_call_1_const_int_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_int__random_access_iterator_const_int_ptr__ :: operator_call_1_class_contiguous_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_int__random_access_iterator_const_int_ptr__ :: operator_call_1_class_random_access_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_int__random_access_iterator_const_int_ptr__ :: operator_call_1_class_bidirectional_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_int__random_access_iterator_const_int_ptr__ :: operator_call_1_class_forward_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_int__random_access_iterator_const_int_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_5(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: Test_const_int__bidirectional_iterator_const_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_int__bidirectional_iterator_const_int_ptr__ :: operator_call_1_const_int_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_int__bidirectional_iterator_const_int_ptr__ :: operator_call_1_class_contiguous_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_int__bidirectional_iterator_const_int_ptr__ :: operator_call_1_class_random_access_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_int__bidirectional_iterator_const_int_ptr__ :: operator_call_1_class_bidirectional_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_int__bidirectional_iterator_const_int_ptr__ :: operator_call_1_class_forward_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_int__bidirectional_iterator_const_int_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_6(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: Test_const_int__forward_iterator_const_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_int__forward_iterator_const_int_ptr__::operator_call_1_const_int_ptr(
                    &mut f,
                )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_int__forward_iterator_const_int_ptr__ :: operator_call_1_class_contiguous_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_int__forward_iterator_const_int_ptr__ :: operator_call_1_class_random_access_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_int__forward_iterator_const_int_ptr__ :: operator_call_1_class_bidirectional_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_int__forward_iterator_const_int_ptr__ :: operator_call_1_class_forward_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_int__forward_iterator_const_int_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_7(
    mut _a0 : types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__,
    mut f: Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ :: operator_call_1_const_int_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ :: operator_call_1_class_contiguous_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ :: operator_call_1_class_random_access_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ :: operator_call_1_class_bidirectional_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ :: operator_call_1_class_forward_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_8(
    mut _a0 : types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
    mut f: TestIter_const_wchar_t_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestIter_const_wchar_t_::operator_call_1_const_wchar_t_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestIter_const_wchar_t_ :: operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestIter_const_wchar_t_ :: operator_call_1_class_random_access_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestIter_const_wchar_t_ :: operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestIter_const_wchar_t_::operator_call_1_class_forward_iterator_const_wchar_t_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                TestIter_const_wchar_t_ :: operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_9(
    mut _a0 : types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
    mut f: Test_const_wchar_t__const_wchar_t_ptr_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_wchar_t__const_wchar_t_ptr_::operator_call_1_const_wchar_t_ptr(&mut f)
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_wchar_t__const_wchar_t_ptr_ :: operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_wchar_t__const_wchar_t_ptr_ :: operator_call_1_class_random_access_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_wchar_t__const_wchar_t_ptr_ :: operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_wchar_t__const_wchar_t_ptr_ :: operator_call_1_class_forward_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_wchar_t__const_wchar_t_ptr_ :: operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_10(
    mut _a0 : types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
    mut f: Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ :: operator_call_1_const_wchar_t_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ :: operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ :: operator_call_1_class_random_access_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ :: operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ :: operator_call_1_class_forward_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_11(
    mut _a0 : types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
    mut f: Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ :: operator_call_1_const_wchar_t_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ :: operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ :: operator_call_1_class_random_access_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ :: operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ :: operator_call_1_class_forward_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_12(
    mut _a0 : types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
    mut f: Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ :: operator_call_1_const_wchar_t_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ :: operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ :: operator_call_1_class_random_access_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ :: operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ :: operator_call_1_class_forward_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_13(
    mut _a0 : types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
    mut f: Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ :: operator_call_1_const_wchar_t_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ :: operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ :: operator_call_1_class_random_access_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ :: operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ :: operator_call_1_class_forward_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_14(
    mut _a0 : types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
    mut f: Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ :: operator_call_1_const_wchar_t_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ :: operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ :: operator_call_1_class_random_access_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ :: operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ :: operator_call_1_class_forward_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_15(
    mut _a0 : types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__,
    mut f: TestIter_const_char_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestIter_const_char_::operator_call_1_const_char_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestIter_const_char_::operator_call_1_class_contiguous_iterator_const_char_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestIter_const_char_::operator_call_1_class_random_access_iterator_const_char_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestIter_const_char_::operator_call_1_class_bidirectional_iterator_const_char_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestIter_const_char_::operator_call_1_class_forward_iterator_const_char_ptr_(&mut f)
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                TestIter_const_char_::operator_call_1_class_cpp17_input_iterator_const_char_ptr_(
                    &mut f,
                )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_16(
    mut _a0 : types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__,
    mut f: Test_const_char__const_char_ptr_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { Test_const_char__const_char_ptr_::operator_call_1_const_char_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_char__const_char_ptr_ :: operator_call_1_class_contiguous_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_char__const_char_ptr_ :: operator_call_1_class_random_access_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_char__const_char_ptr_ :: operator_call_1_class_bidirectional_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_char__const_char_ptr_ :: operator_call_1_class_forward_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_char__const_char_ptr_ :: operator_call_1_class_cpp17_input_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_17(
    mut _a0 : types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__,
    mut f: Test_const_char__contiguous_iterator_const_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_char__contiguous_iterator_const_char_ptr__ :: operator_call_1_const_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_char__contiguous_iterator_const_char_ptr__ :: operator_call_1_class_contiguous_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_char__contiguous_iterator_const_char_ptr__ :: operator_call_1_class_random_access_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_char__contiguous_iterator_const_char_ptr__ :: operator_call_1_class_bidirectional_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_char__contiguous_iterator_const_char_ptr__ :: operator_call_1_class_forward_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_char__contiguous_iterator_const_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_18(
    mut _a0 : types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__,
    mut f: Test_const_char__random_access_iterator_const_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_char__random_access_iterator_const_char_ptr__ :: operator_call_1_const_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_char__random_access_iterator_const_char_ptr__ :: operator_call_1_class_contiguous_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_char__random_access_iterator_const_char_ptr__ :: operator_call_1_class_random_access_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_char__random_access_iterator_const_char_ptr__ :: operator_call_1_class_bidirectional_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_char__random_access_iterator_const_char_ptr__ :: operator_call_1_class_forward_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_char__random_access_iterator_const_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_19(
    mut _a0 : types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__,
    mut f: Test_const_char__bidirectional_iterator_const_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_char__bidirectional_iterator_const_char_ptr__ :: operator_call_1_const_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_char__bidirectional_iterator_const_char_ptr__ :: operator_call_1_class_contiguous_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_char__bidirectional_iterator_const_char_ptr__ :: operator_call_1_class_random_access_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_char__bidirectional_iterator_const_char_ptr__ :: operator_call_1_class_bidirectional_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_char__bidirectional_iterator_const_char_ptr__ :: operator_call_1_class_forward_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_char__bidirectional_iterator_const_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_20(
    mut _a0 : types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__,
    mut f: Test_const_char__forward_iterator_const_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_char__forward_iterator_const_char_ptr__::operator_call_1_const_char_ptr(
                    &mut f,
                )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_char__forward_iterator_const_char_ptr__ :: operator_call_1_class_contiguous_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_char__forward_iterator_const_char_ptr__ :: operator_call_1_class_random_access_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_char__forward_iterator_const_char_ptr__ :: operator_call_1_class_bidirectional_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_char__forward_iterator_const_char_ptr__ :: operator_call_1_class_forward_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_char__forward_iterator_const_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_21(
    mut _a0 : types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__,
    mut f: Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ :: operator_call_1_const_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ :: operator_call_1_class_contiguous_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ :: operator_call_1_class_random_access_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ :: operator_call_1_class_bidirectional_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ :: operator_call_1_class_forward_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_const_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_22(
    mut _a0 : types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
    mut f: TestIter_unsigned_char_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestIter_unsigned_char_::operator_call_1_unsigned_char_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestIter_unsigned_char_ :: operator_call_1_class_contiguous_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestIter_unsigned_char_ :: operator_call_1_class_random_access_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestIter_unsigned_char_ :: operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestIter_unsigned_char_::operator_call_1_class_forward_iterator_unsigned_char_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                TestIter_unsigned_char_ :: operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_23(
    mut _a0 : types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
    mut f: Test_unsigned_char__unsigned_char_ptr_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_unsigned_char__unsigned_char_ptr_::operator_call_1_unsigned_char_ptr(&mut f)
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_unsigned_char__unsigned_char_ptr_ :: operator_call_1_class_contiguous_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_unsigned_char__unsigned_char_ptr_ :: operator_call_1_class_random_access_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_unsigned_char__unsigned_char_ptr_ :: operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_unsigned_char__unsigned_char_ptr_ :: operator_call_1_class_forward_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_unsigned_char__unsigned_char_ptr_ :: operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_24(
    mut _a0 : types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
    mut f: Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ :: operator_call_1_unsigned_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ :: operator_call_1_class_contiguous_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ :: operator_call_1_class_random_access_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ :: operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ :: operator_call_1_class_forward_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_25(
    mut _a0 : types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
    mut f: Test_unsigned_char__random_access_iterator_unsigned_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ :: operator_call_1_unsigned_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ :: operator_call_1_class_contiguous_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ :: operator_call_1_class_random_access_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ :: operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ :: operator_call_1_class_forward_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_26(
    mut _a0 : types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
    mut f: Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ :: operator_call_1_unsigned_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ :: operator_call_1_class_contiguous_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ :: operator_call_1_class_random_access_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ :: operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ :: operator_call_1_class_forward_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_27(
    mut _a0 : types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
    mut f: Test_unsigned_char__forward_iterator_unsigned_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_unsigned_char__forward_iterator_unsigned_char_ptr__ :: operator_call_1_unsigned_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_unsigned_char__forward_iterator_unsigned_char_ptr__ :: operator_call_1_class_contiguous_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_unsigned_char__forward_iterator_unsigned_char_ptr__ :: operator_call_1_class_random_access_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_unsigned_char__forward_iterator_unsigned_char_ptr__ :: operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_unsigned_char__forward_iterator_unsigned_char_ptr__ :: operator_call_1_class_forward_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_unsigned_char__forward_iterator_unsigned_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn for_each_28(
    mut _a0 : types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
    mut f: Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ :: operator_call_1_unsigned_char_ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ :: operator_call_1_class_contiguous_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ :: operator_call_1_class_random_access_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ :: operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ :: operator_call_1_class_forward_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg5: i32 = {
            (unsafe {
                Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ :: operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn swallow_1(
    mut _a0: i32,
    mut _a1: i32,
    mut _a2: i32,
    mut _a3: i32,
    mut _a4: i32,
    mut _a5: i32,
) {
}
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
pub unsafe fn operator_eq_29(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_30(
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
impl std::cmp::PartialEq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_29(
                self as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
                other as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
pub unsafe fn operator_eq_31(
    x: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    y: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_32(
    x: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    y: *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
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
    ) -> *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
    ) -> *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_));
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_31(
                self as *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
                other as *const cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
pub unsafe fn operator_eq_33(
    x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    y: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_34(
    x: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    y: *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    pub unsafe fn new(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_,
    ) -> *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_char_ptr__const_char_ptr_));
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_33(
                self as *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
                other as *const cpp17_input_iterator_const_char_ptr__const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_char_ptr__const_char_ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_const_char_ptr__const_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
pub unsafe fn operator_eq_35(
    x: *const cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
    y: *const cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_36(
    x: *const cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
    y: *const cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_ {
    it_: *mut u8,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_ {
    pub unsafe fn new(mut it: *mut u8) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut u8 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_));
    }
    pub unsafe fn move_from(
        _a0: *mut cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
    ) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
    ) -> *mut cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_));
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_35(
                self as *const cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
                other as *const cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_37(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_38(
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
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_37(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_39(
    x: *const forward_iterator_const_wchar_t_ptr_,
    y: *const forward_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_40(
    x: *const forward_iterator_const_wchar_t_ptr_,
    y: *const forward_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_wchar_t_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_wchar_t_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_wchar_t_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_wchar_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_wchar_t_ptr_,
    ) -> *mut forward_iterator_const_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_wchar_t_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_39(
                self as *const forward_iterator_const_wchar_t_ptr_,
                other as *const forward_iterator_const_wchar_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_wchar_t_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_const_wchar_t_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_41(
    x: *const forward_iterator_const_char_ptr_,
    y: *const forward_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_42(
    x: *const forward_iterator_const_char_ptr_,
    y: *const forward_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_char_ptr_,
    ) -> *mut forward_iterator_const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_char_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_41(
                self as *const forward_iterator_const_char_ptr_,
                other as *const forward_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_char_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_const_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_43(
    x: *const forward_iterator_unsigned_char_ptr_,
    y: *const forward_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_44(
    x: *const forward_iterator_unsigned_char_ptr_,
    y: *const forward_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_unsigned_char_ptr_ {
    it_: *mut u8,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_unsigned_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut u8) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut u8 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_unsigned_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_unsigned_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_unsigned_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_unsigned_char_ptr_,
    ) -> *mut forward_iterator_unsigned_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_unsigned_char_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_unsigned_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_43(
                self as *const forward_iterator_unsigned_char_ptr_,
                other as *const forward_iterator_unsigned_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_unsigned_char_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_unsigned_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_45(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_46(
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
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_45(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_47(
    x: *const bidirectional_iterator_const_wchar_t_ptr_,
    y: *const bidirectional_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_48(
    x: *const bidirectional_iterator_const_wchar_t_ptr_,
    y: *const bidirectional_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_const_wchar_t_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_wchar_t_ptr_ {
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
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_wchar_t_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_wchar_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_const_wchar_t_ptr_,
    ) -> *mut bidirectional_iterator_const_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_const_wchar_t_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_47(
                self as *const bidirectional_iterator_const_wchar_t_ptr_,
                other as *const bidirectional_iterator_const_wchar_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_wchar_t_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_const_wchar_t_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_49(
    x: *const bidirectional_iterator_const_char_ptr_,
    y: *const bidirectional_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_50(
    x: *const bidirectional_iterator_const_char_ptr_,
    y: *const bidirectional_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_const_char_ptr_,
    ) -> *mut bidirectional_iterator_const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_const_char_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_49(
                self as *const bidirectional_iterator_const_char_ptr_,
                other as *const bidirectional_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_char_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_const_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_51(
    x: *const bidirectional_iterator_unsigned_char_ptr_,
    y: *const bidirectional_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_52(
    x: *const bidirectional_iterator_unsigned_char_ptr_,
    y: *const bidirectional_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_unsigned_char_ptr_ {
    it_: *mut u8,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_unsigned_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut u8) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut u8 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_unsigned_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_unsigned_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_unsigned_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_unsigned_char_ptr_,
    ) -> *mut bidirectional_iterator_unsigned_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_unsigned_char_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_unsigned_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_51(
                self as *const bidirectional_iterator_unsigned_char_ptr_,
                other as *const bidirectional_iterator_unsigned_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_unsigned_char_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_unsigned_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_add_53(
    mut x: random_access_iterator_const_int_ptr_,
    mut n: isize,
) -> random_access_iterator_const_int_ptr_ {
    (unsafe { random_access_iterator_const_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_54(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_55(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_56(
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
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
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
            operator_eq_55(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
pub unsafe fn operator_add_57(
    mut x: random_access_iterator_const_wchar_t_ptr_,
    mut n: isize,
) -> random_access_iterator_const_wchar_t_ptr_ {
    (unsafe { random_access_iterator_const_wchar_t_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_wchar_t_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_58(
    mut x: random_access_iterator_const_wchar_t_ptr_,
    mut y: random_access_iterator_const_wchar_t_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_59(
    x: *const random_access_iterator_const_wchar_t_ptr_,
    y: *const random_access_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_60(
    x: *const random_access_iterator_const_wchar_t_ptr_,
    y: *const random_access_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_wchar_t_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_wchar_t_ptr_ {
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
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_wchar_t_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_const_wchar_t_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_wchar_t_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_wchar_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_wchar_t_ptr_,
    ) -> *mut random_access_iterator_const_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_wchar_t_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_59(
                self as *const random_access_iterator_const_wchar_t_ptr_,
                other as *const random_access_iterator_const_wchar_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_wchar_t_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_const_wchar_t_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
pub unsafe fn operator_add_61(
    mut x: random_access_iterator_const_char_ptr_,
    mut n: isize,
) -> random_access_iterator_const_char_ptr_ {
    (unsafe { random_access_iterator_const_char_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_char_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_62(
    mut x: random_access_iterator_const_char_ptr_,
    mut y: random_access_iterator_const_char_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<libc::c_char>()) as i64
        as isize);
}
pub unsafe fn operator_eq_63(
    x: *const random_access_iterator_const_char_ptr_,
    y: *const random_access_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_64(
    x: *const random_access_iterator_const_char_ptr_,
    y: *const random_access_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_char_ptr_,
    ) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_63(
                self as *const random_access_iterator_const_char_ptr_,
                other as *const random_access_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_char_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_const_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
pub unsafe fn operator_add_65(
    mut x: random_access_iterator_unsigned_char_ptr_,
    mut n: isize,
) -> random_access_iterator_unsigned_char_ptr_ {
    (unsafe { random_access_iterator_unsigned_char_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_unsigned_char_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_66(
    mut x: random_access_iterator_unsigned_char_ptr_,
    mut y: random_access_iterator_unsigned_char_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<u8>()) as i64 as isize);
}
pub unsafe fn operator_eq_67(
    x: *const random_access_iterator_unsigned_char_ptr_,
    y: *const random_access_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_68(
    x: *const random_access_iterator_unsigned_char_ptr_,
    y: *const random_access_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_unsigned_char_ptr_ {
    it_: *mut u8,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_unsigned_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut u8) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut u8 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_unsigned_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_unsigned_char_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_unsigned_char_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_unsigned_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_unsigned_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_unsigned_char_ptr_,
    ) -> *mut random_access_iterator_unsigned_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_unsigned_char_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_unsigned_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_67(
                self as *const random_access_iterator_unsigned_char_ptr_,
                other as *const random_access_iterator_unsigned_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_unsigned_char_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_unsigned_char_ptr_ {
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
pub unsafe fn operator_eq_69(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_70(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
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
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_int_ptr_,
    ) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_69(
                self as *const contiguous_iterator_const_int_ptr_,
                other as *const contiguous_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
pub unsafe fn operator_eq_71(
    x: *const contiguous_iterator_const_wchar_t_ptr_,
    y: *const contiguous_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_72(
    x: *const contiguous_iterator_const_wchar_t_ptr_,
    y: *const contiguous_iterator_const_wchar_t_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
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
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_wchar_t_ptr_));
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
impl std::cmp::PartialEq for contiguous_iterator_const_wchar_t_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_71(
                self as *const contiguous_iterator_const_wchar_t_ptr_,
                other as *const contiguous_iterator_const_wchar_t_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_wchar_t_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_const_wchar_t_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
pub unsafe fn operator_eq_73(
    x: *const contiguous_iterator_const_char_ptr_,
    y: *const contiguous_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_74(
    x: *const contiguous_iterator_const_char_ptr_,
    y: *const contiguous_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
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
    pub unsafe fn operator_deref(&self) -> *const libc::c_char {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_char_ptr_));
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
impl std::cmp::PartialEq for contiguous_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_73(
                self as *const contiguous_iterator_const_char_ptr_,
                other as *const contiguous_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_char_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_const_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
pub unsafe fn operator_eq_75(
    x: *const contiguous_iterator_unsigned_char_ptr_,
    y: *const contiguous_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_76(
    x: *const contiguous_iterator_unsigned_char_ptr_,
    y: *const contiguous_iterator_unsigned_char_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_unsigned_char_ptr_ {
    it_: *mut u8,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_unsigned_char_ptr_ {
    pub unsafe fn new_1(mut it: *mut u8) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut u8 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_unsigned_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_unsigned_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_unsigned_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_unsigned_char_ptr_,
    ) -> *mut contiguous_iterator_unsigned_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_unsigned_char_ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_unsigned_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_75(
                self as *const contiguous_iterator_unsigned_char_ptr_,
                other as *const contiguous_iterator_unsigned_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_unsigned_char_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_unsigned_char_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
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
pub unsafe fn operator_eq_77(
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
            operator_eq_77(
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
pub unsafe fn operator_add_78(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_79(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_78(_i, n)
    });
}
pub unsafe fn iter_swap_80(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_81(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_82(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_83(
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
        unsafe { operator_cmp_83(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_82(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct Test_const_int__const_int_ptr_ {}
impl Test_const_int__const_int_ptr_ {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_int__contiguous_iterator_const_int_ptr__ {}
impl Test_const_int__contiguous_iterator_const_int_ptr__ {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_int__random_access_iterator_const_int_ptr__ {}
impl Test_const_int__random_access_iterator_const_int_ptr__ {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_int__bidirectional_iterator_const_int_ptr__ {}
impl Test_const_int__bidirectional_iterator_const_int_ptr__ {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_int__forward_iterator_const_int_ptr__ {}
impl Test_const_int__forward_iterator_const_int_ptr__ {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ {}
impl Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) {
        let ia: [i32; 4] = [1, 2, 3, 4];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1, 2, 3];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_wchar_t__const_wchar_t_ptr_ {}
impl Test_const_wchar_t__const_wchar_t_ptr_ {
    pub unsafe fn operator_call_1_const_wchar_t_ptr(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ {}
impl Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ {
    pub unsafe fn operator_call_1_const_wchar_t_ptr(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ {}
impl Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ {
    pub unsafe fn operator_call_1_const_wchar_t_ptr(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ {}
impl Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ {
    pub unsafe fn operator_call_1_const_wchar_t_ptr(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ {}
impl Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ {
    pub unsafe fn operator_call_1_const_wchar_t_ptr(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ {}
impl Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ {
    pub unsafe fn operator_call_1_const_wchar_t_ptr(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_(&mut self) {
        let ia: [i32; 4] = [1_i32, 2_i32, 3_i32, 4_i32];
        let sa: u32 = (((::std::mem::size_of::<[i32; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let ib: [i32; 3] = [1_i32, 2_i32, 3_i32];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_char__const_char_ptr_ {}
impl Test_const_char__const_char_ptr_ {
    pub unsafe fn operator_call_1_const_char_ptr(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_char__contiguous_iterator_const_char_ptr__ {}
impl Test_const_char__contiguous_iterator_const_char_ptr__ {
    pub unsafe fn operator_call_1_const_char_ptr(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_char__random_access_iterator_const_char_ptr__ {}
impl Test_const_char__random_access_iterator_const_char_ptr__ {
    pub unsafe fn operator_call_1_const_char_ptr(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_char__bidirectional_iterator_const_char_ptr__ {}
impl Test_const_char__bidirectional_iterator_const_char_ptr__ {
    pub unsafe fn operator_call_1_const_char_ptr(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_char__forward_iterator_const_char_ptr__ {}
impl Test_const_char__forward_iterator_const_char_ptr__ {
    pub unsafe fn operator_call_1_const_char_ptr(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ {}
impl Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ {
    pub unsafe fn operator_call_1_const_char_ptr(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_char_ptr_(&mut self) {
        let ia: [libc::c_char; 4] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
            (4 as libc::c_char),
        ];
        let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize)))
            as u32);
        let ib: [libc::c_char; 3] = [
            (1 as libc::c_char),
            (2 as libc::c_char),
            (3 as libc::c_char),
        ];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_unsigned_char__unsigned_char_ptr_ {}
impl Test_unsigned_char__unsigned_char_ptr_ {
    pub unsafe fn operator_call_1_unsigned_char_ptr(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ {}
impl Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ {
    pub unsafe fn operator_call_1_unsigned_char_ptr(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ {}
impl Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ {
    pub unsafe fn operator_call_1_unsigned_char_ptr(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ {}
impl Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ {
    pub unsafe fn operator_call_1_unsigned_char_ptr(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_unsigned_char__forward_iterator_unsigned_char_ptr__ {}
impl Test_unsigned_char__forward_iterator_unsigned_char_ptr__ {
    pub unsafe fn operator_call_1_unsigned_char_ptr(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ {}
impl Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ {
    pub unsafe fn operator_call_1_unsigned_char_ptr(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_forward_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_(&mut self) {
        let mut ia: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
        let sa: u32 = (((::std::mem::size_of::<[u8; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<u8>() as usize))) as u32);
        let mut ib: [u8; 3] = [1_u8, 2_u8, 3_u8];
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIter_const_int_ {}
impl TestIter_const_int_ {
    pub unsafe fn operator_call_1_const_int_ptr(&mut self) -> bool {
        (unsafe {
            for_each_2 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <Test_const_int__const_int_ptr_ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_int_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_3 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <Test_const_int__contiguous_iterator_const_int_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_int_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_4 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <Test_const_int__random_access_iterator_const_int_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_int_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_5 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <Test_const_int__bidirectional_iterator_const_int_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_int_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_6 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <Test_const_int__forward_iterator_const_int_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_int_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_7 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <Test_const_int__cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , )
        });
        return true;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIter_const_wchar_t_ {}
impl TestIter_const_wchar_t_ {
    pub unsafe fn operator_call_1_const_wchar_t_ptr(&mut self) -> bool {
        (unsafe {
            for_each_9 ( <types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , <Test_const_wchar_t__const_wchar_t_ptr_ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_wchar_t_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_10 ( <types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , <Test_const_wchar_t__contiguous_iterator_const_wchar_t_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_wchar_t_ptr_(
        &mut self,
    ) -> bool {
        (unsafe {
            for_each_11 ( <types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , <Test_const_wchar_t__random_access_iterator_const_wchar_t_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_wchar_t_ptr_(
        &mut self,
    ) -> bool {
        (unsafe {
            for_each_12 ( <types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , <Test_const_wchar_t__bidirectional_iterator_const_wchar_t_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_wchar_t_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_13 ( <types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , <Test_const_wchar_t__forward_iterator_const_wchar_t_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_wchar_t_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_14 ( <types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , <Test_const_wchar_t__cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , )
        });
        return true;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIter_const_char_ {}
impl TestIter_const_char_ {
    pub unsafe fn operator_call_1_const_char_ptr(&mut self) -> bool {
        (unsafe {
            for_each_16 ( <types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , <Test_const_char__const_char_ptr_ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_const_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_17 ( <types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , <Test_const_char__contiguous_iterator_const_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_const_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_18 ( <types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , <Test_const_char__random_access_iterator_const_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_const_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_19 ( <types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , <Test_const_char__bidirectional_iterator_const_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_forward_iterator_const_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_20 ( <types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , <Test_const_char__forward_iterator_const_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_const_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_21 ( <types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , <Test_const_char__cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , )
        });
        return true;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIter_unsigned_char_ {}
impl TestIter_unsigned_char_ {
    pub unsafe fn operator_call_1_unsigned_char_ptr(&mut self) -> bool {
        (unsafe {
            for_each_23 ( <types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , <Test_unsigned_char__unsigned_char_ptr_ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_unsigned_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_24 ( <types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , <Test_unsigned_char__contiguous_iterator_unsigned_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_unsigned_char_ptr_(
        &mut self,
    ) -> bool {
        (unsafe {
            for_each_25 ( <types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , <Test_unsigned_char__random_access_iterator_unsigned_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_unsigned_char_ptr_(
        &mut self,
    ) -> bool {
        (unsafe {
            for_each_26 ( <types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , <Test_unsigned_char__bidirectional_iterator_unsigned_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_forward_iterator_unsigned_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_27 ( <types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , <Test_unsigned_char__forward_iterator_unsigned_char_ptr__ >::default() , )
        });
        return true;
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_unsigned_char_ptr_(&mut self) -> bool {
        (unsafe {
            for_each_28 ( <types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , <Test_unsigned_char__cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , )
        });
        return true;
    }
}
pub unsafe fn test_84() -> bool {
    (unsafe {
        for_each_0 ( <types_type_list_const_int_ptr__contiguous_iterator_const_int_ptr___random_access_iterator_const_int_ptr___bidirectional_iterator_const_int_ptr___forward_iterator_const_int_ptr___cpp17_input_iterator_const_int_ptr__const_int_ptr__ >::default() , <TestIter_const_int_ >::default() , )
    });
    (unsafe {
        for_each_8 ( <types_type_list_const_wchar_t_ptr__contiguous_iterator_const_wchar_t_ptr___random_access_iterator_const_wchar_t_ptr___bidirectional_iterator_const_wchar_t_ptr___forward_iterator_const_wchar_t_ptr___cpp17_input_iterator_const_wchar_t_ptr__const_wchar_t_ptr__ >::default() , <TestIter_const_wchar_t_ >::default() , )
    });
    (unsafe {
        for_each_15 ( <types_type_list_const_char_ptr__contiguous_iterator_const_char_ptr___random_access_iterator_const_char_ptr___bidirectional_iterator_const_char_ptr___forward_iterator_const_char_ptr___cpp17_input_iterator_const_char_ptr__const_char_ptr__ >::default() , <TestIter_const_char_ >::default() , )
    });
    (unsafe {
        for_each_22 ( <types_type_list_unsigned_char_ptr__contiguous_iterator_unsigned_char_ptr___random_access_iterator_unsigned_char_ptr___bidirectional_iterator_unsigned_char_ptr___forward_iterator_unsigned_char_ptr___cpp17_input_iterator_unsigned_char_ptr__unsigned_char_ptr__ >::default() , <TestIter_unsigned_char_ >::default() , )
    });
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
    (unsafe { test_84() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
