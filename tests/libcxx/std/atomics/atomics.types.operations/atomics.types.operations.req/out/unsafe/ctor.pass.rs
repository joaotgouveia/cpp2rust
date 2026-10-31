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
pub struct TestEachIntegralType_TestFunc_ {}
impl TestEachIntegralType_TestFunc_ {
    pub unsafe fn operator_call(&self) {
        (unsafe { TestFunc_char_::operator_call(&<TestFunc_char_>::default()) });
        (unsafe { TestFunc_signed_char_::operator_call(&<TestFunc_signed_char_>::default()) });
        (unsafe { TestFunc_unsigned_char_::operator_call(&<TestFunc_unsigned_char_>::default()) });
        (unsafe { TestFunc_short_::operator_call(&<TestFunc_short_>::default()) });
        (unsafe {
            TestFunc_unsigned_short_::operator_call(&<TestFunc_unsigned_short_>::default())
        });
        (unsafe { TestFunc_int_::operator_call(&<TestFunc_int_>::default()) });
        (unsafe { TestFunc_unsigned_int_::operator_call(&<TestFunc_unsigned_int_>::default()) });
        (unsafe { TestFunc_long_::operator_call(&<TestFunc_long_>::default()) });
        (unsafe { TestFunc_unsigned_long_::operator_call(&<TestFunc_unsigned_long_>::default()) });
        (unsafe { TestFunc_long_long_::operator_call(&<TestFunc_long_long_>::default()) });
        (unsafe {
            TestFunc_unsigned_long_long_::operator_call(&<TestFunc_unsigned_long_long_>::default())
        });
        (unsafe { TestFunc_wchar_t_::operator_call(&<TestFunc_wchar_t_>::default()) });
        (unsafe { TestFunc_char8_t_::operator_call(&<TestFunc_char8_t_>::default()) });
        (unsafe { TestFunc_char16_t_::operator_call(&<TestFunc_char16_t_>::default()) });
        (unsafe { TestFunc_char32_t_::operator_call(&<TestFunc_char32_t_>::default()) });
        (unsafe { TestFunc_signed_char_::operator_call(&<TestFunc_signed_char_>::default()) });
        (unsafe { TestFunc_unsigned_char_::operator_call(&<TestFunc_unsigned_char_>::default()) });
        (unsafe { TestFunc_short_::operator_call(&<TestFunc_short_>::default()) });
        (unsafe {
            TestFunc_unsigned_short_::operator_call(&<TestFunc_unsigned_short_>::default())
        });
        (unsafe { TestFunc_int_::operator_call(&<TestFunc_int_>::default()) });
        (unsafe { TestFunc_unsigned_int_::operator_call(&<TestFunc_unsigned_int_>::default()) });
        (unsafe { TestFunc_long_::operator_call(&<TestFunc_long_>::default()) });
        (unsafe { TestFunc_unsigned_long_::operator_call(&<TestFunc_unsigned_long_>::default()) });
    }
}
pub unsafe fn operator_eq_14(x: *const UserType, y: *const UserType) -> bool {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct UserType {
    pub i: i32,
}
impl UserType {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { i: 0_i32 };
        this
    }
    pub unsafe fn new_2(mut d: i32) -> Self {
        let mut this = Self { i: d };
        this
    }
}
impl std::cmp::PartialEq for UserType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_14(self as *const UserType, other as *const UserType) }
    }
}
impl std::cmp::Eq for UserType {}
impl Default for UserType {
    fn default() -> Self {
        unsafe { UserType::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_UserType_ {}
impl TestFunc_UserType_ {
    pub unsafe fn operator_call(&self) {
        let t: UserType = UserType::new_2({ 42 });
        {
            let a: () = unimplemented!();
            assert!(
                (unsafe {
                    let mut _x: UserType = unimplemented!();
                    operator_eq_14(&mut _x, &t)
                })
            );
        }
        {
            let a: () = unimplemented!();
            assert!(
                (unsafe {
                    let mut _x: UserType = unimplemented!();
                    operator_eq_14(&mut _x, &t)
                })
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_char_ {}
impl TestFunc_char_ {
    pub unsafe fn operator_call(&self) {
        let t: libc::c_char = (42 as libc::c_char);
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_signed_char_ {}
impl TestFunc_signed_char_ {
    pub unsafe fn operator_call(&self) {
        let t: i8 = 42_i8;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_unsigned_char_ {}
impl TestFunc_unsigned_char_ {
    pub unsafe fn operator_call(&self) {
        let t: u8 = 42_u8;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_short_ {}
impl TestFunc_short_ {
    pub unsafe fn operator_call(&self) {
        let t: i16 = 42_i16;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_unsigned_short_ {}
impl TestFunc_unsigned_short_ {
    pub unsafe fn operator_call(&self) {
        let t: u16 = 42_u16;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_int_ {}
impl TestFunc_int_ {
    pub unsafe fn operator_call(&self) {
        let t: i32 = 42;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_unsigned_int_ {}
impl TestFunc_unsigned_int_ {
    pub unsafe fn operator_call(&self) {
        let t: u32 = 42_u32;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_long_ {}
impl TestFunc_long_ {
    pub unsafe fn operator_call(&self) {
        let t: i64 = 42_i64;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_unsigned_long_ {}
impl TestFunc_unsigned_long_ {
    pub unsafe fn operator_call(&self) {
        let t: u64 = 42_u64;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_long_long_ {}
impl TestFunc_long_long_ {
    pub unsafe fn operator_call(&self) {
        let t: i64 = 42_i64;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_unsigned_long_long_ {}
impl TestFunc_unsigned_long_long_ {
    pub unsafe fn operator_call(&self) {
        let t: u64 = 42_u64;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!()) == (t)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_wchar_t_ {}
impl TestFunc_wchar_t_ {
    pub unsafe fn operator_call(&self) {
        let t: i32 = 42_i32;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_char8_t_ {}
impl TestFunc_char8_t_ {
    pub unsafe fn operator_call(&self) {
        let t: u8 = 42_u8;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_char16_t_ {}
impl TestFunc_char16_t_ {
    pub unsafe fn operator_call(&self) {
        let t: u16 = 42_u16;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as i32) == (t as i32)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFunc_char32_t_ {}
impl TestFunc_char32_t_ {
    pub unsafe fn operator_call(&self) {
        let t: u32 = 42_u32;
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as u32) == (t)));
        }
        {
            let a: () = unimplemented!();
            assert!(((unimplemented!() as u32) == (t)));
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
    (unsafe { TestFunc_UserType_::operator_call(&<TestFunc_UserType_>::default()) });
    (unsafe {
        TestEachIntegralType_TestFunc_::operator_call(&<TestEachIntegralType_TestFunc_>::default())
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
