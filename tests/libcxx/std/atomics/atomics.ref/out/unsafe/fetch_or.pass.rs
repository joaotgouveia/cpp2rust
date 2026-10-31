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
pub struct TestEachIntegralType_TestFetchOr_ {}
impl TestEachIntegralType_TestFetchOr_ {
    pub unsafe fn operator_call(&self) {
        (unsafe { TestFetchOr_char_::operator_call(&<TestFetchOr_char_>::default()) });
        (unsafe {
            TestFetchOr_signed_char_::operator_call(&<TestFetchOr_signed_char_>::default())
        });
        (unsafe {
            TestFetchOr_unsigned_char_::operator_call(&<TestFetchOr_unsigned_char_>::default())
        });
        (unsafe { TestFetchOr_short_::operator_call(&<TestFetchOr_short_>::default()) });
        (unsafe {
            TestFetchOr_unsigned_short_::operator_call(&<TestFetchOr_unsigned_short_>::default())
        });
        (unsafe { TestFetchOr_int_::operator_call(&<TestFetchOr_int_>::default()) });
        (unsafe {
            TestFetchOr_unsigned_int_::operator_call(&<TestFetchOr_unsigned_int_>::default())
        });
        (unsafe { TestFetchOr_long_::operator_call(&<TestFetchOr_long_>::default()) });
        (unsafe {
            TestFetchOr_unsigned_long_::operator_call(&<TestFetchOr_unsigned_long_>::default())
        });
        (unsafe { TestFetchOr_long_long_::operator_call(&<TestFetchOr_long_long_>::default()) });
        (unsafe {
            TestFetchOr_unsigned_long_long_::operator_call(
                &<TestFetchOr_unsigned_long_long_>::default(),
            )
        });
        (unsafe { TestFetchOr_wchar_t_::operator_call(&<TestFetchOr_wchar_t_>::default()) });
        (unsafe { TestFetchOr_char8_t_::operator_call(&<TestFetchOr_char8_t_>::default()) });
        (unsafe { TestFetchOr_char16_t_::operator_call(&<TestFetchOr_char16_t_>::default()) });
        (unsafe { TestFetchOr_char32_t_::operator_call(&<TestFetchOr_char32_t_>::default()) });
        (unsafe {
            TestFetchOr_signed_char_::operator_call(&<TestFetchOr_signed_char_>::default())
        });
        (unsafe {
            TestFetchOr_unsigned_char_::operator_call(&<TestFetchOr_unsigned_char_>::default())
        });
        (unsafe { TestFetchOr_short_::operator_call(&<TestFetchOr_short_>::default()) });
        (unsafe {
            TestFetchOr_unsigned_short_::operator_call(&<TestFetchOr_unsigned_short_>::default())
        });
        (unsafe { TestFetchOr_int_::operator_call(&<TestFetchOr_int_>::default()) });
        (unsafe {
            TestFetchOr_unsigned_int_::operator_call(&<TestFetchOr_unsigned_int_>::default())
        });
        (unsafe { TestFetchOr_long_::operator_call(&<TestFetchOr_long_>::default()) });
        (unsafe {
            TestFetchOr_unsigned_long_::operator_call(&<TestFetchOr_unsigned_long_>::default())
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachFloatingPointType_TestDoesNotHaveFetchOr_ {}
impl TestEachFloatingPointType_TestDoesNotHaveFetchOr_ {
    pub unsafe fn operator_call(&self) {
        (unsafe {
            TestDoesNotHaveFetchOr_float_::operator_call(&<TestDoesNotHaveFetchOr_float_>::default())
        });
        (unsafe {
            TestDoesNotHaveFetchOr_double_::operator_call(
                &<TestDoesNotHaveFetchOr_double_>::default(),
            )
        });
        (unsafe {
            TestDoesNotHaveFetchOr_long_double_::operator_call(
                &<TestDoesNotHaveFetchOr_long_double_>::default(),
            )
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachPointerType_TestDoesNotHaveFetchOr_ {}
impl TestEachPointerType_TestDoesNotHaveFetchOr_ {
    pub unsafe fn operator_call(&self) {
        (unsafe {
            TestDoesNotHaveFetchOr_int_ptr_::operator_call(
                &<TestDoesNotHaveFetchOr_int_ptr_>::default(),
            )
        });
        (unsafe {
            TestDoesNotHaveFetchOr_const_int_ptr_::operator_call(
                &<TestDoesNotHaveFetchOr_const_int_ptr_>::default(),
            )
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_bool_ {}
impl TestDoesNotHaveFetchOr_bool_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_UserAtomicType_ {}
impl TestDoesNotHaveFetchOr_UserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_LargeUserAtomicType_ {}
impl TestDoesNotHaveFetchOr_LargeUserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_float_ {}
impl TestDoesNotHaveFetchOr_float_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_double_ {}
impl TestDoesNotHaveFetchOr_double_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_long_double_ {}
impl TestDoesNotHaveFetchOr_long_double_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_int_ptr_ {}
impl TestDoesNotHaveFetchOr_int_ptr_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchOr_const_int_ptr_ {}
impl TestDoesNotHaveFetchOr_const_int_ptr_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_or<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_char_ {}
impl TestFetchOr_char_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: libc::c_char = (1 as libc::c_char);
        let a: () = unimplemented!();
        {
            let mut y: libc::c_char = unimplemented!();
            assert!(((y as i32) == ((1 as libc::c_char) as i32)));
            assert!(((x as i32) == ((3 as libc::c_char) as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: libc::c_char = unimplemented!();
            assert!(((y as i32) == ((3 as libc::c_char) as i32)));
            assert!(((x as i32) == ((3 as libc::c_char) as i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_signed_char_ {}
impl TestFetchOr_signed_char_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i8 = 1_i8;
        let a: () = unimplemented!();
        {
            let mut y: i8 = unimplemented!();
            assert!(((y as i32) == (1_i8 as i32)));
            assert!(((x as i32) == (3_i8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i8 = unimplemented!();
            assert!(((y as i32) == (3_i8 as i32)));
            assert!(((x as i32) == (3_i8 as i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_unsigned_char_ {}
impl TestFetchOr_unsigned_char_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u8 = 1_u8;
        let a: () = unimplemented!();
        {
            let mut y: u8 = unimplemented!();
            assert!(((y as i32) == (1_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u8 = unimplemented!();
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_short_ {}
impl TestFetchOr_short_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i16 = 1_i16;
        let a: () = unimplemented!();
        {
            let mut y: i16 = unimplemented!();
            assert!(((y as i32) == (1_i16 as i32)));
            assert!(((x as i32) == (3_i16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i16 = unimplemented!();
            assert!(((y as i32) == (3_i16 as i32)));
            assert!(((x as i32) == (3_i16 as i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_unsigned_short_ {}
impl TestFetchOr_unsigned_short_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u16 = 1_u16;
        let a: () = unimplemented!();
        {
            let mut y: u16 = unimplemented!();
            assert!(((y as i32) == (1_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u16 = unimplemented!();
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_int_ {}
impl TestFetchOr_int_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i32 = 1;
        let a: () = unimplemented!();
        {
            let mut y: i32 = unimplemented!();
            assert!(((y) == (1)));
            assert!(((x) == (3)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i32 = unimplemented!();
            assert!(((y) == (3)));
            assert!(((x) == (3)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_unsigned_int_ {}
impl TestFetchOr_unsigned_int_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u32 = 1_u32;
        let a: () = unimplemented!();
        {
            let mut y: u32 = unimplemented!();
            assert!(((y) == (1_u32)));
            assert!(((x) == (3_u32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u32 = unimplemented!();
            assert!(((y) == (3_u32)));
            assert!(((x) == (3_u32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_long_ {}
impl TestFetchOr_long_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i64 = 1_i64;
        let a: () = unimplemented!();
        {
            let mut y: i64 = unimplemented!();
            assert!(((y) == (1_i64)));
            assert!(((x) == (3_i64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i64 = unimplemented!();
            assert!(((y) == (3_i64)));
            assert!(((x) == (3_i64)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_unsigned_long_ {}
impl TestFetchOr_unsigned_long_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u64 = 1_u64;
        let a: () = unimplemented!();
        {
            let mut y: u64 = unimplemented!();
            assert!(((y) == (1_u64)));
            assert!(((x) == (3_u64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u64 = unimplemented!();
            assert!(((y) == (3_u64)));
            assert!(((x) == (3_u64)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_long_long_ {}
impl TestFetchOr_long_long_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i64 = 1_i64;
        let a: () = unimplemented!();
        {
            let mut y: i64 = unimplemented!();
            assert!(((y) == (1_i64)));
            assert!(((x) == (3_i64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i64 = unimplemented!();
            assert!(((y) == (3_i64)));
            assert!(((x) == (3_i64)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_unsigned_long_long_ {}
impl TestFetchOr_unsigned_long_long_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u64 = 1_u64;
        let a: () = unimplemented!();
        {
            let mut y: u64 = unimplemented!();
            assert!(((y) == (1_u64)));
            assert!(((x) == (3_u64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u64 = unimplemented!();
            assert!(((y) == (3_u64)));
            assert!(((x) == (3_u64)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_wchar_t_ {}
impl TestFetchOr_wchar_t_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: i32 = 1_i32;
        let a: () = unimplemented!();
        {
            let mut y: i32 = unimplemented!();
            assert!(((y) == (1_i32)));
            assert!(((x) == (3_i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: i32 = unimplemented!();
            assert!(((y) == (3_i32)));
            assert!(((x) == (3_i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_char8_t_ {}
impl TestFetchOr_char8_t_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u8 = 1_u8;
        let a: () = unimplemented!();
        {
            let mut y: u8 = unimplemented!();
            assert!(((y as i32) == (1_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u8 = unimplemented!();
            assert!(((y as i32) == (3_u8 as i32)));
            assert!(((x as i32) == (3_u8 as i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_char16_t_ {}
impl TestFetchOr_char16_t_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u16 = 1_u16;
        let a: () = unimplemented!();
        {
            let mut y: u16 = unimplemented!();
            assert!(((y as i32) == (1_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u16 = unimplemented!();
            assert!(((y as i32) == (3_u16 as i32)));
            assert!(((x as i32) == (3_u16 as i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchOr_char32_t_ {}
impl TestFetchOr_char32_t_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(true, "std::is_integral_v<T>");;
        let mut x: u32 = 1_u32;
        let a: () = unimplemented!();
        {
            let mut y: u32 = unimplemented!();
            assert!(((y) == (1_u32)));
            assert!(((x) == (3_u32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut y: u32 = unimplemented!();
            assert!(((y) == (3_u32)));
            assert!(((x) == (3_u32)));
            const _: () = assert!(true, "");;
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
        TestEachIntegralType_TestFetchOr_::operator_call(
            &<TestEachIntegralType_TestFetchOr_>::default(),
        )
    });
    (unsafe {
        TestEachFloatingPointType_TestDoesNotHaveFetchOr_::operator_call(
            &<TestEachFloatingPointType_TestDoesNotHaveFetchOr_>::default(),
        )
    });
    (unsafe {
        TestEachPointerType_TestDoesNotHaveFetchOr_::operator_call(
            &<TestEachPointerType_TestDoesNotHaveFetchOr_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveFetchOr_bool_::operator_call(&<TestDoesNotHaveFetchOr_bool_>::default())
    });
    (unsafe {
        TestDoesNotHaveFetchOr_UserAtomicType_::operator_call(
            &<TestDoesNotHaveFetchOr_UserAtomicType_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveFetchOr_LargeUserAtomicType_::operator_call(
            &<TestDoesNotHaveFetchOr_LargeUserAtomicType_>::default(),
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
