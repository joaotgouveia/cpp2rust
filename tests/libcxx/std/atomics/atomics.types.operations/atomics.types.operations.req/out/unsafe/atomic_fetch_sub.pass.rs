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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct LargeUserAtomicType {
    pub a: [i32; 128],
}
impl LargeUserAtomicType {
    pub unsafe fn new(mut d: Option<i32>) -> Self {
        let mut d: i32 = d.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self {
            a: [<i32>::default(); 128],
        };
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
pub struct TestEachIntegralType_TestFn_ {}
impl TestEachIntegralType_TestFn_ {
    pub unsafe fn operator_call(&self) {
        (unsafe { TestFn_char_::operator_call(&<TestFn_char_>::default()) });
        (unsafe { TestFn_signed_char_::operator_call(&<TestFn_signed_char_>::default()) });
        (unsafe { TestFn_unsigned_char_::operator_call(&<TestFn_unsigned_char_>::default()) });
        (unsafe { TestFn_short_::operator_call(&<TestFn_short_>::default()) });
        (unsafe { TestFn_unsigned_short_::operator_call(&<TestFn_unsigned_short_>::default()) });
        (unsafe { TestFn_int_::operator_call(&<TestFn_int_>::default()) });
        (unsafe { TestFn_unsigned_int_::operator_call(&<TestFn_unsigned_int_>::default()) });
        (unsafe { TestFn_long_::operator_call(&<TestFn_long_>::default()) });
        (unsafe { TestFn_unsigned_long_::operator_call(&<TestFn_unsigned_long_>::default()) });
        (unsafe { TestFn_long_long_::operator_call(&<TestFn_long_long_>::default()) });
        (unsafe {
            TestFn_unsigned_long_long_::operator_call(&<TestFn_unsigned_long_long_>::default())
        });
        (unsafe { TestFn_wchar_t_::operator_call(&<TestFn_wchar_t_>::default()) });
        (unsafe { TestFn_char8_t_::operator_call(&<TestFn_char8_t_>::default()) });
        (unsafe { TestFn_char16_t_::operator_call(&<TestFn_char16_t_>::default()) });
        (unsafe { TestFn_char32_t_::operator_call(&<TestFn_char32_t_>::default()) });
        (unsafe { TestFn_signed_char_::operator_call(&<TestFn_signed_char_>::default()) });
        (unsafe { TestFn_unsigned_char_::operator_call(&<TestFn_unsigned_char_>::default()) });
        (unsafe { TestFn_short_::operator_call(&<TestFn_short_>::default()) });
        (unsafe { TestFn_unsigned_short_::operator_call(&<TestFn_unsigned_short_>::default()) });
        (unsafe { TestFn_int_::operator_call(&<TestFn_int_>::default()) });
        (unsafe { TestFn_unsigned_int_::operator_call(&<TestFn_unsigned_int_>::default()) });
        (unsafe { TestFn_long_::operator_call(&<TestFn_long_>::default()) });
        (unsafe { TestFn_unsigned_long_::operator_call(&<TestFn_unsigned_long_>::default()) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_char_ {}
impl TestFn_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_char_ = std_atomic_char_::new_1({ (3 as libc::c_char) });
            assert!(
                (((unsafe {
                    atomic_fetch_sub_14((&mut t as *mut std_atomic_char_), (2 as libc::c_char))
                }) as i32)
                    == ((3 as libc::c_char) as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_char_).to_libcc_char_7() }) as i32)
                    == ((1 as libc::c_char) as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_char_ = std_atomic_char_::new_1({ (3 as libc::c_char) });
            assert!(
                (((unsafe {
                    atomic_fetch_sub_15((&mut t as *mut std_atomic_char_), (2 as libc::c_char))
                }) as i32)
                    == ((3 as libc::c_char) as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_char_).to_libcc_char_8() }) as i32)
                    == ((1 as libc::c_char) as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_signed_char_ {}
impl TestFn_signed_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_signed_char_ = std_atomic_signed_char_::new_1({ 3_i8 });
            assert!(
                (((unsafe { atomic_fetch_sub_16((&mut t as *mut std_atomic_signed_char_), 2_i8,) })
                    as i32)
                    == (3_i8 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_signed_char_).to_i8_7() }) as i32)
                    == (1_i8 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_signed_char_ = std_atomic_signed_char_::new_1({ 3_i8 });
            assert!(
                (((unsafe { atomic_fetch_sub_17((&mut t as *mut std_atomic_signed_char_), 2_i8,) })
                    as i32)
                    == (3_i8 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_signed_char_).to_i8_8() }) as i32)
                    == (1_i8 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_unsigned_char_ {}
impl TestFn_unsigned_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_unsigned_char_ = std_atomic_unsigned_char_::new_1({ 3_u8 });
            assert!(
                (((unsafe {
                    atomic_fetch_sub_18((&mut t as *mut std_atomic_unsigned_char_), 2_u8)
                }) as i32)
                    == (3_u8 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_unsigned_char_).to_u8_7() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_unsigned_char_ = std_atomic_unsigned_char_::new_1({ 3_u8 });
            assert!(
                (((unsafe {
                    atomic_fetch_sub_19((&mut t as *mut std_atomic_unsigned_char_), 2_u8)
                }) as i32)
                    == (3_u8 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_unsigned_char_).to_u8_8() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_short_ {}
impl TestFn_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_short_ = std_atomic_short_::new_1({ 3_i16 });
            assert!(
                (((unsafe { atomic_fetch_sub_20((&mut t as *mut std_atomic_short_), 2_i16,) })
                    as i32)
                    == (3_i16 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_short_).to_i16_7() }) as i32)
                    == (1_i16 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_short_ = std_atomic_short_::new_1({ 3_i16 });
            assert!(
                (((unsafe { atomic_fetch_sub_21((&mut t as *mut std_atomic_short_), 2_i16,) })
                    as i32)
                    == (3_i16 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_short_).to_i16_8() }) as i32)
                    == (1_i16 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_unsigned_short_ {}
impl TestFn_unsigned_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_unsigned_short_ = std_atomic_unsigned_short_::new_1({ 3_u16 });
            assert!(
                (((unsafe {
                    atomic_fetch_sub_22((&mut t as *mut std_atomic_unsigned_short_), 2_u16)
                }) as i32)
                    == (3_u16 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_unsigned_short_).to_u16_7() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_unsigned_short_ = std_atomic_unsigned_short_::new_1({ 3_u16 });
            assert!(
                (((unsafe {
                    atomic_fetch_sub_23((&mut t as *mut std_atomic_unsigned_short_), 2_u16)
                }) as i32)
                    == (3_u16 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_unsigned_short_).to_u16_8() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_int_ {}
impl TestFn_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_int_ = std_atomic_int_::new_1({ 3 });
            assert!(
                ((unsafe { atomic_fetch_sub_24((&mut t as *mut std_atomic_int_), 2,) }) == (3))
            );
            assert!(((unsafe { (t as std___atomic_base_int_).to_i32_7() }) == (1)));
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_int_ = std_atomic_int_::new_1({ 3 });
            assert!(
                ((unsafe { atomic_fetch_sub_25((&mut t as *mut std_atomic_int_), 2,) }) == (3))
            );
            assert!(((unsafe { (t as std___atomic_base_int_).to_i32_8() }) == (1)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_unsigned_int_ {}
impl TestFn_unsigned_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_unsigned_int_ = std_atomic_unsigned_int_::new_1({ 3_u32 });
            assert!(
                ((unsafe {
                    atomic_fetch_sub_26((&mut t as *mut std_atomic_unsigned_int_), 2_u32)
                }) == (3_u32))
            );
            assert!(((unsafe { (t as std___atomic_base_unsigned_int_).to_u32_7() }) == (1_u32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_unsigned_int_ = std_atomic_unsigned_int_::new_1({ 3_u32 });
            assert!(
                ((unsafe {
                    atomic_fetch_sub_27((&mut t as *mut std_atomic_unsigned_int_), 2_u32)
                }) == (3_u32))
            );
            assert!(((unsafe { (t as std___atomic_base_unsigned_int_).to_u32_8() }) == (1_u32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_long_ {}
impl TestFn_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_long_ = std_atomic_long_::new_1({ 3_i64 });
            assert!(
                ((unsafe { atomic_fetch_sub_28((&mut t as *mut std_atomic_long_), 2_i64,) })
                    == (3_i64))
            );
            assert!(((unsafe { (t as std___atomic_base_long_).to_i64_7() }) == (1_i64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_long_ = std_atomic_long_::new_1({ 3_i64 });
            assert!(
                ((unsafe { atomic_fetch_sub_29((&mut t as *mut std_atomic_long_), 2_i64,) })
                    == (3_i64))
            );
            assert!(((unsafe { (t as std___atomic_base_long_).to_i64_8() }) == (1_i64)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_unsigned_long_ {}
impl TestFn_unsigned_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_unsigned_long_ = std_atomic_unsigned_long_::new_1({ 3_u64 });
            assert!(
                ((unsafe {
                    atomic_fetch_sub_30((&mut t as *mut std_atomic_unsigned_long_), 2_u64)
                }) == (3_u64))
            );
            assert!(((unsafe { (t as std___atomic_base_unsigned_long_).to_u64_7() }) == (1_u64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_unsigned_long_ = std_atomic_unsigned_long_::new_1({ 3_u64 });
            assert!(
                ((unsafe {
                    atomic_fetch_sub_31((&mut t as *mut std_atomic_unsigned_long_), 2_u64)
                }) == (3_u64))
            );
            assert!(((unsafe { (t as std___atomic_base_unsigned_long_).to_u64_8() }) == (1_u64)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_long_long_ {}
impl TestFn_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_long_long_ = std_atomic_long_long_::new_1({ 3_i64 });
            assert!(
                ((unsafe { atomic_fetch_sub_32((&mut t as *mut std_atomic_long_long_), 2_i64,) })
                    == (3_i64))
            );
            assert!(((unsafe { (t as std___atomic_base_long_long_).to_i64_7() }) == (1_i64)));
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_long_long_ = std_atomic_long_long_::new_1({ 3_i64 });
            assert!(
                ((unsafe { atomic_fetch_sub_33((&mut t as *mut std_atomic_long_long_), 2_i64,) })
                    == (3_i64))
            );
            assert!(((unsafe { (t as std___atomic_base_long_long_).to_i64_8() }) == (1_i64)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_unsigned_long_long_ {}
impl TestFn_unsigned_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_unsigned_long_long_ =
                std_atomic_unsigned_long_long_::new_1({ 3_u64 });
            assert!(
                ((unsafe {
                    atomic_fetch_sub_34((&mut t as *mut std_atomic_unsigned_long_long_), 2_u64)
                }) == (3_u64))
            );
            assert!(
                ((unsafe { (t as std___atomic_base_unsigned_long_long_).to_u64_7() }) == (1_u64))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_unsigned_long_long_ =
                std_atomic_unsigned_long_long_::new_1({ 3_u64 });
            assert!(
                ((unsafe {
                    atomic_fetch_sub_35((&mut t as *mut std_atomic_unsigned_long_long_), 2_u64)
                }) == (3_u64))
            );
            assert!(
                ((unsafe { (t as std___atomic_base_unsigned_long_long_).to_u64_8() }) == (1_u64))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_wchar_t_ {}
impl TestFn_wchar_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_wchar_t_ = std_atomic_wchar_t_::new_1({ 3_i32 });
            assert!(
                ((unsafe { atomic_fetch_sub_36((&mut t as *mut std_atomic_wchar_t_), 2_i32,) })
                    == (3_i32))
            );
            assert!(((unsafe { (t as std___atomic_base_wchar_t_).to_i32_7() }) == (1_i32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_wchar_t_ = std_atomic_wchar_t_::new_1({ 3_i32 });
            assert!(
                ((unsafe { atomic_fetch_sub_37((&mut t as *mut std_atomic_wchar_t_), 2_i32,) })
                    == (3_i32))
            );
            assert!(((unsafe { (t as std___atomic_base_wchar_t_).to_i32_8() }) == (1_i32)));
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_char8_t_ {}
impl TestFn_char8_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_char8_t_ = std_atomic_char8_t_::new_1({ 3_u8 });
            assert!(
                (((unsafe { atomic_fetch_sub_38((&mut t as *mut std_atomic_char8_t_), 2_u8,) })
                    as i32)
                    == (3_u8 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_char8_t_).to_u8_7() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_char8_t_ = std_atomic_char8_t_::new_1({ 3_u8 });
            assert!(
                (((unsafe { atomic_fetch_sub_39((&mut t as *mut std_atomic_char8_t_), 2_u8,) })
                    as i32)
                    == (3_u8 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_char8_t_).to_u8_8() }) as i32)
                    == (1_u8 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_char16_t_ {}
impl TestFn_char16_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_char16_t_ = std_atomic_char16_t_::new_1({ 3_u16 });
            assert!(
                (((unsafe { atomic_fetch_sub_40((&mut t as *mut std_atomic_char16_t_), 2_u16,) })
                    as i32)
                    == (3_u16 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_char16_t_).to_u16_7() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_char16_t_ = std_atomic_char16_t_::new_1({ 3_u16 });
            assert!(
                (((unsafe { atomic_fetch_sub_41((&mut t as *mut std_atomic_char16_t_), 2_u16,) })
                    as i32)
                    == (3_u16 as i32))
            );
            assert!(
                (((unsafe { (t as std___atomic_base_char16_t_).to_u16_8() }) as i32)
                    == (1_u16 as i32))
            );
            const _: () = assert!(true, "");;
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFn_char32_t_ {}
impl TestFn_char32_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut t: std_atomic_char32_t_ = std_atomic_char32_t_::new_1({ 3_u32 });
            assert!(
                ((unsafe { atomic_fetch_sub_42((&mut t as *mut std_atomic_char32_t_), 2_u32,) })
                    == (3_u32))
            );
            assert!(((unsafe { (t as std___atomic_base_char32_t_).to_u32_7() }) == (1_u32)));
            const _: () = assert!(true, "");;
        }
        {
            let mut t: std_atomic_char32_t_ = std_atomic_char32_t_::new_1({ 3_u32 });
            assert!(
                ((unsafe { atomic_fetch_sub_43((&mut t as *mut std_atomic_char32_t_), 2_u32,) })
                    == (3_u32))
            );
            assert!(((unsafe { (t as std___atomic_base_char32_t_).to_u32_8() }) == (1_u32)));
            const _: () = assert!(true, "");;
        }
    }
}
pub unsafe fn testp_44() {
    {
        let mut a: [i32; 3] = [0, <i32>::default(), <i32>::default()];
        let mut t: std_atomic_int_ptr_ =
            std_atomic_int_ptr_::new_1({ (&mut a[(2) as usize] as *mut i32) });
        assert!(
            ((unsafe { atomic_fetch_sub_45((&mut t as *mut std_atomic_int_ptr_), 2_i64,) })
                == (&mut a[(2) as usize] as *mut i32))
        );
        (unsafe { atomic_fetch_sub_45((&mut t as *mut std_atomic_int_ptr_), 0_i64) });
        assert!(((unsafe { t.to_pmuti32_7() }) == (&mut a[(0) as usize] as *mut i32)));
        const _: () = assert!(true, "");;
    }
    {
        let mut a: [i32; 3] = [0, <i32>::default(), <i32>::default()];
        let mut t: std_atomic_int_ptr_ =
            std_atomic_int_ptr_::new_1({ (&mut a[(2) as usize] as *mut i32) });
        assert!(
            ((unsafe { atomic_fetch_sub_46((&mut t as *mut std_atomic_int_ptr_), 2_i64,) })
                == (&mut a[(2) as usize] as *mut i32))
        );
        (unsafe { atomic_fetch_sub_46((&mut t as *mut std_atomic_int_ptr_), 0_i64) });
        assert!(((unsafe { t.to_pmuti32_8() }) == (&mut a[(0) as usize] as *mut i32)));
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testp_47() {
    {
        let a: [i32; 3] = [0, <i32>::default(), <i32>::default()];
        let mut t: std_atomic_const_int_ptr_ =
            std_atomic_const_int_ptr_::new_1({ (&a[(2) as usize] as *const i32) });
        assert!(
            ((unsafe { atomic_fetch_sub_48((&mut t as *mut std_atomic_const_int_ptr_), 2_i64,) })
                == (&a[(2) as usize] as *const i32))
        );
        (unsafe { atomic_fetch_sub_48((&mut t as *mut std_atomic_const_int_ptr_), 0_i64) });
        assert!(((unsafe { t.to_pconsti32_7() }) == (&a[(0) as usize] as *const i32)));
        const _: () = assert!(true, "");;
    }
    {
        let a: [i32; 3] = [0, <i32>::default(), <i32>::default()];
        let mut t: std_atomic_const_int_ptr_ =
            std_atomic_const_int_ptr_::new_1({ (&a[(2) as usize] as *const i32) });
        assert!(
            ((unsafe { atomic_fetch_sub_49((&mut t as *mut std_atomic_const_int_ptr_), 2_i64,) })
                == (&a[(2) as usize] as *const i32))
        );
        (unsafe { atomic_fetch_sub_49((&mut t as *mut std_atomic_const_int_ptr_), 0_i64) });
        assert!(((unsafe { t.to_pconsti32_8() }) == (&a[(0) as usize] as *const i32)));
        const _: () = assert!(true, "");;
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
        TestEachIntegralType_TestFn_::operator_call(&<TestEachIntegralType_TestFn_>::default())
    });
    (unsafe { testp_44() });
    (unsafe { testp_47() });
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
