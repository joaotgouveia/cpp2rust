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
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_15(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_16(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_17(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_18(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_19(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_20(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_21(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_22(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_23(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_24(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_25(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_26(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_27(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_28(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_29(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_30(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_31(
    mut store_op: FnPtr<fn(*const (), *mut (), *mut ())>,
    mut load_op: FnPtr<fn(*const ()) -> *mut ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_32(
    mut store_op: FnPtr<fn(*const (), *const (), *const ())>,
    mut load_op: FnPtr<fn(*const ()) -> *const ()>,
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_float_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_double_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_char_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe {
                            ((*x) as std___atomic_ref_base_signed_char_).load(None)
                        });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe {
                            ((*x) as std___atomic_ref_base_unsigned_char_).load(None)
                        });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_short_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe {
                            ((*x) as std___atomic_ref_base_unsigned_short_).load(None)
                        });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_int_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe {
                            ((*x) as std___atomic_ref_base_unsigned_int_).load(None)
                        });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_long_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe {
                            ((*x) as std___atomic_ref_base_unsigned_long_).load(None)
                        });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_long_long_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe {
                            ((*x) as std___atomic_ref_base_unsigned_long_long_).load(None)
                        });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_wchar_t_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_char8_t_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_char16_t_).load(None) });
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
            let mut x: () = 1_();
            let a: () = unimplemented!();
            let mut y: () = (unimplemented!());
            unimplemented!();
            unimplemented!();
            const _: () = assert!(true, "");;
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_char32_t_).load(None) });
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
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                    <()>::default(),
                ];
                let mut p: *mut () = (&mut t[(1_()) as usize] as *mut ());
                let a: () = unimplemented!();
                let mut y: *mut () = (unimplemented!());
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), *mut (), *mut ())> =
                FnPtr::<fn(*const (), *mut (), *mut ())>::new(
                    |x: *const (), mut old_val: *mut (), mut new_val: *mut ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *mut ()> =
                FnPtr::<fn(*const ()) -> *mut ()>::new(|x: *const ()| -> *mut () {
                    unsafe {
                        return (unsafe { ((*x) as std___atomic_ref_base_int_ptr_).load(None) });
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
                let mut p: *const () = (&t[(1_()) as usize] as *const ());
                let a: () = unimplemented!();
                let mut y: *const () = (unimplemented!());
                unimplemented!();
                unimplemented!();
                const _: () = assert!(true, "");;
            }
        }
        {
            let mut plus_equals: FnPtr<fn(*const (), *const (), *const ())> =
                FnPtr::<fn(*const (), *const (), *const ())>::new(
                    |x: *const (), mut old_val: *const (), mut new_val: *const ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *const ()> =
                FnPtr::<fn(*const ()) -> *const ()>::new(|x: *const ()| -> *const () {
                    unsafe {
                        return (unsafe {
                            ((*x) as std___atomic_ref_base_const_int_ptr_).load(None)
                        });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
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
