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
    mut f: TestPtr,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestPtr::operator_call_1_int_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe { TestPtr::operator_call_1_class_contiguous_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg2: i32 = {
            (unsafe { TestPtr::operator_call_1_class_random_access_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg3: i32 = {
            (unsafe { TestPtr::operator_call_1_class_bidirectional_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg4: i32 = {
            (unsafe { TestPtr::operator_call_1_class_forward_iterator_int_ptr_(&mut f) });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: TestPtr_TestImpl_int_ptr_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestPtr_TestImpl_int_ptr_::operator_call_1_int_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestPtr_TestImpl_int_ptr_::operator_call_1_class_contiguous_iterator_int_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestPtr_TestImpl_int_ptr_::operator_call_1_class_random_access_iterator_int_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestPtr_TestImpl_int_ptr_::operator_call_1_class_bidirectional_iterator_int_ptr_(
                    &mut f,
                )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestPtr_TestImpl_int_ptr_::operator_call_1_class_forward_iterator_int_ptr_(&mut f)
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_3(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: TestPtr_TestImpl_contiguous_iterator_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestPtr_TestImpl_contiguous_iterator_int_ptr__::operator_call_1_int_ptr(&mut f)
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestPtr_TestImpl_contiguous_iterator_int_ptr__ :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestPtr_TestImpl_contiguous_iterator_int_ptr__ :: operator_call_1_class_random_access_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestPtr_TestImpl_contiguous_iterator_int_ptr__ :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestPtr_TestImpl_contiguous_iterator_int_ptr__ :: operator_call_1_class_forward_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_4(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: TestPtr_TestImpl_random_access_iterator_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestPtr_TestImpl_random_access_iterator_int_ptr__::operator_call_1_int_ptr(&mut f)
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestPtr_TestImpl_random_access_iterator_int_ptr__ :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestPtr_TestImpl_random_access_iterator_int_ptr__ :: operator_call_1_class_random_access_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestPtr_TestImpl_random_access_iterator_int_ptr__ :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestPtr_TestImpl_random_access_iterator_int_ptr__ :: operator_call_1_class_forward_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_5(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: TestPtr_TestImpl_bidirectional_iterator_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestPtr_TestImpl_bidirectional_iterator_int_ptr__::operator_call_1_int_ptr(&mut f)
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestPtr_TestImpl_bidirectional_iterator_int_ptr__ :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestPtr_TestImpl_bidirectional_iterator_int_ptr__ :: operator_call_1_class_random_access_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestPtr_TestImpl_bidirectional_iterator_int_ptr__ :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestPtr_TestImpl_bidirectional_iterator_int_ptr__ :: operator_call_1_class_forward_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_6(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__,
    mut f: TestPtr_TestImpl_forward_iterator_int_ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestPtr_TestImpl_forward_iterator_int_ptr__::operator_call_1_int_ptr(&mut f)
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestPtr_TestImpl_forward_iterator_int_ptr__ :: operator_call_1_class_contiguous_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestPtr_TestImpl_forward_iterator_int_ptr__ :: operator_call_1_class_random_access_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestPtr_TestImpl_forward_iterator_int_ptr__ :: operator_call_1_class_bidirectional_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestPtr_TestImpl_forward_iterator_int_ptr__ :: operator_call_1_class_forward_iterator_int_ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_7(
    mut _a0 : types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
    mut f: TestUniquePtr,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { TestUniquePtr::operator_call_1_class_std_unique_ptr_int__ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestUniquePtr :: operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestUniquePtr :: operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestUniquePtr :: operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestUniquePtr::operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(
                    &mut f,
                )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_8(
    mut _a0 : types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
    mut f: TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ :: operator_call_1_class_std_unique_ptr_int__ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ :: operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ :: operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ :: operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ :: operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_9(
    mut _a0 : types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
    mut f : TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_std_unique_ptr_int__ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_10(
    mut _a0 : types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
    mut f : TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_std_unique_ptr_int__ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_11(
    mut _a0 : types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
    mut f : TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_std_unique_ptr_int__ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4)
    });
}
pub unsafe fn for_each_12(
    mut _a0 : types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
    mut f : TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_std_unique_ptr_int__ptr ( &mut f  , )
            });
            0
        };
        let _arg1: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg2: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg3: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
            });
            0
        };
        let _arg4: i32 = {
            (unsafe {
                TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ :: operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_ ( &mut f  , )
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
pub unsafe fn operator_ne_13(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_14(i: *const forward_iterator_int_ptr_) -> *mut i32 {
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
            !(operator_ne_13(
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
pub unsafe fn operator_ne_15(
    x: *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_16(
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
            !(operator_ne_15(
                self as *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
                other as *const forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Option<Box<i32>>>
    for forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
    fn iter_deref(&self) -> *mut Option<Box<i32>> {
        unsafe {
            forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_deref(self)
                as *mut Option<Box<i32>>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_inc_6(
                self,
            );
        }
        self
    }
}
impl UnsafeCppInputIterator<Option<Box<i32>>>
    for forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
}
pub unsafe fn operator_ne_17(
    x: *const forward_iterator_std_array_int__2__ptr_,
    y: *const forward_iterator_std_array_int__2__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_std_array_int__2__ptr_ {
    it_: *mut Vec<i32>,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std_array_int__2__ptr_ {
    pub unsafe fn new_1(mut it: *mut Vec<i32>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Vec<i32> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_std_array_int__2__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_std_array_int__2__ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_std_array_int__2__ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_std_array_int__2__ptr_,
    ) -> *mut forward_iterator_std_array_int__2__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_std_array_int__2__ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_std_array_int__2__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_17(
                self as *const forward_iterator_std_array_int__2__ptr_,
                other as *const forward_iterator_std_array_int__2__ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for forward_iterator_std_array_int__2__ptr_ {}
impl UnsafeCppAdvance for forward_iterator_std_array_int__2__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Vec<i32>> for forward_iterator_std_array_int__2__ptr_ {
    fn iter_deref(&self) -> *mut Vec<i32> {
        unsafe { forward_iterator_std_array_int__2__ptr_::operator_deref(self) as *mut Vec<i32> }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            forward_iterator_std_array_int__2__ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Vec<i32>> for forward_iterator_std_array_int__2__ptr_ {}
pub unsafe fn operator_ne_18(
    x: *const forward_iterator_std_array_int__3__ptr_,
    y: *const forward_iterator_std_array_int__3__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_std_array_int__3__ptr_ {
    it_: *mut Vec<i32>,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_std_array_int__3__ptr_ {
    pub unsafe fn new_1(mut it: *mut Vec<i32>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Vec<i32> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_std_array_int__3__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_std_array_int__3__ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_std_array_int__3__ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_std_array_int__3__ptr_,
    ) -> *mut forward_iterator_std_array_int__3__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_std_array_int__3__ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_std_array_int__3__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_18(
                self as *const forward_iterator_std_array_int__3__ptr_,
                other as *const forward_iterator_std_array_int__3__ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for forward_iterator_std_array_int__3__ptr_ {}
impl UnsafeCppAdvance for forward_iterator_std_array_int__3__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Vec<i32>> for forward_iterator_std_array_int__3__ptr_ {
    fn iter_deref(&self) -> *mut Vec<i32> {
        unsafe { forward_iterator_std_array_int__3__ptr_::operator_deref(self) as *mut Vec<i32> }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            forward_iterator_std_array_int__3__ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Vec<i32>> for forward_iterator_std_array_int__3__ptr_ {}
pub unsafe fn operator_ne_19(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_20(i: *const bidirectional_iterator_int_ptr_) -> *mut i32 {
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
            !(operator_ne_19(
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
pub unsafe fn operator_ne_21(
    x: *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_22(
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
            !(operator_ne_21(self as *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_, other as *const bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_))
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Option<Box<i32>>>
    for bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
    fn iter_deref(&self) -> *mut Option<Box<i32>> {
        unsafe {
            bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_deref(
                self,
            ) as *mut Option<Box<i32>>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Option<Box<i32>>>
    for bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
}
pub unsafe fn operator_ne_23(
    x: *const bidirectional_iterator_std_array_int__2__ptr_,
    y: *const bidirectional_iterator_std_array_int__2__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_std_array_int__2__ptr_ {
    it_: *mut Vec<i32>,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_std_array_int__2__ptr_ {
    pub unsafe fn new_1(mut it: *mut Vec<i32>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Vec<i32> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_std_array_int__2__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_std_array_int__2__ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_std_array_int__2__ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_std_array_int__2__ptr_,
    ) -> *mut bidirectional_iterator_std_array_int__2__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_std_array_int__2__ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_std_array_int__2__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_23(
                self as *const bidirectional_iterator_std_array_int__2__ptr_,
                other as *const bidirectional_iterator_std_array_int__2__ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_std_array_int__2__ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_std_array_int__2__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Vec<i32>> for bidirectional_iterator_std_array_int__2__ptr_ {
    fn iter_deref(&self) -> *mut Vec<i32> {
        unsafe {
            bidirectional_iterator_std_array_int__2__ptr_::operator_deref(self) as *mut Vec<i32>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            bidirectional_iterator_std_array_int__2__ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Vec<i32>> for bidirectional_iterator_std_array_int__2__ptr_ {}
pub unsafe fn operator_ne_24(
    x: *const bidirectional_iterator_std_array_int__3__ptr_,
    y: *const bidirectional_iterator_std_array_int__3__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_std_array_int__3__ptr_ {
    it_: *mut Vec<i32>,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_std_array_int__3__ptr_ {
    pub unsafe fn new_1(mut it: *mut Vec<i32>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Vec<i32> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_std_array_int__3__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_std_array_int__3__ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_std_array_int__3__ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_std_array_int__3__ptr_,
    ) -> *mut bidirectional_iterator_std_array_int__3__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_std_array_int__3__ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_std_array_int__3__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_24(
                self as *const bidirectional_iterator_std_array_int__3__ptr_,
                other as *const bidirectional_iterator_std_array_int__3__ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_std_array_int__3__ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_std_array_int__3__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<Vec<i32>> for bidirectional_iterator_std_array_int__3__ptr_ {
    fn iter_deref(&self) -> *mut Vec<i32> {
        unsafe {
            bidirectional_iterator_std_array_int__3__ptr_::operator_deref(self) as *mut Vec<i32>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            bidirectional_iterator_std_array_int__3__ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Vec<i32>> for bidirectional_iterator_std_array_int__3__ptr_ {}
pub unsafe fn operator_ne_25(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_26(i: *const random_access_iterator_int_ptr_) -> *mut i32 {
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
            !(operator_ne_25(
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
pub unsafe fn operator_ne_27(
    x: *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_28(
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
            !(operator_ne_27(self as *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_, other as *const random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_))
        }
    }
}
impl std::cmp::Eq for random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
impl UnsafeCppIterator<Option<Box<i32>>>
    for random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
    fn iter_deref(&self) -> *mut Option<Box<i32>> {
        unsafe {
            random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_deref(
                self,
            ) as *mut Option<Box<i32>>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_inc_7(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Option<Box<i32>>>
    for random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
}
pub unsafe fn operator_ne_29(
    x: *const random_access_iterator_std_array_int__2__ptr_,
    y: *const random_access_iterator_std_array_int__2__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std_array_int__2__ptr_ {
    it_: *mut Vec<i32>,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std_array_int__2__ptr_ {
    pub unsafe fn new_1(mut it: *mut Vec<i32>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Vec<i32> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_std_array_int__2__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_std_array_int__2__ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_std_array_int__2__ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_std_array_int__2__ptr_,
    ) -> *mut random_access_iterator_std_array_int__2__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_std_array_int__2__ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_std_array_int__2__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_29(
                self as *const random_access_iterator_std_array_int__2__ptr_,
                other as *const random_access_iterator_std_array_int__2__ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for random_access_iterator_std_array_int__2__ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_std_array_int__2__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
impl UnsafeCppIterator<Vec<i32>> for random_access_iterator_std_array_int__2__ptr_ {
    fn iter_deref(&self) -> *mut Vec<i32> {
        unsafe {
            random_access_iterator_std_array_int__2__ptr_::operator_deref(self) as *mut Vec<i32>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            random_access_iterator_std_array_int__2__ptr_::operator_inc_7(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Vec<i32>> for random_access_iterator_std_array_int__2__ptr_ {}
pub unsafe fn operator_ne_30(
    x: *const random_access_iterator_std_array_int__3__ptr_,
    y: *const random_access_iterator_std_array_int__3__ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_std_array_int__3__ptr_ {
    it_: *mut Vec<i32>,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_std_array_int__3__ptr_ {
    pub unsafe fn new_1(mut it: *mut Vec<i32>) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut Vec<i32> {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_std_array_int__3__ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_std_array_int__3__ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_std_array_int__3__ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_std_array_int__3__ptr_,
    ) -> *mut random_access_iterator_std_array_int__3__ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_std_array_int__3__ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_std_array_int__3__ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_ne_30(
                self as *const random_access_iterator_std_array_int__3__ptr_,
                other as *const random_access_iterator_std_array_int__3__ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for random_access_iterator_std_array_int__3__ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_std_array_int__3__ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
impl UnsafeCppIterator<Vec<i32>> for random_access_iterator_std_array_int__3__ptr_ {
    fn iter_deref(&self) -> *mut Vec<i32> {
        unsafe {
            random_access_iterator_std_array_int__3__ptr_::operator_deref(self) as *mut Vec<i32>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            random_access_iterator_std_array_int__3__ptr_::operator_inc_7(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<Vec<i32>> for random_access_iterator_std_array_int__3__ptr_ {}
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
            !(operator_ne_31(
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
pub unsafe fn operator_ne_33(
    x: *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
    y: *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_34(
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
            !(operator_ne_33(
                self
                    as *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
                other
                    as *const contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
impl UnsafeCppIterator<Option<Box<i32>>>
    for contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
    fn iter_deref(&self) -> *mut Option<Box<i32>> {
        unsafe {
            contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_deref(
                self,
            ) as *mut Option<Box<i32>>
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_::operator_inc_9(
                self,
            );
        }
        self
    }
}
impl UnsafeCppInputIterator<Option<Box<i32>>>
    for contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_
{
}
pub unsafe fn base_35(mut i: *mut i32) -> *mut i32 {
    return i;
}
pub unsafe fn base_36(mut i: *mut Option<Box<i32>>) -> *mut Option<Box<i32>> {
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
pub unsafe fn operator_eq_37(
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
            operator_eq_37(
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
pub unsafe fn operator_add_38(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_39(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_38(_i, n)
    });
}
pub unsafe fn iter_swap_40(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_41(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_42(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_43(
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
        unsafe { operator_cmp_43(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_42(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestPtr_TestImpl_int_ptr_ {}
impl TestPtr_TestImpl_int_ptr_ {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: *mut i32 = unimplemented!();
            assert!(((unsafe { base_35(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: *mut i32 = unimplemented!();
            assert!(
                ((unsafe { base_35(r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_32(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_32(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_26(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_26(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_20(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_20(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_14(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_14(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
}
impl Callable0<()> for TestPtr_TestImpl_int_ptr_ {
    fn call(&self) -> () {
        unsafe {
            TestPtr_TestImpl_int_ptr_::operator_call_1_int_ptr(&mut *(&raw const *self).cast_mut())
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestPtr_TestImpl_contiguous_iterator_int_ptr__ {}
impl TestPtr_TestImpl_contiguous_iterator_int_ptr__ {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: *mut i32 = unimplemented!();
            assert!(((unsafe { base_35(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: *mut i32 = unimplemented!();
            assert!(
                ((unsafe { base_35(r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_32(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_32(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_26(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_26(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_20(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_20(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_14(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_14(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
}
impl Callable0<()> for TestPtr_TestImpl_contiguous_iterator_int_ptr__ {
    fn call(&self) -> () {
        unsafe {
            TestPtr_TestImpl_contiguous_iterator_int_ptr__::operator_call_1_int_ptr(
                &mut *(&raw const *self).cast_mut(),
            )
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestPtr_TestImpl_random_access_iterator_int_ptr__ {}
impl TestPtr_TestImpl_random_access_iterator_int_ptr__ {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: *mut i32 = unimplemented!();
            assert!(((unsafe { base_35(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: *mut i32 = unimplemented!();
            assert!(
                ((unsafe { base_35(r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_32(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_32(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_26(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_26(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_20(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_20(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_14(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_14(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
}
impl Callable0<()> for TestPtr_TestImpl_random_access_iterator_int_ptr__ {
    fn call(&self) -> () {
        unsafe {
            TestPtr_TestImpl_random_access_iterator_int_ptr__::operator_call_1_int_ptr(
                &mut *(&raw const *self).cast_mut(),
            )
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestPtr_TestImpl_bidirectional_iterator_int_ptr__ {}
impl TestPtr_TestImpl_bidirectional_iterator_int_ptr__ {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: *mut i32 = unimplemented!();
            assert!(((unsafe { base_35(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: *mut i32 = unimplemented!();
            assert!(
                ((unsafe { base_35(r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_32(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_32(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_26(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_26(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_20(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_20(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_14(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_14(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
}
impl Callable0<()> for TestPtr_TestImpl_bidirectional_iterator_int_ptr__ {
    fn call(&self) -> () {
        unsafe {
            TestPtr_TestImpl_bidirectional_iterator_int_ptr__::operator_call_1_int_ptr(
                &mut *(&raw const *self).cast_mut(),
            )
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestPtr_TestImpl_forward_iterator_int_ptr__ {}
impl TestPtr_TestImpl_forward_iterator_int_ptr__ {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: *mut i32 = unimplemented!();
            assert!(((unsafe { base_35(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: *mut i32 = unimplemented!();
            assert!(
                ((unsafe { base_35(r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_32(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: contiguous_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_32(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_26(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: random_access_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_26(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_20(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: bidirectional_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_20(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        {
            let mut a: [i32; 3] = [1, 2, 3];
            let mut b: [i32; 3] = [4, 5, 6];
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(((unsafe { base_14(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
            assert!(
                (((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)))
                    && ((a[(2) as usize]) == (6))
            );
            assert!(
                (((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)))
                    && ((b[(2) as usize]) == (3))
            );
        }
        {
            let N: i32 = 100;
            let mut a: Vec<i32> = std::array::from_fn::<_, 100, _>(|_| 0_i32).to_vec();
            let mut b: Vec<i32> = vec![42; (((N) + (2)) as usize) as usize];
            (*((&mut b).first_mut().unwrap())) = 1;
            (*((&mut b).last_mut().unwrap())) = -1_i32;
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                (*{
                    let __idx = (i as usize) as usize;
                    let __v: &mut Vec<i32> = &mut a;
                    __v.as_mut_ptr().add(__idx)
                }) = (((i) * (i)) + (1));
                i.prefix_inc();
            }
            let mut r: forward_iterator_int_ptr_ = unimplemented!();
            assert!(
                ((unsafe { base_14(&r,) })
                    == (b.as_mut_ptr().offset((N) as isize).offset((1) as isize)))
            );
            assert!(((*((&mut b).first_mut().unwrap())) == (1)));
            assert!(((*((&mut b).last_mut().unwrap())) == (-1_i32)));
            let mut i: i32 = 0;
            'loop_: while ((i) < (N)) {
                assert!(
                    ((*{
                        let __idx = (i as usize) as usize;
                        let __v: &mut Vec<i32> = &mut a;
                        __v.as_mut_ptr().add(__idx)
                    }) == (42))
                );
                assert!(
                    ((*(&mut b).as_mut_ptr().add((((i) + (1)) as usize) as usize))
                        == (((i) * (i)) + (1)))
                );
                i.prefix_inc();
            }
        }
    }
}
impl Callable0<()> for TestPtr_TestImpl_forward_iterator_int_ptr__ {
    fn call(&self) -> () {
        unsafe {
            TestPtr_TestImpl_forward_iterator_int_ptr__::operator_call_1_int_ptr(
                &mut *(&raw const *self).cast_mut(),
            )
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestPtr {}
impl TestPtr {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        (unsafe {
            for_each_2 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <TestPtr_TestImpl_int_ptr_ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        (unsafe {
            for_each_3 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <TestPtr_TestImpl_contiguous_iterator_int_ptr__ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        (unsafe {
            for_each_4 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <TestPtr_TestImpl_random_access_iterator_int_ptr__ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        (unsafe {
            for_each_5 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <TestPtr_TestImpl_bidirectional_iterator_int_ptr__ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        (unsafe {
            for_each_6 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <TestPtr_TestImpl_forward_iterator_int_ptr__ >::default() , )
        });
    }
}
impl Callable0<()> for TestPtr {
    fn call(&self) -> () {
        unsafe { TestPtr::operator_call_1_int_ptr(&mut *(&raw const *self).cast_mut()) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ {}
impl TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int__ptr(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: *mut Option<Box<i32>> = unimplemented!();
        assert!(((unsafe { base_36(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_34(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_28(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_22(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_16(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
}
impl Callable0<()> for TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ {
    fn call(&self) -> () {
        unsafe {
            TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_::operator_call_1_class_std_unique_ptr_int__ptr(&mut *(&raw const *self).cast_mut())
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{}
impl TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int__ptr(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: *mut Option<Box<i32>> = unimplemented!();
        assert!(((unsafe { base_36(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_34(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_28(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_22(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_16(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
}
impl Callable0<() > for TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ { fn call(&self) -> ()  { unsafe { TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__::operator_call_1_class_std_unique_ptr_int__ptr(&mut *(&raw const *self).cast_mut()) } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{}
impl
    TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{
    pub unsafe fn operator_call_1_class_std_unique_ptr_int__ptr(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: *mut Option<Box<i32>> = unimplemented!();
        assert!(((unsafe { base_36(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_34(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_28(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_22(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_16(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
}
impl Callable0<() > for TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ { fn call(&self) -> ()  { unsafe { TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__::operator_call_1_class_std_unique_ptr_int__ptr(&mut *(&raw const *self).cast_mut()) } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{}
impl
    TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{
    pub unsafe fn operator_call_1_class_std_unique_ptr_int__ptr(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: *mut Option<Box<i32>> = unimplemented!();
        assert!(((unsafe { base_36(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_34(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_28(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_22(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_16(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
}
impl Callable0<() > for TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ { fn call(&self) -> ()  { unsafe { TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__::operator_call_1_class_std_unique_ptr_int__ptr(&mut *(&raw const *self).cast_mut()) } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{}
impl TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int__ptr(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: *mut Option<Box<i32>> = unimplemented!();
        assert!(((unsafe { base_36(r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_34(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_28(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_22(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
    pub unsafe fn operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(&mut self) {
        let mut a: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (1)))) as *mut i32);
                a[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut b: [Option<Box<i32>>; 3] = std::array::from_fn::<_, 3, _>(|_| None);
        let mut k: i32 = 0;
        'loop_: while ((k) < (3)) {
            {
                let _a0: *mut i32 = (Box::leak(Box::new(((k) + (4)))) as *mut i32);
                b[(k) as usize] = if _a0.is_null() {
                    None
                } else {
                    Some(Box::from_raw(_a0))
                }
            };
            k.prefix_inc();
        }
        let mut r: forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr_ =
            unimplemented!();
        assert!(((unsafe { base_16(&r,) }) == (b.as_mut_ptr().offset((3) as isize))));
        assert!(
            (((*a[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
                && ((*a[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (5)))
                && ((*a[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (6))
        );
        assert!(
            (((*b[(0) as usize]
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
                && ((*b[(1) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (2)))
                && ((*b[(2) as usize]
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                    == (3))
        );
    }
}
impl Callable0<()>
    for TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__
{
    fn call(&self) -> () {
        unsafe {
            TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__::operator_call_1_class_std_unique_ptr_int__ptr(&mut *(&raw const *self).cast_mut())
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestUniquePtr {}
impl TestUniquePtr {
    pub unsafe fn operator_call_1_class_std_unique_ptr_int__ptr(&mut self) {
        (unsafe {
            for_each_8 ( <types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , <TestUniquePtr_TestImpl_std_unique_ptr_int__std_default_delete_int___ptr_ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        (unsafe {
            for_each_9 ( <types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , <TestUniquePtr_TestImpl_contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        (unsafe {
            for_each_10 ( <types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , <TestUniquePtr_TestImpl_random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_class_std_unique_ptr_int__ptr_(
        &mut self,
    ) {
        (unsafe {
            for_each_11 ( <types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , <TestUniquePtr_TestImpl_bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , )
        });
    }
    pub unsafe fn operator_call_1_class_forward_iterator_class_std_unique_ptr_int__ptr_(&mut self) {
        (unsafe {
            for_each_12 ( <types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , <TestUniquePtr_TestImpl_forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , )
        });
    }
}
impl Callable0<()> for TestUniquePtr {
    fn call(&self) -> () {
        unsafe {
            TestUniquePtr::operator_call_1_class_std_unique_ptr_int__ptr(
                &mut *(&raw const *self).cast_mut(),
            )
        }
    }
}
pub unsafe fn test_simple_cases_44() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_simple_cases_45() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_simple_cases_46() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_simple_cases_47() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_simple_cases_48() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_simple_cases_49() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_simple_cases_50() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_simple_cases_51() -> bool {
    {
        let mut a: [i32; 2] = [1, 2];
        let mut b: [i32; 2] = [4, 5];
        unimplemented!();
        assert!(((a[(0) as usize]) == (4)) && ((a[(1) as usize]) == (5)));
        assert!(((b[(0) as usize]) == (1)) && ((b[(1) as usize]) == (2)));
    }
    {
        let mut a: Vec<i32> = Vec::from([1, 2, 3]);
        let mut a0: Vec<i32> = a.clone();
        let mut b: Vec<i32> = Vec::from([4, 5, 6]);
        let mut b0: Vec<i32> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([2, 3])]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([Vec::from([9, 8]), Vec::from([7, 6])]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut a: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 1, 2]),
            Vec::from([3, 4, 5]),
            Vec::from([6, 7, 8]),
        ]);
        let mut a0: Vec<Vec<i32>> = a.clone();
        let mut b: Vec<Vec<i32>> = Vec::from([
            Vec::from([9, 8, 7]),
            Vec::from([6, 5, 4]),
            Vec::from([3, 2, 1]),
        ]);
        let mut b0: Vec<Vec<i32>> = b.clone();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_vector_bool_52() {
    let mut f: Vec<bool> = unimplemented!();
    let mut t: Vec<bool> = unimplemented!();
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
}
pub unsafe fn test_vector_bool_53() {
    let mut f: Vec<bool> = unimplemented!();
    let mut t: Vec<bool> = unimplemented!();
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
}
pub unsafe fn test_vector_bool_54() {
    let mut f: Vec<bool> = unimplemented!();
    let mut t: Vec<bool> = unimplemented!();
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
}
pub unsafe fn test_vector_bool_55() {
    let mut f: Vec<bool> = unimplemented!();
    let mut t: Vec<bool> = unimplemented!();
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
}
pub unsafe fn test_vector_bool_56() {
    let mut f: Vec<bool> = unimplemented!();
    let mut t: Vec<bool> = unimplemented!();
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
}
pub unsafe fn test_vector_bool_57() {
    let mut f: Vec<bool> = unimplemented!();
    let mut t: Vec<bool> = unimplemented!();
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
}
pub unsafe fn test_vector_bool_58() {
    let mut f: Vec<bool> = unimplemented!();
    let mut t: Vec<bool> = unimplemented!();
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut f1: Vec<bool> = unimplemented!();
        let mut t1: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
}
pub unsafe fn test_59() -> bool {
    (unsafe { test_simple_cases_44() });
    (unsafe { test_simple_cases_45() });
    (unsafe { test_simple_cases_46() });
    (unsafe { test_simple_cases_47() });
    (unsafe { test_simple_cases_48() });
    (unsafe { test_simple_cases_49() });
    (unsafe { test_simple_cases_50() });
    (unsafe { test_simple_cases_51() });
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr__ >::default() , <TestPtr >::default() , )
    });
    {
        (unsafe {
            for_each_7 ( <types_type_list_std_unique_ptr_int__std_default_delete_int___ptr__contiguous_iterator_std_unique_ptr_int__std_default_delete_int___ptr___random_access_iterator_std_unique_ptr_int__std_default_delete_int___ptr___bidirectional_iterator_std_unique_ptr_int__std_default_delete_int___ptr___forward_iterator_std_unique_ptr_int__std_default_delete_int___ptr__ >::default() , <TestUniquePtr >::default() , )
        });
    }
    {
        (unsafe { test_vector_bool_52() });
        (unsafe { test_vector_bool_53() });
        (unsafe { test_vector_bool_54() });
        (unsafe { test_vector_bool_55() });
        (unsafe { test_vector_bool_56() });
        (unsafe { test_vector_bool_57() });
        (unsafe { test_vector_bool_58() });
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
    (unsafe { test_59() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
