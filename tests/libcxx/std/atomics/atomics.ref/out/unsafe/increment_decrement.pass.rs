extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type LockFreeStatus = ();
pub const LockFreeStatus_unknown: LockFreeStatus = -1;
pub const LockFreeStatus_never: LockFreeStatus = 0;
pub const LockFreeStatus_sometimes: LockFreeStatus = 1;
pub const LockFreeStatus_always: LockFreeStatus = 2;
pub static mut value_0: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_0)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_char_ {}
pub static mut value_2: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_2)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_short_ {}
pub static mut value_4: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_4)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_int_ {}
pub static mut value_6: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_6)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_long_ {}
pub static mut value_8: std::cell::LazyCell<LockFreeStatus> = std::cell::LazyCell::new(|| unsafe {
    ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
    {
        2_()
    } else {
        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            -1_()
                        })
                    })
                })
            })
        })
    }) as LockFreeStatus)
});
pub static mut status_known_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    ((*std::cell::LazyCell::force_mut(&mut *&raw mut value_8)) != (LockFreeStatus_unknown))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LockFreeStatusInfo_void_ptr_ {}
pub static mut value_10: std::cell::LazyCell<LockFreeStatus> =
    std::cell::LazyCell::new(|| unsafe {
        ((if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
        {
            2_()
        } else {
            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
            {
                2_()
            } else {
                (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                    && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                {
                    2_()
                } else {
                    (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                        && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                    {
                        2_()
                    } else {
                        (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                            && ((::std::mem::align_of::<()>()) >= (::std::mem::size_of::<()>())))
                        {
                            2_()
                        } else {
                            (if (((::std::mem::size_of::<()>()) == (::std::mem::size_of::<()>()))
                                && ((::std::mem::align_of::<()>())
                                    >= (::std::mem::size_of::<()>())))
                            {
                                2_()
                            } else {
                                -1_()
                            })
                        })
                    })
                })
            })
        }) as LockFreeStatus)
    });
pub static mut status_known_11: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
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
pub unsafe fn operator_eq_12(x: *const UserAtomicType, y: *const UserAtomicType) -> () {
    return (((*x).i) == ((*y).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct UserAtomicType {
    pub i: (),
}
impl UserAtomicType {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
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
pub unsafe fn operator_eq_13(x: *const LargeUserAtomicType, y: *const LargeUserAtomicType) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
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
    pub a: [(); 128],
}
impl LargeUserAtomicType {
    pub unsafe fn new(mut d: Option<()>) -> Self {
        let mut d: () = d.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            a: [unimplemented!(); 128],
        };
        'loop_: for e in IterRange::new(
            this.a.as_mut_ptr(),
            this.a.as_mut_ptr().offset((128_()) as isize),
        ) {
            let e: *mut () = &mut (*e);
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
pub unsafe fn does_not_have_increment_nor_decrement_operators_14() -> () {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_15() -> () {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_16() -> () {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_17() -> () {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_18() -> () {
    return (((!(false)) && (!(false))) && (!(false))) && (!(false));
}
pub unsafe fn does_not_have_increment_nor_decrement_operators_19() -> () {
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char_ {}
impl TestIncrementDecrement_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_signed_char_ {}
impl TestIncrementDecrement_signed_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_char_ {}
impl TestIncrementDecrement_unsigned_char_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_short_ {}
impl TestIncrementDecrement_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_short_ {}
impl TestIncrementDecrement_unsigned_short_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_int_ {}
impl TestIncrementDecrement_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_int_ {}
impl TestIncrementDecrement_unsigned_int_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_long_ {}
impl TestIncrementDecrement_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_long_ {}
impl TestIncrementDecrement_unsigned_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_long_long_ {}
impl TestIncrementDecrement_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_unsigned_long_long_ {}
impl TestIncrementDecrement_unsigned_long_long_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_wchar_t_ {}
impl TestIncrementDecrement_wchar_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char8_t_ {}
impl TestIncrementDecrement_char8_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char16_t_ {}
impl TestIncrementDecrement_char16_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestIncrementDecrement_char32_t_ {}
impl TestIncrementDecrement_char32_t_ {
    pub unsafe fn operator_call(&self) {
        {
            let mut x: () = 1_();
            let a: () = unimplemented!();
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
            {
                let mut y: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
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
                let mut p: *mut () = (&mut t[(1_()) as usize] as *mut ());
                let a: () = unimplemented!();
                {
                    let mut y: *mut () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *mut () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *mut () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *mut () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
            }
        }
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
                let mut p: *const () = (&t[(1_()) as usize] as *const ());
                let a: () = unimplemented!();
                {
                    let mut y: *const () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *const () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *const () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
                {
                    let mut y: *const () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    const _: () = assert!(true, "");;
                }
            }
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
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
    return 0_();
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
