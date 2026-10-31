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
pub struct TestEachIntegralType_TestIncrementDecrement_ {}
impl TestEachIntegralType_TestIncrementDecrement_ {
    pub unsafe fn operator_call(&self) {
        (unsafe {
            TestIncrementDecrement_char_::operator_call(&<TestIncrementDecrement_char_>::default())
        });
        (unsafe {
            TestIncrementDecrement_signed_char_::operator_call(
                &<TestIncrementDecrement_signed_char_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_unsigned_char_::operator_call(
                &<TestIncrementDecrement_unsigned_char_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_short_::operator_call(&<TestIncrementDecrement_short_>::default())
        });
        (unsafe {
            TestIncrementDecrement_unsigned_short_::operator_call(
                &<TestIncrementDecrement_unsigned_short_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_int_::operator_call(&<TestIncrementDecrement_int_>::default())
        });
        (unsafe {
            TestIncrementDecrement_unsigned_int_::operator_call(
                &<TestIncrementDecrement_unsigned_int_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_long_::operator_call(&<TestIncrementDecrement_long_>::default())
        });
        (unsafe {
            TestIncrementDecrement_unsigned_long_::operator_call(
                &<TestIncrementDecrement_unsigned_long_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_long_long_::operator_call(
                &<TestIncrementDecrement_long_long_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_unsigned_long_long_::operator_call(
                &<TestIncrementDecrement_unsigned_long_long_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_wchar_t_::operator_call(
                &<TestIncrementDecrement_wchar_t_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_char8_t_::operator_call(
                &<TestIncrementDecrement_char8_t_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_char16_t_::operator_call(
                &<TestIncrementDecrement_char16_t_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_char32_t_::operator_call(
                &<TestIncrementDecrement_char32_t_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_signed_char_::operator_call(
                &<TestIncrementDecrement_signed_char_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_unsigned_char_::operator_call(
                &<TestIncrementDecrement_unsigned_char_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_short_::operator_call(&<TestIncrementDecrement_short_>::default())
        });
        (unsafe {
            TestIncrementDecrement_unsigned_short_::operator_call(
                &<TestIncrementDecrement_unsigned_short_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_int_::operator_call(&<TestIncrementDecrement_int_>::default())
        });
        (unsafe {
            TestIncrementDecrement_unsigned_int_::operator_call(
                &<TestIncrementDecrement_unsigned_int_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_long_::operator_call(&<TestIncrementDecrement_long_>::default())
        });
        (unsafe {
            TestIncrementDecrement_unsigned_long_::operator_call(
                &<TestIncrementDecrement_unsigned_long_>::default(),
            )
        });
    }
}
impl Callable0<()> for TestEachIntegralType_TestIncrementDecrement_ {
    fn call(&self) -> () {
        unsafe { TestEachIntegralType_TestIncrementDecrement_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachFloatingPointType_TestDoesNotHaveIncrementDecrement_ {}
impl TestEachFloatingPointType_TestDoesNotHaveIncrementDecrement_ {
    pub unsafe fn operator_call(&self) {
        (unsafe {
            TestDoesNotHaveIncrementDecrement_float_::operator_call(
                &<TestDoesNotHaveIncrementDecrement_float_>::default(),
            )
        });
        (unsafe {
            TestDoesNotHaveIncrementDecrement_double_::operator_call(
                &<TestDoesNotHaveIncrementDecrement_double_>::default(),
            )
        });
        (unsafe {
            TestDoesNotHaveIncrementDecrement_long_double_::operator_call(
                &<TestDoesNotHaveIncrementDecrement_long_double_>::default(),
            )
        });
    }
}
impl Callable0<()> for TestEachFloatingPointType_TestDoesNotHaveIncrementDecrement_ {
    fn call(&self) -> () {
        unsafe { TestEachFloatingPointType_TestDoesNotHaveIncrementDecrement_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachPointerType_TestIncrementDecrement_ {}
impl TestEachPointerType_TestIncrementDecrement_ {
    pub unsafe fn operator_call(&self) {
        (unsafe {
            TestIncrementDecrement_int_ptr_::operator_call(
                &<TestIncrementDecrement_int_ptr_>::default(),
            )
        });
        (unsafe {
            TestIncrementDecrement_const_int_ptr_::operator_call(
                &<TestIncrementDecrement_const_int_ptr_>::default(),
            )
        });
    }
}
impl Callable0<()> for TestEachPointerType_TestIncrementDecrement_ {
    fn call(&self) -> () {
        unsafe { TestEachPointerType_TestIncrementDecrement_::operator_call(self) }
    }
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_14() -> bool {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_15() -> bool {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_16() -> bool {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_17() -> bool {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_18() -> bool {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_19() -> bool {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveIncrementDecrement_bool_ {}
impl TestDoesNotHaveIncrementDecrement_bool_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(
            true,
            "does_not_have_increment_nor_decrement_operators<std::atomic_ref<T>>()"
        );;
    }
}
impl Callable0<()> for TestDoesNotHaveIncrementDecrement_bool_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveIncrementDecrement_bool_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveIncrementDecrement_UserAtomicType_ {}
impl TestDoesNotHaveIncrementDecrement_UserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(
            true,
            "does_not_have_increment_nor_decrement_operators<std::atomic_ref<T>>()"
        );;
    }
}
impl Callable0<()> for TestDoesNotHaveIncrementDecrement_UserAtomicType_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveIncrementDecrement_UserAtomicType_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveIncrementDecrement_LargeUserAtomicType_ {}
impl TestDoesNotHaveIncrementDecrement_LargeUserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(
            true,
            "does_not_have_increment_nor_decrement_operators<std::atomic_ref<T>>()"
        );;
    }
}
impl Callable0<()> for TestDoesNotHaveIncrementDecrement_LargeUserAtomicType_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveIncrementDecrement_LargeUserAtomicType_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveIncrementDecrement_float_ {}
impl TestDoesNotHaveIncrementDecrement_float_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(
            true,
            "does_not_have_increment_nor_decrement_operators<std::atomic_ref<T>>()"
        );;
    }
}
impl Callable0<()> for TestDoesNotHaveIncrementDecrement_float_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveIncrementDecrement_float_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveIncrementDecrement_double_ {}
impl TestDoesNotHaveIncrementDecrement_double_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(
            true,
            "does_not_have_increment_nor_decrement_operators<std::atomic_ref<T>>()"
        );;
    }
}
impl Callable0<()> for TestDoesNotHaveIncrementDecrement_double_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveIncrementDecrement_double_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveIncrementDecrement_long_double_ {}
impl TestDoesNotHaveIncrementDecrement_long_double_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(
            true,
            "does_not_have_increment_nor_decrement_operators<std::atomic_ref<T>>()"
        );;
    }
}
impl Callable0<()> for TestDoesNotHaveIncrementDecrement_long_double_ {
    fn call(&self) -> () {
        unsafe { TestDoesNotHaveIncrementDecrement_long_double_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char_ {}
impl TestIncrementDecrement_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: libc::c_char = (1 as libc::c_char);
            let a: () = unimplemented!();
            {
                let mut y: libc::c_char = unimplemented!();
                assert!(((y as i32) == ((2 as libc::c_char) as i32)));
                assert!(((x as i32) == ((2 as libc::c_char) as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: libc::c_char = unimplemented!();
                assert!(((y as i32) == ((1 as libc::c_char) as i32)));
                assert!(((x as i32) == ((1 as libc::c_char) as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: libc::c_char = unimplemented!();
                assert!(((y as i32) == ((1 as libc::c_char) as i32)));
                assert!(((x as i32) == ((2 as libc::c_char) as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: libc::c_char = unimplemented!();
                assert!(((y as i32) == ((2 as libc::c_char) as i32)));
                assert!(((x as i32) == ((1 as libc::c_char) as i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_char_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_char_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_signed_char_ {}
impl TestIncrementDecrement_signed_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i8 = 1_i8;
            let a: () = unimplemented!();
            {
                let mut y: i8 = unimplemented!();
                assert!(((y as i32) == (2_i8 as i32)));
                assert!(((x as i32) == (2_i8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i8 = unimplemented!();
                assert!(((y as i32) == (1_i8 as i32)));
                assert!(((x as i32) == (1_i8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i8 = unimplemented!();
                assert!(((y as i32) == (1_i8 as i32)));
                assert!(((x as i32) == (2_i8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i8 = unimplemented!();
                assert!(((y as i32) == (2_i8 as i32)));
                assert!(((x as i32) == (1_i8 as i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_signed_char_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_signed_char_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_char_ {}
impl TestIncrementDecrement_unsigned_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u8 = 1_u8;
            let a: () = unimplemented!();
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (2_u8 as i32)));
                assert!(((x as i32) == (2_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (1_u8 as i32)));
                assert!(((x as i32) == (1_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (1_u8 as i32)));
                assert!(((x as i32) == (2_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (2_u8 as i32)));
                assert!(((x as i32) == (1_u8 as i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_unsigned_char_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_unsigned_char_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_short_ {}
impl TestIncrementDecrement_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i16 = 1_i16;
            let a: () = unimplemented!();
            {
                let mut y: i16 = unimplemented!();
                assert!(((y as i32) == (2_i16 as i32)));
                assert!(((x as i32) == (2_i16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i16 = unimplemented!();
                assert!(((y as i32) == (1_i16 as i32)));
                assert!(((x as i32) == (1_i16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i16 = unimplemented!();
                assert!(((y as i32) == (1_i16 as i32)));
                assert!(((x as i32) == (2_i16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i16 = unimplemented!();
                assert!(((y as i32) == (2_i16 as i32)));
                assert!(((x as i32) == (1_i16 as i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_short_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_short_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_short_ {}
impl TestIncrementDecrement_unsigned_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u16 = 1_u16;
            let a: () = unimplemented!();
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (2_u16 as i32)));
                assert!(((x as i32) == (2_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (1_u16 as i32)));
                assert!(((x as i32) == (1_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (1_u16 as i32)));
                assert!(((x as i32) == (2_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (2_u16 as i32)));
                assert!(((x as i32) == (1_u16 as i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_unsigned_short_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_unsigned_short_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_int_ {}
impl TestIncrementDecrement_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i32 = 1;
            let a: () = unimplemented!();
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (2)));
                assert!(((x) == (2)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (1)));
                assert!(((x) == (1)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (1)));
                assert!(((x) == (2)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (2)));
                assert!(((x) == (1)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_int_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_int_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_int_ {}
impl TestIncrementDecrement_unsigned_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u32 = 1_u32;
            let a: () = unimplemented!();
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (2_u32)));
                assert!(((x) == (2_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (1_u32)));
                assert!(((x) == (1_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (1_u32)));
                assert!(((x) == (2_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (2_u32)));
                assert!(((x) == (1_u32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_unsigned_int_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_unsigned_int_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_long_ {}
impl TestIncrementDecrement_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i64 = 1_i64;
            let a: () = unimplemented!();
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (2_i64)));
                assert!(((x) == (2_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (1_i64)));
                assert!(((x) == (1_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (1_i64)));
                assert!(((x) == (2_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (2_i64)));
                assert!(((x) == (1_i64)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_long_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_long_ {}
impl TestIncrementDecrement_unsigned_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u64 = 1_u64;
            let a: () = unimplemented!();
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (2_u64)));
                assert!(((x) == (2_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (1_u64)));
                assert!(((x) == (1_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (1_u64)));
                assert!(((x) == (2_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (2_u64)));
                assert!(((x) == (1_u64)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_unsigned_long_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_unsigned_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_long_long_ {}
impl TestIncrementDecrement_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i64 = 1_i64;
            let a: () = unimplemented!();
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (2_i64)));
                assert!(((x) == (2_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (1_i64)));
                assert!(((x) == (1_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (1_i64)));
                assert!(((x) == (2_i64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i64 = unimplemented!();
                assert!(((y) == (2_i64)));
                assert!(((x) == (1_i64)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_long_long_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_long_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_long_long_ {}
impl TestIncrementDecrement_unsigned_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u64 = 1_u64;
            let a: () = unimplemented!();
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (2_u64)));
                assert!(((x) == (2_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (1_u64)));
                assert!(((x) == (1_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (1_u64)));
                assert!(((x) == (2_u64)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u64 = unimplemented!();
                assert!(((y) == (2_u64)));
                assert!(((x) == (1_u64)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_unsigned_long_long_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_unsigned_long_long_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_wchar_t_ {}
impl TestIncrementDecrement_wchar_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: i32 = 1_i32;
            let a: () = unimplemented!();
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (2_i32)));
                assert!(((x) == (2_i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (1_i32)));
                assert!(((x) == (1_i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (1_i32)));
                assert!(((x) == (2_i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: i32 = unimplemented!();
                assert!(((y) == (2_i32)));
                assert!(((x) == (1_i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_wchar_t_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_wchar_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char8_t_ {}
impl TestIncrementDecrement_char8_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u8 = 1_u8;
            let a: () = unimplemented!();
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (2_u8 as i32)));
                assert!(((x as i32) == (2_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (1_u8 as i32)));
                assert!(((x as i32) == (1_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (1_u8 as i32)));
                assert!(((x as i32) == (2_u8 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u8 = unimplemented!();
                assert!(((y as i32) == (2_u8 as i32)));
                assert!(((x as i32) == (1_u8 as i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_char8_t_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_char8_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char16_t_ {}
impl TestIncrementDecrement_char16_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u16 = 1_u16;
            let a: () = unimplemented!();
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (2_u16 as i32)));
                assert!(((x as i32) == (2_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (1_u16 as i32)));
                assert!(((x as i32) == (1_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (1_u16 as i32)));
                assert!(((x as i32) == (2_u16 as i32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u16 = unimplemented!();
                assert!(((y as i32) == (2_u16 as i32)));
                assert!(((x as i32) == (1_u16 as i32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_char16_t_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_char16_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char32_t_ {}
impl TestIncrementDecrement_char32_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: u32 = 1_u32;
            let a: () = unimplemented!();
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (2_u32)));
                assert!(((x) == (2_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (1_u32)));
                assert!(((x) == (1_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (1_u32)));
                assert!(((x) == (2_u32)));
                const _: () = assert!(true, "");;
            }
            {
                let mut y: u32 = unimplemented!();
                assert!(((y) == (2_u32)));
                assert!(((x) == (1_u32)));
                const _: () = assert!(true, "");;
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_char32_t_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_char32_t_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_int_ptr_ {}
impl TestIncrementDecrement_int_ptr_ {
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
                {
                    let mut y: *mut i32 = unimplemented!();
                    assert!(((y) == (&mut t[(2) as usize] as *mut ())));
                    assert!(((p) == (&mut t[(2) as usize] as *mut ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *mut i32 = unimplemented!();
                    assert!(((y) == (&mut t[(1) as usize] as *mut ())));
                    assert!(((p) == (&mut t[(1) as usize] as *mut ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *mut i32 = unimplemented!();
                    assert!(((y) == (&mut t[(1) as usize] as *mut ())));
                    assert!(((p) == (&mut t[(2) as usize] as *mut ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *mut i32 = unimplemented!();
                    assert!(((y) == (&mut t[(2) as usize] as *mut ())));
                    assert!(((p) == (&mut t[(1) as usize] as *mut ())));
                    const _: () = assert!(true, "");;
                }
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_int_ptr_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_int_ptr_::operator_call(self) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_const_int_ptr_ {}
impl TestIncrementDecrement_const_int_ptr_ {
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
                {
                    let mut y: *const i32 = unimplemented!();
                    assert!(((y) == (&t[(2) as usize] as *const ())));
                    assert!(((p) == (&t[(2) as usize] as *const ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *const i32 = unimplemented!();
                    assert!(((y) == (&t[(1) as usize] as *const ())));
                    assert!(((p) == (&t[(1) as usize] as *const ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *const i32 = unimplemented!();
                    assert!(((y) == (&t[(1) as usize] as *const ())));
                    assert!(((p) == (&t[(2) as usize] as *const ())));
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *const i32 = unimplemented!();
                    assert!(((y) == (&t[(2) as usize] as *const ())));
                    assert!(((p) == (&t[(1) as usize] as *const ())));
                    const _: () = assert!(true, "");;
                }
            }
        }
    }
}
impl Callable0<()> for TestIncrementDecrement_const_int_ptr_ {
    fn call(&self) -> () {
        unsafe { TestIncrementDecrement_const_int_ptr_::operator_call(self) }
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
        TestEachIntegralType_TestIncrementDecrement_::operator_call(
            &<TestEachIntegralType_TestIncrementDecrement_>::default(),
        )
    });
    (unsafe {
        TestEachFloatingPointType_TestDoesNotHaveIncrementDecrement_::operator_call(
            &<TestEachFloatingPointType_TestDoesNotHaveIncrementDecrement_>::default(),
        )
    });
    (unsafe {
        TestEachPointerType_TestIncrementDecrement_::operator_call(
            &<TestEachPointerType_TestIncrementDecrement_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveIncrementDecrement_bool_::operator_call(
            &<TestDoesNotHaveIncrementDecrement_bool_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveIncrementDecrement_UserAtomicType_::operator_call(
            &<TestDoesNotHaveIncrementDecrement_UserAtomicType_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveIncrementDecrement_LargeUserAtomicType_::operator_call(
            &<TestDoesNotHaveIncrementDecrement_LargeUserAtomicType_>::default(),
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
