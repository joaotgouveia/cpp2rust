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
pub struct TestEachIntegralType_TestOperatorPlusEquals_ {}
impl TestEachIntegralType_TestOperatorPlusEquals_ {
    pub unsafe fn operator_call(&self) {
        (unsafe {
            TestOperatorPlusEquals_char_::operator_call(&<TestOperatorPlusEquals_char_>::default())
        });
        (unsafe {
            TestOperatorPlusEquals_signed_char_::operator_call(
                &<TestOperatorPlusEquals_signed_char_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_char_::operator_call(
                &<TestOperatorPlusEquals_unsigned_char_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_short_::operator_call(&<TestOperatorPlusEquals_short_>::default())
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_short_::operator_call(
                &<TestOperatorPlusEquals_unsigned_short_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_int_::operator_call(&<TestOperatorPlusEquals_int_>::default())
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_int_::operator_call(
                &<TestOperatorPlusEquals_unsigned_int_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_long_::operator_call(&<TestOperatorPlusEquals_long_>::default())
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_long_::operator_call(
                &<TestOperatorPlusEquals_unsigned_long_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_long_long_::operator_call(
                &<TestOperatorPlusEquals_long_long_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_long_long_::operator_call(
                &<TestOperatorPlusEquals_unsigned_long_long_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_wchar_t_::operator_call(
                &<TestOperatorPlusEquals_wchar_t_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_char8_t_::operator_call(
                &<TestOperatorPlusEquals_char8_t_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_char16_t_::operator_call(
                &<TestOperatorPlusEquals_char16_t_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_char32_t_::operator_call(
                &<TestOperatorPlusEquals_char32_t_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_signed_char_::operator_call(
                &<TestOperatorPlusEquals_signed_char_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_char_::operator_call(
                &<TestOperatorPlusEquals_unsigned_char_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_short_::operator_call(&<TestOperatorPlusEquals_short_>::default())
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_short_::operator_call(
                &<TestOperatorPlusEquals_unsigned_short_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_int_::operator_call(&<TestOperatorPlusEquals_int_>::default())
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_int_::operator_call(
                &<TestOperatorPlusEquals_unsigned_int_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_long_::operator_call(&<TestOperatorPlusEquals_long_>::default())
        });
        (unsafe {
            TestOperatorPlusEquals_unsigned_long_::operator_call(
                &<TestOperatorPlusEquals_unsigned_long_>::default(),
            )
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachPointerType_TestOperatorPlusEquals_ {}
impl TestEachPointerType_TestOperatorPlusEquals_ {
    pub unsafe fn operator_call(&self) {
        (unsafe {
            TestOperatorPlusEquals_int_ptr_::operator_call(
                &<TestOperatorPlusEquals_int_ptr_>::default(),
            )
        });
        (unsafe {
            TestOperatorPlusEquals_const_int_ptr_::operator_call(
                &<TestOperatorPlusEquals_const_int_ptr_>::default(),
            )
        });
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
    mut store_op: FnPtr<fn(*const (), i8, i8)>,
    mut load_op: FnPtr<fn(*const ()) -> i8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_16(
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_17(
    mut store_op: FnPtr<fn(*const (), i16, i16)>,
    mut load_op: FnPtr<fn(*const ()) -> i16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_18(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_19(
    mut store_op: FnPtr<fn(*const (), i32, i32)>,
    mut load_op: FnPtr<fn(*const ()) -> i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_20(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_21(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_22(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_23(
    mut store_op: FnPtr<fn(*const (), i64, i64)>,
    mut load_op: FnPtr<fn(*const ()) -> i64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_24(
    mut store_op: FnPtr<fn(*const (), u64, u64)>,
    mut load_op: FnPtr<fn(*const ()) -> u64>,
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
    mut store_op: FnPtr<fn(*const (), u8, u8)>,
    mut load_op: FnPtr<fn(*const ()) -> u8>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_27(
    mut store_op: FnPtr<fn(*const (), u16, u16)>,
    mut load_op: FnPtr<fn(*const ()) -> u16>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_28(
    mut store_op: FnPtr<fn(*const (), u32, u32)>,
    mut load_op: FnPtr<fn(*const ()) -> u32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_29(
    mut store_op: FnPtr<fn(*const (), f32, f32)>,
    mut load_op: FnPtr<fn(*const ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_30(
    mut store_op: FnPtr<fn(*const (), f64, f64)>,
    mut load_op: FnPtr<fn(*const ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_31(
    mut store_op: FnPtr<fn(*const (), *mut i32, *mut i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *mut i32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_32(
    mut store_op: FnPtr<fn(*const (), *const i32, *const i32)>,
    mut load_op: FnPtr<fn(*const ()) -> *const i32>,
) {
    &(store_op);
    &(load_op);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveOperatorPlusEquals_bool_ {}
impl TestDoesNotHaveOperatorPlusEquals_bool_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_operator_plus_equals<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveOperatorPlusEquals_UserAtomicType_ {}
impl TestDoesNotHaveOperatorPlusEquals_UserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_operator_plus_equals<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_ {}
impl TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_operator_plus_equals<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_float_ {}
impl TestOperatorPlusEquals_float_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: f32 = 1_f32;
            let a: () = unimplemented!();
            let mut y: f32 = (unimplemented!());
            assert!(((y) == (3_f32)));
            assert!(((x) == (3_f32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), f32, f32)> =
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
            (unsafe { test_seq_cst_29(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_double_ {}
impl TestOperatorPlusEquals_double_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: f64 = 1_f64;
            let a: () = unimplemented!();
            let mut y: f64 = (unimplemented!());
            assert!(((y) == (3_f64)));
            assert!(((x) == (3_f64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), f64, f64)> =
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
            (unsafe { test_seq_cst_30(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_char_ {}
impl TestOperatorPlusEquals_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: libc::c_char = (1 as libc::c_char);
            let a: () = unimplemented!();
            let mut y: libc::c_char = (unimplemented!());
            assert!(((y as i32) == ((3 as libc::c_char) as i32)));
            assert!(((x as i32) == ((3 as libc::c_char) as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), libc::c_char, libc::c_char)> =
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
            (unsafe { test_seq_cst_14(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_signed_char_ {}
impl TestOperatorPlusEquals_signed_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i8 = 1_i8;
            let a: () = unimplemented!();
            let mut y: i8 = (unimplemented!());
            assert!(((y as i32) == (3_i8 as i32)));
            assert!(((x as i32) == (3_i8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), i8, i8)> = FnPtr::<fn(*const (), i8, i8)>::new(
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
            (unsafe { test_seq_cst_15(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_unsigned_char_ {}
impl TestOperatorPlusEquals_unsigned_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u8 = 1_u8;
            let a: () = unimplemented!();
            let mut y: u8 = (unimplemented!());
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u8, u8)> = FnPtr::<fn(*const (), u8, u8)>::new(
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
            (unsafe { test_seq_cst_16(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_short_ {}
impl TestOperatorPlusEquals_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i16 = 1_i16;
            let a: () = unimplemented!();
            let mut y: i16 = (unimplemented!());
            assert!(((y as i32) == (3_i16 as i32)));
            assert!(((x as i32) == (3_i16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), i16, i16)> =
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
            (unsafe { test_seq_cst_17(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_unsigned_short_ {}
impl TestOperatorPlusEquals_unsigned_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u16 = 1_u16;
            let a: () = unimplemented!();
            let mut y: u16 = (unimplemented!());
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u16, u16)> =
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
            (unsafe { test_seq_cst_18(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_int_ {}
impl TestOperatorPlusEquals_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i32 = 1;
            let a: () = unimplemented!();
            let mut y: i32 = (unimplemented!());
            assert!(((y) == (3)));
            assert!(((x) == (3)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), i32, i32)> =
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
            (unsafe { test_seq_cst_19(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_unsigned_int_ {}
impl TestOperatorPlusEquals_unsigned_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u32 = 1_u32;
            let a: () = unimplemented!();
            let mut y: u32 = (unimplemented!());
            assert!(((y) == (3_u32)));
            assert!(((x) == (3_u32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u32, u32)> =
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
            (unsafe { test_seq_cst_20(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_long_ {}
impl TestOperatorPlusEquals_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i64 = 1_i64;
            let a: () = unimplemented!();
            let mut y: i64 = (unimplemented!());
            assert!(((y) == (3_i64)));
            assert!(((x) == (3_i64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), i64, i64)> =
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
            (unsafe { test_seq_cst_21(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_unsigned_long_ {}
impl TestOperatorPlusEquals_unsigned_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u64 = 1_u64;
            let a: () = unimplemented!();
            let mut y: u64 = (unimplemented!());
            assert!(((y) == (3_u64)));
            assert!(((x) == (3_u64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u64, u64)> =
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
            (unsafe { test_seq_cst_22(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_long_long_ {}
impl TestOperatorPlusEquals_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i64 = 1_i64;
            let a: () = unimplemented!();
            let mut y: i64 = (unimplemented!());
            assert!(((y) == (3_i64)));
            assert!(((x) == (3_i64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), i64, i64)> =
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
            (unsafe { test_seq_cst_23(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_unsigned_long_long_ {}
impl TestOperatorPlusEquals_unsigned_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u64 = 1_u64;
            let a: () = unimplemented!();
            let mut y: u64 = (unimplemented!());
            assert!(((y) == (3_u64)));
            assert!(((x) == (3_u64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u64, u64)> =
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
            (unsafe { test_seq_cst_24(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_wchar_t_ {}
impl TestOperatorPlusEquals_wchar_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i32 = 1_i32;
            let a: () = unimplemented!();
            let mut y: i32 = (unimplemented!());
            assert!(((y) == (3_i32)));
            assert!(((x) == (3_i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), i32, i32)> =
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
            (unsafe { test_seq_cst_25(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_char8_t_ {}
impl TestOperatorPlusEquals_char8_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u8 = 1_u8;
            let a: () = unimplemented!();
            let mut y: u8 = (unimplemented!());
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u8, u8)> = FnPtr::<fn(*const (), u8, u8)>::new(
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
            (unsafe { test_seq_cst_26(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_char16_t_ {}
impl TestOperatorPlusEquals_char16_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u16 = 1_u16;
            let a: () = unimplemented!();
            let mut y: u16 = (unimplemented!());
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u16, u16)> =
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
            (unsafe { test_seq_cst_27(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_char32_t_ {}
impl TestOperatorPlusEquals_char32_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u32 = 1_u32;
            let a: () = unimplemented!();
            let mut y: u32 = (unimplemented!());
            assert!(((y) == (3_u32)));
            assert!(((x) == (3_u32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), u32, u32)> =
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
            (unsafe { test_seq_cst_28(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_int_ptr_ {}
impl TestOperatorPlusEquals_int_ptr_ {
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
                let mut p: *mut i32 = (&mut t[(1) as usize] as *mut ());
                let a: () = unimplemented!();
                let mut y: *mut i32 = (unimplemented!());
                assert!(((y) == (&mut t[(3) as usize] as *mut ())));
                assert!(((unimplemented!()) == (&mut t[(3) as usize] as *mut ())));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), *mut i32, *mut i32)> =
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
            (unsafe { test_seq_cst_31(plus_equals.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestOperatorPlusEquals_const_int_ptr_ {}
impl TestOperatorPlusEquals_const_int_ptr_ {
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
                let mut p: *const i32 = (&t[(1) as usize] as *const ());
                let a: () = unimplemented!();
                let mut y: *const i32 = (unimplemented!());
                assert!(((y) == (&t[(3) as usize] as *const ())));
                assert!(((unimplemented!()) == (&t[(3) as usize] as *const ())));
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), *const i32, *const i32)> =
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
            (unsafe { test_seq_cst_32(plus_equals.copy_from(), load.copy_from()) });
        }
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
        TestEachIntegralType_TestOperatorPlusEquals_::operator_call(
            &<TestEachIntegralType_TestOperatorPlusEquals_>::default(),
        )
    });
    (unsafe {
        TestOperatorPlusEquals_float_::operator_call(&<TestOperatorPlusEquals_float_>::default())
    });
    (unsafe {
        TestOperatorPlusEquals_double_::operator_call(&<TestOperatorPlusEquals_double_>::default())
    });
    (unsafe {
        TestEachPointerType_TestOperatorPlusEquals_::operator_call(
            &<TestEachPointerType_TestOperatorPlusEquals_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveOperatorPlusEquals_bool_::operator_call(
            &<TestDoesNotHaveOperatorPlusEquals_bool_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveOperatorPlusEquals_UserAtomicType_::operator_call(
            &<TestDoesNotHaveOperatorPlusEquals_UserAtomicType_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_::operator_call(
            &<TestDoesNotHaveOperatorPlusEquals_LargeUserAtomicType_>::default(),
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
