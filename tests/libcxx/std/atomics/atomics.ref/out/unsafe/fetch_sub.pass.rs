extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type LockFreeStatus = i32;
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
pub static mut value_0: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<libc::c_char>()) == (::std::mem::size_of::<libc::c_char>()))
        && ((::std::mem::align_of::<libc::c_char>()) >= (::std::mem::size_of::<libc::c_char>())))
    {
        2
    } else {
        (if (((::std::mem::size_of::<libc::c_char>()) == (::std::mem::size_of::<i16>()))
            && ((::std::mem::align_of::<libc::c_char>())
                >= (::std::mem::size_of::<libc::c_char>())))
        {
            2
        } else {
            (if (((::std::mem::size_of::<libc::c_char>()) == (::std::mem::size_of::<i32>()))
                && ((::std::mem::align_of::<libc::c_char>())
                    >= (::std::mem::size_of::<libc::c_char>())))
            {
                2
            } else {
                (if (((::std::mem::size_of::<libc::c_char>()) == (::std::mem::size_of::<i64>()))
                    && ((::std::mem::align_of::<libc::c_char>())
                        >= (::std::mem::size_of::<libc::c_char>())))
                {
                    2
                } else {
                    (if (((::std::mem::size_of::<libc::c_char>())
                        == (::std::mem::size_of::<i64>()))
                        && ((::std::mem::align_of::<libc::c_char>())
                            >= (::std::mem::size_of::<libc::c_char>())))
                    {
                        2
                    } else {
                        (if (((::std::mem::size_of::<libc::c_char>())
                            == (::std::mem::size_of::<*mut ::libc::c_void>()))
                            && ((::std::mem::align_of::<libc::c_char>())
                                >= (::std::mem::size_of::<libc::c_char>())))
                        {
                            2
                        } else {
                            -1_i32
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_1: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_0)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_char_ {}
pub static mut value_2: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<i16>()) == (::std::mem::size_of::<libc::c_char>()))
        && ((::std::mem::align_of::<i16>()) >= (::std::mem::size_of::<i16>())))
    {
        2
    } else {
        (if (((::std::mem::size_of::<i16>()) == (::std::mem::size_of::<i16>()))
            && ((::std::mem::align_of::<i16>()) >= (::std::mem::size_of::<i16>())))
        {
            2
        } else {
            (if (((::std::mem::size_of::<i16>()) == (::std::mem::size_of::<i32>()))
                && ((::std::mem::align_of::<i16>()) >= (::std::mem::size_of::<i16>())))
            {
                2
            } else {
                (if (((::std::mem::size_of::<i16>()) == (::std::mem::size_of::<i64>()))
                    && ((::std::mem::align_of::<i16>()) >= (::std::mem::size_of::<i16>())))
                {
                    2
                } else {
                    (if (((::std::mem::size_of::<i16>()) == (::std::mem::size_of::<i64>()))
                        && ((::std::mem::align_of::<i16>()) >= (::std::mem::size_of::<i16>())))
                    {
                        2
                    } else {
                        (if (((::std::mem::size_of::<i16>())
                            == (::std::mem::size_of::<*mut ::libc::c_void>()))
                            && ((::std::mem::align_of::<i16>()) >= (::std::mem::size_of::<i16>())))
                        {
                            2
                        } else {
                            -1_i32
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_3: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_2)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_short_ {}
pub static mut value_4: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<libc::c_char>()))
        && ((::std::mem::align_of::<i32>()) >= (::std::mem::size_of::<i32>())))
    {
        2
    } else {
        (if (((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i16>()))
            && ((::std::mem::align_of::<i32>()) >= (::std::mem::size_of::<i32>())))
        {
            2
        } else {
            (if (((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i32>()))
                && ((::std::mem::align_of::<i32>()) >= (::std::mem::size_of::<i32>())))
            {
                2
            } else {
                (if (((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i64>()))
                    && ((::std::mem::align_of::<i32>()) >= (::std::mem::size_of::<i32>())))
                {
                    2
                } else {
                    (if (((::std::mem::size_of::<i32>()) == (::std::mem::size_of::<i64>()))
                        && ((::std::mem::align_of::<i32>()) >= (::std::mem::size_of::<i32>())))
                    {
                        2
                    } else {
                        (if (((::std::mem::size_of::<i32>())
                            == (::std::mem::size_of::<*mut ::libc::c_void>()))
                            && ((::std::mem::align_of::<i32>()) >= (::std::mem::size_of::<i32>())))
                        {
                            2
                        } else {
                            -1_i32
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_5: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_4)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_int_ {}
pub static mut value_6: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<libc::c_char>()))
        && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
    {
        2
    } else {
        (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i16>()))
            && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
        {
            2
        } else {
            (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i32>()))
                && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
            {
                2
            } else {
                (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i64>()))
                    && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
                {
                    2
                } else {
                    (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i64>()))
                        && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
                    {
                        2
                    } else {
                        (if (((::std::mem::size_of::<i64>())
                            == (::std::mem::size_of::<*mut ::libc::c_void>()))
                            && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
                        {
                            2
                        } else {
                            -1_i32
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_7: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_6)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_long_ {}
pub static mut value_8: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<*mut ::libc::c_void>())
        == (::std::mem::size_of::<libc::c_char>()))
        && ((::std::mem::align_of::<*mut ::libc::c_void>())
            >= (::std::mem::size_of::<*mut ::libc::c_void>())))
    {
        2
    } else {
        (if (((::std::mem::size_of::<*mut ::libc::c_void>()) == (::std::mem::size_of::<i16>()))
            && ((::std::mem::align_of::<*mut ::libc::c_void>())
                >= (::std::mem::size_of::<*mut ::libc::c_void>())))
        {
            2
        } else {
            (if (((::std::mem::size_of::<*mut ::libc::c_void>()) == (::std::mem::size_of::<i32>()))
                && ((::std::mem::align_of::<*mut ::libc::c_void>())
                    >= (::std::mem::size_of::<*mut ::libc::c_void>())))
            {
                2
            } else {
                (if (((::std::mem::size_of::<*mut ::libc::c_void>())
                    == (::std::mem::size_of::<i64>()))
                    && ((::std::mem::align_of::<*mut ::libc::c_void>())
                        >= (::std::mem::size_of::<*mut ::libc::c_void>())))
                {
                    2
                } else {
                    (if (((::std::mem::size_of::<*mut ::libc::c_void>())
                        == (::std::mem::size_of::<i64>()))
                        && ((::std::mem::align_of::<*mut ::libc::c_void>())
                            >= (::std::mem::size_of::<*mut ::libc::c_void>())))
                    {
                        2
                    } else {
                        (if (((::std::mem::size_of::<*mut ::libc::c_void>())
                            == (::std::mem::size_of::<*mut ::libc::c_void>()))
                            && ((::std::mem::align_of::<*mut ::libc::c_void>())
                                >= (::std::mem::size_of::<*mut ::libc::c_void>())))
                        {
                            2
                        } else {
                            -1_i32
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_9: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_8)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_void_ptr_ {}
pub static mut value_10: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<libc::c_char>()))
            && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
        {
            2
        } else {
            (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i16>()))
                && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
            {
                2
            } else {
                (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i32>()))
                    && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
                {
                    2
                } else {
                    (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i64>()))
                        && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
                    {
                        2
                    } else {
                        (if (((::std::mem::size_of::<i64>()) == (::std::mem::size_of::<i64>()))
                            && ((::std::mem::align_of::<i64>()) >= (::std::mem::size_of::<i64>())))
                        {
                            2
                        } else {
                            (if (((::std::mem::size_of::<i64>())
                                == (::std::mem::size_of::<*mut ::libc::c_void>()))
                                && ((::std::mem::align_of::<i64>())
                                    >= (::std::mem::size_of::<i64>())))
                            {
                                2
                            } else {
                                -1_i32
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_11: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_10)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_long_long_ {}
const _: () = assert!(true, "LockFreeStatusInfo<char>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<short>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<int>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<long>::status_known");
const _: () = assert!(true, "LockFreeStatusInfo<void*>::status_known");
const _: () = assert!(
    true,
    "alignof(long long) == sizeof(long long) ? LockFreeStatusInfo<long long>::status_known : true"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<char>::value == LockFreeStatus::always"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<short>::value == LockFreeStatus::always"
);
const _: () = assert!(
    true,
    "LockFreeStatusInfo<int>::value == LockFreeStatus::always"
);
pub unsafe fn operator_eq_12(x: *const UserAtomicType, y: *const UserAtomicType) -> bool {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct UserAtomicType {
    pub i: i32,
}
impl UserAtomicType {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { i: d };
        this
    }
}
impl std::cmp::PartialEq for UserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const UserAtomicType,
                other as *const UserAtomicType,
            )
        }
    }
}
impl std::cmp::Eq for UserAtomicType {}
impl Default for UserAtomicType {
    fn default() -> Self {
        unsafe { UserAtomicType::new(None) }
    }
}
pub unsafe fn operator_eq_13(x: *const LargeUserAtomicType, y: *const LargeUserAtomicType) -> bool {
    let mut i: i32 = 0;
    'loop_: while ((i) < (128)) {
        if (((*x).a[(i) as usize]) != ((*y).a[(i) as usize])) {
            return false;
        }
        i.prefix_inc();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct LargeUserAtomicType {
    pub a: [i32; 128],
}
impl LargeUserAtomicType {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { a: [0_i32; 128] };
        'loop_: for e in IterRange::new(
            this.a.as_mut_ptr(),
            this.a.as_mut_ptr().offset((128_i64) as isize),
        ) {
            let e: *mut i32 = &mut (*e);
            (*e) = d.postfix_inc();
        }
        this
    }
}
impl std::cmp::PartialEq for LargeUserAtomicType {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
                self as *const LargeUserAtomicType,
                other as *const LargeUserAtomicType,
            )
        }
    }
}
impl std::cmp::Eq for LargeUserAtomicType {}
impl Default for LargeUserAtomicType {
    fn default() -> Self {
        unsafe { LargeUserAtomicType::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachIntegralType_TestFetchSub_ {}
impl TestEachIntegralType_TestFetchSub_ {
    pub unsafe fn operator_call(&self) {
        (unsafe { TestFetchSub_char_::operator_call(&<TestFetchSub_char_>::default()) });
        (unsafe {
            TestFetchSub_signed_char_::operator_call(&<TestFetchSub_signed_char_>::default())
        });
        (unsafe {
            TestFetchSub_unsigned_char_::operator_call(&<TestFetchSub_unsigned_char_>::default())
        });
        (unsafe { TestFetchSub_short_::operator_call(&<TestFetchSub_short_>::default()) });
        (unsafe {
            TestFetchSub_unsigned_short_::operator_call(&<TestFetchSub_unsigned_short_>::default())
        });
        (unsafe { TestFetchSub_int_::operator_call(&<TestFetchSub_int_>::default()) });
        (unsafe {
            TestFetchSub_unsigned_int_::operator_call(&<TestFetchSub_unsigned_int_>::default())
        });
        (unsafe { TestFetchSub_long_::operator_call(&<TestFetchSub_long_>::default()) });
        (unsafe {
            TestFetchSub_unsigned_long_::operator_call(&<TestFetchSub_unsigned_long_>::default())
        });
        (unsafe { TestFetchSub_long_long_::operator_call(&<TestFetchSub_long_long_>::default()) });
        (unsafe {
            TestFetchSub_unsigned_long_long_::operator_call(
                &<TestFetchSub_unsigned_long_long_>::default(),
            )
        });
        (unsafe { TestFetchSub_wchar_t_::operator_call(&<TestFetchSub_wchar_t_>::default()) });
        (unsafe { TestFetchSub_char8_t_::operator_call(&<TestFetchSub_char8_t_>::default()) });
        (unsafe { TestFetchSub_char16_t_::operator_call(&<TestFetchSub_char16_t_>::default()) });
        (unsafe { TestFetchSub_char32_t_::operator_call(&<TestFetchSub_char32_t_>::default()) });
        (unsafe {
            TestFetchSub_signed_char_::operator_call(&<TestFetchSub_signed_char_>::default())
        });
        (unsafe {
            TestFetchSub_unsigned_char_::operator_call(&<TestFetchSub_unsigned_char_>::default())
        });
        (unsafe { TestFetchSub_short_::operator_call(&<TestFetchSub_short_>::default()) });
        (unsafe {
            TestFetchSub_unsigned_short_::operator_call(&<TestFetchSub_unsigned_short_>::default())
        });
        (unsafe { TestFetchSub_int_::operator_call(&<TestFetchSub_int_>::default()) });
        (unsafe {
            TestFetchSub_unsigned_int_::operator_call(&<TestFetchSub_unsigned_int_>::default())
        });
        (unsafe { TestFetchSub_long_::operator_call(&<TestFetchSub_long_>::default()) });
        (unsafe {
            TestFetchSub_unsigned_long_::operator_call(&<TestFetchSub_unsigned_long_>::default())
        });
    }
}
impl Callable0<()> for TestEachIntegralType_TestFetchSub_ {
    fn call(&self) -> () {
        unsafe { TestEachIntegralType_TestFetchSub_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachPointerType_TestFetchSub_ {}
impl TestEachPointerType_TestFetchSub_ {
    pub unsafe fn operator_call(&self) {
        (unsafe { TestFetchSub_int_ptr_::operator_call(&<TestFetchSub_int_ptr_>::default()) });
        (unsafe {
            TestFetchSub_const_int_ptr_::operator_call(&<TestFetchSub_const_int_ptr_>::default())
        });
    }
}
impl Callable0<()> for TestEachPointerType_TestFetchSub_ {
    fn call(&self) -> () {
        unsafe { TestEachPointerType_TestFetchSub_::operator_call(self) }
    }
}
pub unsafe fn test_seq_cst_14(
    mut store_op: FnPtr<fn(*const (), libc::c_char, libc::c_char)>,
    mut load_op: FnPtr<fn(*const ()) -> libc::c_char>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_15(
    mut store_op: FnPtr<fn(*const (), libc::c_char, libc::c_char)>,
    mut load_op: FnPtr<fn(*const ()) -> libc::c_char>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_16(
    mut store_op: FnPtr<fn(*const (), i8, i8)>,
    mut load_op: FnPtr<fn(*const ()) -> i8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_17(
    mut store_op: FnPtr<fn(*const (), i8, i8)>,
    mut load_op: FnPtr<fn(*const ()) -> i8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_18(
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_19(
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_20(
    mut store_op: FnPtr<fn(*const (), i16, i16)>,
    mut load_op: FnPtr<fn(*const ()) -> i16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_21(
    mut store_op: FnPtr<fn(*const (), i16, i16)>,
    mut load_op: FnPtr<fn(*const ()) -> i16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_22(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_23(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_24(
    mut store_op: FnPtr<fn(*const (), i32, i32)>,
    mut load_op: FnPtr<fn(*const ()) -> i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_25(
    mut store_op: FnPtr<fn(*const (), i32, i32)>,
    mut load_op: FnPtr<fn(*const ()) -> i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_26(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_27(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_28(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_29(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_30(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_31(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_32(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_33(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_34(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_35(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_36(
    mut store_op: FnPtr<fn(*const (), i32, i32)>,
    mut load_op: FnPtr<fn(*const ()) -> i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_37(
    mut store_op: FnPtr<fn(*const (), i32, i32)>,
    mut load_op: FnPtr<fn(*const ()) -> i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_38(
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_39(
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_40(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_41(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_42(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_43(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_44(
    mut store_op: FnPtr<fn(*const (), f32, f32)>,
    mut load_op: FnPtr<fn(*const ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_45(
    mut store_op: FnPtr<fn(*const (), f32, f32)>,
    mut load_op: FnPtr<fn(*const ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_46(
    mut store_op: FnPtr<fn(*const (), f64, f64)>,
    mut load_op: FnPtr<fn(*const ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_47(
    mut store_op: FnPtr<fn(*const (), f64, f64)>,
    mut load_op: FnPtr<fn(*const ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_48(
    mut store_op: FnPtr<fn(*const (), *mut i32, *mut i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *mut i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_49(
    mut store_op: FnPtr<fn(*const (), *mut i32, *mut i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *mut i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_50(
    mut store_op: FnPtr<fn(*const (), *const i32, *const i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *const i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_51(
    mut store_op: FnPtr<fn(*const (), *const i32, *const i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *const i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_52(
    mut store_op: FnPtr<fn(*const (), libc::c_char, libc::c_char)>,
    mut load_op: FnPtr<fn(*const ()) -> libc::c_char>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_53(
    mut store_op: FnPtr<fn(*const (), i8, i8)>,
    mut load_op: FnPtr<fn(*const ()) -> i8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_54(
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_55(
    mut store_op: FnPtr<fn(*const (), i16, i16)>,
    mut load_op: FnPtr<fn(*const ()) -> i16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_56(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_57(
    mut store_op: FnPtr<fn(*const (), i32, i32)>,
    mut load_op: FnPtr<fn(*const ()) -> i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_58(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_59(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_60(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_61(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_62(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_63(
    mut store_op: FnPtr<fn(*const (), i32, i32)>,
    mut load_op: FnPtr<fn(*const ()) -> i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_64(
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_65(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_66(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_67(
    mut store_op: FnPtr<fn(*const (), f32, f32)>,
    mut load_op: FnPtr<fn(*const ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_68(
    mut store_op: FnPtr<fn(*const (), f64, f64)>,
    mut load_op: FnPtr<fn(*const ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_69(
    mut store_op: FnPtr<fn(*const (), *mut i32, *mut i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *mut i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_70(
    mut store_op: FnPtr<fn(*const (), *const i32, *const i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *const i32>,
) {
    &(store_op);
    &(load_op);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchSub_bool_ {}
impl TestDoesNotHaveFetchSub_bool_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_sub<std::atomic_ref<T>>");;
    }
}
impl Callable0<()> for TestDoesNotHaveFetchSub_bool_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveFetchSub_bool_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchSub_UserAtomicType_ {}
impl TestDoesNotHaveFetchSub_UserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_sub<std::atomic_ref<T>>");;
    }
}
impl Callable0<()> for TestDoesNotHaveFetchSub_UserAtomicType_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveFetchSub_UserAtomicType_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchSub_LargeUserAtomicType_ {}
impl TestDoesNotHaveFetchSub_LargeUserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_sub<std::atomic_ref<T>>");;
    }
}
impl Callable0<()> for TestDoesNotHaveFetchSub_LargeUserAtomicType_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveFetchSub_LargeUserAtomicType_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_float_ {}
impl TestFetchSub_float_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: f32 = 7_f32;
            let a: () = unimplemented!();
            {
                let mut y: f32 = unimplemented!();
                assert!(((y) == (7_f32)));
                assert!(((x) == (3_f32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: f32 = unimplemented!();
                assert!(((y) == (3_f32)));
                assert!(((x) == (1_f32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), f32, f32)> =
                FnPtr::<fn(*const (), f32, f32)>::new(
                    |x: *const (), mut old_val: f32, mut new_val: f32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> f32> =
                FnPtr::<fn(*const ()) -> f32>::new(|x: *const ()| -> f32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_67(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), f32, f32)> =
                FnPtr::<fn(*const (), f32, f32)>::new(
                    |x: *const (), mut old_val: f32, mut new_val: f32| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), f32, f32)> =
                FnPtr::<fn(*const (), f32, f32)>::new(
                    |x: *const (), mut old_val: f32, mut new_val: f32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> f32> =
                FnPtr::<fn(*const ()) -> f32>::new(|x: *const ()| -> f32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_44(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_45(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_float_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_float_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_double_ {}
impl TestFetchSub_double_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: f64 = 7_f64;
            let a: () = unimplemented!();
            {
                let mut y: f64 = unimplemented!();
                assert!(((y) == (7_f64)));
                assert!(((x) == (3_f64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: f64 = unimplemented!();
                assert!(((y) == (3_f64)));
                assert!(((x) == (1_f64)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), f64, f64)> =
                FnPtr::<fn(*const (), f64, f64)>::new(
                    |x: *const (), mut old_val: f64, mut new_val: f64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> f64> =
                FnPtr::<fn(*const ()) -> f64>::new(|x: *const ()| -> f64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_68(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), f64, f64)> =
                FnPtr::<fn(*const (), f64, f64)>::new(
                    |x: *const (), mut old_val: f64, mut new_val: f64| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), f64, f64)> =
                FnPtr::<fn(*const (), f64, f64)>::new(
                    |x: *const (), mut old_val: f64, mut new_val: f64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> f64> =
                FnPtr::<fn(*const ()) -> f64>::new(|x: *const ()| -> f64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_46(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_47(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_double_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_double_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_char_ {}
impl TestFetchSub_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: libc::c_char = (7 as libc::c_char);
            let a: () = unimplemented!();
            {
                let mut y: libc::c_char = unimplemented!();
                assert!(((y as i32) == ((7 as libc::c_char) as i32)));
                assert!(((x as i32) == ((3 as libc::c_char) as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: libc::c_char = unimplemented!();
                assert!(((y as i32) == ((3 as libc::c_char) as i32)));
                assert!(((x as i32) == ((1 as libc::c_char) as i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), libc::c_char, libc::c_char)> =
                FnPtr::<fn(*const (), libc::c_char, libc::c_char)>::new(
                    |x: *const (), mut old_val: libc::c_char, mut new_val: libc::c_char| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> libc::c_char> =
                FnPtr::<fn(*const ()) -> libc::c_char>::new(|x: *const ()| -> libc::c_char {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_52(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), libc::c_char, libc::c_char)> =
                FnPtr::<fn(*const (), libc::c_char, libc::c_char)>::new(
                    |x: *const (), mut old_val: libc::c_char, mut new_val: libc::c_char| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), libc::c_char, libc::c_char)> =
                FnPtr::<fn(*const (), libc::c_char, libc::c_char)>::new(
                    |x: *const (), mut old_val: libc::c_char, mut new_val: libc::c_char| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> libc::c_char> =
                FnPtr::<fn(*const ()) -> libc::c_char>::new(|x: *const ()| -> libc::c_char {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_14(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_15(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_char_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_char_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_signed_char_ {}
impl TestFetchSub_signed_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i8 = 7_i8;
            let a: () = unimplemented!();
            {
                let mut y: i8 = unimplemented!();
                assert!(((y as i32) == (7_i8 as i32)));
                assert!(((x as i32) == (3_i8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i8 = unimplemented!();
                assert!(((y as i32) == (3_i8 as i32)));
                assert!(((x as i32) == (1_i8 as i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), i8, i8)> = FnPtr::<fn(*const (), i8, i8)>::new(
                |x: *const (), mut old_val: i8, mut new_val: i8| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> i8> =
                FnPtr::<fn(*const ()) -> i8>::new(|x: *const ()| -> i8 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_53(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), i8, i8)> =
                FnPtr::<fn(*const (), i8, i8)>::new(
                    |x: *const (), mut old_val: i8, mut new_val: i8| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), i8, i8)> =
                FnPtr::<fn(*const (), i8, i8)>::new(
                    |x: *const (), mut old_val: i8, mut new_val: i8| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i8> =
                FnPtr::<fn(*const ()) -> i8>::new(|x: *const ()| -> i8 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_16(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_17(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_signed_char_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_signed_char_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_unsigned_char_ {}
impl TestFetchSub_unsigned_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u8 = 7_u8;
            let a: () = unimplemented!();
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (7_u8 as i32)));
                assert!(((x as i32) == (3_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (3_u8 as i32)));
                assert!(((x as i32) == (1_u8 as i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u8, u8)> = FnPtr::<fn(*const (), u8, u8)>::new(
                |x: *const (), mut old_val: u8, mut new_val: u8| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> u8> =
                FnPtr::<fn(*const ()) -> u8>::new(|x: *const ()| -> u8 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_54(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u8, u8)> =
                FnPtr::<fn(*const (), u8, u8)>::new(
                    |x: *const (), mut old_val: u8, mut new_val: u8| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u8, u8)> =
                FnPtr::<fn(*const (), u8, u8)>::new(
                    |x: *const (), mut old_val: u8, mut new_val: u8| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u8> =
                FnPtr::<fn(*const ()) -> u8>::new(|x: *const ()| -> u8 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_18(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_19(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_unsigned_char_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_unsigned_char_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_short_ {}
impl TestFetchSub_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i16 = 7_i16;
            let a: () = unimplemented!();
            {
                let mut y: i16 = unimplemented!();
                assert!(((y as i32) == (7_i16 as i32)));
                assert!(((x as i32) == (3_i16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i16 = unimplemented!();
                assert!(((y as i32) == (3_i16 as i32)));
                assert!(((x as i32) == (1_i16 as i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), i16, i16)> =
                FnPtr::<fn(*const (), i16, i16)>::new(
                    |x: *const (), mut old_val: i16, mut new_val: i16| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i16> =
                FnPtr::<fn(*const ()) -> i16>::new(|x: *const ()| -> i16 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_55(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), i16, i16)> =
                FnPtr::<fn(*const (), i16, i16)>::new(
                    |x: *const (), mut old_val: i16, mut new_val: i16| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), i16, i16)> =
                FnPtr::<fn(*const (), i16, i16)>::new(
                    |x: *const (), mut old_val: i16, mut new_val: i16| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i16> =
                FnPtr::<fn(*const ()) -> i16>::new(|x: *const ()| -> i16 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_20(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_21(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_short_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_short_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_unsigned_short_ {}
impl TestFetchSub_unsigned_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u16 = 7_u16;
            let a: () = unimplemented!();
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (7_u16 as i32)));
                assert!(((x as i32) == (3_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (3_u16 as i32)));
                assert!(((x as i32) == (1_u16 as i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u16, u16)> =
                FnPtr::<fn(*const (), u16, u16)>::new(
                    |x: *const (), mut old_val: u16, mut new_val: u16| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u16> =
                FnPtr::<fn(*const ()) -> u16>::new(|x: *const ()| -> u16 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_56(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u16, u16)> =
                FnPtr::<fn(*const (), u16, u16)>::new(
                    |x: *const (), mut old_val: u16, mut new_val: u16| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u16, u16)> =
                FnPtr::<fn(*const (), u16, u16)>::new(
                    |x: *const (), mut old_val: u16, mut new_val: u16| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u16> =
                FnPtr::<fn(*const ()) -> u16>::new(|x: *const ()| -> u16 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_22(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_23(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_unsigned_short_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_unsigned_short_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_int_ {}
impl TestFetchSub_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i32 = 7;
            let a: () = unimplemented!();
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (7)));
                assert!(((x) == (3)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (3)));
                assert!(((x) == (1)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), i32, i32)> =
                FnPtr::<fn(*const (), i32, i32)>::new(
                    |x: *const (), mut old_val: i32, mut new_val: i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i32> =
                FnPtr::<fn(*const ()) -> i32>::new(|x: *const ()| -> i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_57(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), i32, i32)> =
                FnPtr::<fn(*const (), i32, i32)>::new(
                    |x: *const (), mut old_val: i32, mut new_val: i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), i32, i32)> =
                FnPtr::<fn(*const (), i32, i32)>::new(
                    |x: *const (), mut old_val: i32, mut new_val: i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i32> =
                FnPtr::<fn(*const ()) -> i32>::new(|x: *const ()| -> i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_24(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_25(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_int_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_int_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_unsigned_int_ {}
impl TestFetchSub_unsigned_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u32 = 7_u32;
            let a: () = unimplemented!();
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (7_u32)));
                assert!(((x) == (3_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (3_u32)));
                assert!(((x) == (1_u32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u32, u32)> =
                FnPtr::<fn(*const (), u32, u32)>::new(
                    |x: *const (), mut old_val: u32, mut new_val: u32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u32> =
                FnPtr::<fn(*const ()) -> u32>::new(|x: *const ()| -> u32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_58(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u32, u32)> =
                FnPtr::<fn(*const (), u32, u32)>::new(
                    |x: *const (), mut old_val: u32, mut new_val: u32| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u32, u32)> =
                FnPtr::<fn(*const (), u32, u32)>::new(
                    |x: *const (), mut old_val: u32, mut new_val: u32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u32> =
                FnPtr::<fn(*const ()) -> u32>::new(|x: *const ()| -> u32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_26(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_27(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_unsigned_int_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_unsigned_int_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_long_ {}
impl TestFetchSub_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i64 = 7_i64;
            let a: () = unimplemented!();
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (7_i64)));
                assert!(((x) == (3_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (3_i64)));
                assert!(((x) == (1_i64)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), i64, i64)> =
                FnPtr::<fn(*const (), i64, i64)>::new(
                    |x: *const (), mut old_val: i64, mut new_val: i64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i64> =
                FnPtr::<fn(*const ()) -> i64>::new(|x: *const ()| -> i64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_59(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), i64, i64)> =
                FnPtr::<fn(*const (), i64, i64)>::new(
                    |x: *const (), mut old_val: i64, mut new_val: i64| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), i64, i64)> =
                FnPtr::<fn(*const (), i64, i64)>::new(
                    |x: *const (), mut old_val: i64, mut new_val: i64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i64> =
                FnPtr::<fn(*const ()) -> i64>::new(|x: *const ()| -> i64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_28(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_29(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_long_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_unsigned_long_ {}
impl TestFetchSub_unsigned_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u64 = 7_u64;
            let a: () = unimplemented!();
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (7_u64)));
                assert!(((x) == (3_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (3_u64)));
                assert!(((x) == (1_u64)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u64, u64)> =
                FnPtr::<fn(*const (), u64, u64)>::new(
                    |x: *const (), mut old_val: u64, mut new_val: u64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u64> =
                FnPtr::<fn(*const ()) -> u64>::new(|x: *const ()| -> u64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_60(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u64, u64)> =
                FnPtr::<fn(*const (), u64, u64)>::new(
                    |x: *const (), mut old_val: u64, mut new_val: u64| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u64, u64)> =
                FnPtr::<fn(*const (), u64, u64)>::new(
                    |x: *const (), mut old_val: u64, mut new_val: u64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u64> =
                FnPtr::<fn(*const ()) -> u64>::new(|x: *const ()| -> u64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_30(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_31(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_unsigned_long_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_unsigned_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_long_long_ {}
impl TestFetchSub_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i64 = 7_i64;
            let a: () = unimplemented!();
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (7_i64)));
                assert!(((x) == (3_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (3_i64)));
                assert!(((x) == (1_i64)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), i64, i64)> =
                FnPtr::<fn(*const (), i64, i64)>::new(
                    |x: *const (), mut old_val: i64, mut new_val: i64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i64> =
                FnPtr::<fn(*const ()) -> i64>::new(|x: *const ()| -> i64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_61(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), i64, i64)> =
                FnPtr::<fn(*const (), i64, i64)>::new(
                    |x: *const (), mut old_val: i64, mut new_val: i64| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), i64, i64)> =
                FnPtr::<fn(*const (), i64, i64)>::new(
                    |x: *const (), mut old_val: i64, mut new_val: i64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i64> =
                FnPtr::<fn(*const ()) -> i64>::new(|x: *const ()| -> i64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_32(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_33(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_long_long_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_long_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_unsigned_long_long_ {}
impl TestFetchSub_unsigned_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u64 = 7_u64;
            let a: () = unimplemented!();
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (7_u64)));
                assert!(((x) == (3_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (3_u64)));
                assert!(((x) == (1_u64)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u64, u64)> =
                FnPtr::<fn(*const (), u64, u64)>::new(
                    |x: *const (), mut old_val: u64, mut new_val: u64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u64> =
                FnPtr::<fn(*const ()) -> u64>::new(|x: *const ()| -> u64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_62(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u64, u64)> =
                FnPtr::<fn(*const (), u64, u64)>::new(
                    |x: *const (), mut old_val: u64, mut new_val: u64| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u64, u64)> =
                FnPtr::<fn(*const (), u64, u64)>::new(
                    |x: *const (), mut old_val: u64, mut new_val: u64| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u64> =
                FnPtr::<fn(*const ()) -> u64>::new(|x: *const ()| -> u64 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_34(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_35(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_unsigned_long_long_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_unsigned_long_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_wchar_t_ {}
impl TestFetchSub_wchar_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i32 = 7_i32;
            let a: () = unimplemented!();
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (7_i32)));
                assert!(((x) == (3_i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (3_i32)));
                assert!(((x) == (1_i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), i32, i32)> =
                FnPtr::<fn(*const (), i32, i32)>::new(
                    |x: *const (), mut old_val: i32, mut new_val: i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i32> =
                FnPtr::<fn(*const ()) -> i32>::new(|x: *const ()| -> i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_63(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), i32, i32)> =
                FnPtr::<fn(*const (), i32, i32)>::new(
                    |x: *const (), mut old_val: i32, mut new_val: i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), i32, i32)> =
                FnPtr::<fn(*const (), i32, i32)>::new(
                    |x: *const (), mut old_val: i32, mut new_val: i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> i32> =
                FnPtr::<fn(*const ()) -> i32>::new(|x: *const ()| -> i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_36(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_37(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_wchar_t_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_wchar_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_char8_t_ {}
impl TestFetchSub_char8_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u8 = 7_u8;
            let a: () = unimplemented!();
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (7_u8 as i32)));
                assert!(((x as i32) == (3_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (3_u8 as i32)));
                assert!(((x as i32) == (1_u8 as i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u8, u8)> = FnPtr::<fn(*const (), u8, u8)>::new(
                |x: *const (), mut old_val: u8, mut new_val: u8| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> u8> =
                FnPtr::<fn(*const ()) -> u8>::new(|x: *const ()| -> u8 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_64(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u8, u8)> =
                FnPtr::<fn(*const (), u8, u8)>::new(
                    |x: *const (), mut old_val: u8, mut new_val: u8| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u8, u8)> =
                FnPtr::<fn(*const (), u8, u8)>::new(
                    |x: *const (), mut old_val: u8, mut new_val: u8| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u8> =
                FnPtr::<fn(*const ()) -> u8>::new(|x: *const ()| -> u8 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_38(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_39(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_char8_t_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_char8_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_char16_t_ {}
impl TestFetchSub_char16_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u16 = 7_u16;
            let a: () = unimplemented!();
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (7_u16 as i32)));
                assert!(((x as i32) == (3_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (3_u16 as i32)));
                assert!(((x as i32) == (1_u16 as i32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u16, u16)> =
                FnPtr::<fn(*const (), u16, u16)>::new(
                    |x: *const (), mut old_val: u16, mut new_val: u16| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u16> =
                FnPtr::<fn(*const ()) -> u16>::new(|x: *const ()| -> u16 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_65(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u16, u16)> =
                FnPtr::<fn(*const (), u16, u16)>::new(
                    |x: *const (), mut old_val: u16, mut new_val: u16| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u16, u16)> =
                FnPtr::<fn(*const (), u16, u16)>::new(
                    |x: *const (), mut old_val: u16, mut new_val: u16| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u16> =
                FnPtr::<fn(*const ()) -> u16>::new(|x: *const ()| -> u16 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_40(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_41(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_char16_t_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_char16_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_char32_t_ {}
impl TestFetchSub_char32_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u32 = 7_u32;
            let a: () = unimplemented!();
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (7_u32)));
                assert!(((x) == (3_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (3_u32)));
                assert!(((x) == (1_u32)));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), u32, u32)> =
                FnPtr::<fn(*const (), u32, u32)>::new(
                    |x: *const (), mut old_val: u32, mut new_val: u32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u32> =
                FnPtr::<fn(*const ()) -> u32>::new(|x: *const ()| -> u32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_66(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), u32, u32)> =
                FnPtr::<fn(*const (), u32, u32)>::new(
                    |x: *const (), mut old_val: u32, mut new_val: u32| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), u32, u32)> =
                FnPtr::<fn(*const (), u32, u32)>::new(
                    |x: *const (), mut old_val: u32, mut new_val: u32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> u32> =
                FnPtr::<fn(*const ()) -> u32>::new(|x: *const ()| -> u32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_42(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_43(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_char32_t_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_char32_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_int_ptr_ {}
impl TestFetchSub_int_ptr_ {
    pub unsafe fn operator_call(&self) {
        {
            {
                let mut t: [(); 9] = [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ];
                let mut p: *mut i32 = (&mut t[(7) as usize] as *mut ());
                let a: () = unimplemented!();
                {
                    let mut y: *mut i32 = unimplemented!();
                    assert!(((y) == (&mut t[(7) as usize] as *mut ())));
                    assert!(((unimplemented!()) == (&mut t[(3) as usize] as *mut ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *mut i32 = unimplemented!();
                    assert!(((y) == (&mut t[(3) as usize] as *mut ())));
                    assert!(((unimplemented!()) == (&mut t[(1) as usize] as *mut ())));
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), *mut i32, *mut i32)> =
                FnPtr::<fn(*const (), *mut i32, *mut i32)>::new(
                    |x: *const (), mut old_val: *mut i32, mut new_val: *mut i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *mut i32> =
                FnPtr::<fn(*const ()) -> *mut i32>::new(|x: *const ()| -> *mut i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_69(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), *mut i32, *mut i32)> =
                FnPtr::<fn(*const (), *mut i32, *mut i32)>::new(
                    |x: *const (), mut old_val: *mut i32, mut new_val: *mut i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), *mut i32, *mut i32)> =
                FnPtr::<fn(*const (), *mut i32, *mut i32)>::new(
                    |x: *const (), mut old_val: *mut i32, mut new_val: *mut i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *mut i32> =
                FnPtr::<fn(*const ()) -> *mut i32>::new(|x: *const ()| -> *mut i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_48(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_49(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_int_ptr_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_int_ptr_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchSub_const_int_ptr_ {}
impl TestFetchSub_const_int_ptr_ {
    pub unsafe fn operator_call(&self) {
        {
            {
                let t: [(); 9] = [
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ];
                let mut p: *const i32 = (&t[(7) as usize] as *const ());
                let a: () = unimplemented!();
                {
                    let mut y: *const i32 = unimplemented!();
                    assert!(((y) == (&t[(7) as usize] as *const ())));
                    assert!(((unimplemented!()) == (&t[(3) as usize] as *const ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *const i32 = unimplemented!();
                    assert!(((y) == (&t[(3) as usize] as *const ())));
                    assert!(((unimplemented!()) == (&t[(1) as usize] as *const ())));
                    const _: () = assert!(true, "");;
                }
            }
        }
        {
            let mut fetch_sub: FnPtr<fn(*const (), *const i32, *const i32)> =
                FnPtr::<fn(*const (), *const i32, *const i32)>::new(
                    |x: *const (), mut old_val: *const i32, mut new_val: *const i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *const i32> =
                FnPtr::<fn(*const ()) -> *const i32>::new(|x: *const ()| -> *const i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_70(fetch_sub.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_sub_no_arg: FnPtr<fn(*const (), *const i32, *const i32)> =
                FnPtr::<fn(*const (), *const i32, *const i32)>::new(
                    |x: *const (), mut old_val: *const i32, mut new_val: *const i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_sub_with_order: FnPtr<fn(*const (), *const i32, *const i32)> =
                FnPtr::<fn(*const (), *const i32, *const i32)>::new(
                    |x: *const (), mut old_val: *const i32, mut new_val: *const i32| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *const i32> =
                FnPtr::<fn(*const ()) -> *const i32>::new(|x: *const ()| -> *const i32 {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_50(fetch_sub_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_51(fetch_sub_with_order.copy_from(), load.copy_from()) });
        }
    }
}
impl Callable0<()> for TestFetchSub_const_int_ptr_ {
    fn call(&self) -> () {
        unsafe { TestFetchSub_const_int_ptr_::operator_call(self) }
    }
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
    (unsafe {
        TestEachIntegralType_TestFetchSub_::operator_call(
            &<TestEachIntegralType_TestFetchSub_>::default(),
        )
    });
    (unsafe { TestFetchSub_float_::operator_call(&<TestFetchSub_float_>::default()) });
    (unsafe { TestFetchSub_double_::operator_call(&<TestFetchSub_double_>::default()) });
    (unsafe {
        TestEachPointerType_TestFetchSub_::operator_call(
            &<TestEachPointerType_TestFetchSub_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveFetchSub_bool_::operator_call(&<TestDoesNotHaveFetchSub_bool_>::default())
    });
    (unsafe {
        TestDoesNotHaveFetchSub_UserAtomicType_::operator_call(
            &<TestDoesNotHaveFetchSub_UserAtomicType_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveFetchSub_LargeUserAtomicType_::operator_call(
            &<TestDoesNotHaveFetchSub_LargeUserAtomicType_>::default(),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const value_0);
    std::cell::LazyCell::force(&*&raw const status_known_1);
    std::cell::LazyCell::force(&*&raw const value_2);
    std::cell::LazyCell::force(&*&raw const status_known_3);
    std::cell::LazyCell::force(&*&raw const value_4);
    std::cell::LazyCell::force(&*&raw const status_known_5);
    std::cell::LazyCell::force(&*&raw const value_6);
    std::cell::LazyCell::force(&*&raw const status_known_7);
    std::cell::LazyCell::force(&*&raw const value_8);
    std::cell::LazyCell::force(&*&raw const status_known_9);
    std::cell::LazyCell::force(&*&raw const value_10);
    std::cell::LazyCell::force(&*&raw const status_known_11);
}
