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
pub struct TestEachIntegralType_TestFetchAdd_ {}
impl TestEachIntegralType_TestFetchAdd_ {
    pub unsafe fn operator_call(&self) {
        (unsafe { TestFetchAdd_char_::operator_call(&<TestFetchAdd_char_>::default()) });
        (unsafe {
            TestFetchAdd_signed_char_::operator_call(&<TestFetchAdd_signed_char_>::default())
        });
        (unsafe {
            TestFetchAdd_unsigned_char_::operator_call(&<TestFetchAdd_unsigned_char_>::default())
        });
        (unsafe { TestFetchAdd_short_::operator_call(&<TestFetchAdd_short_>::default()) });
        (unsafe {
            TestFetchAdd_unsigned_short_::operator_call(&<TestFetchAdd_unsigned_short_>::default())
        });
        (unsafe { TestFetchAdd_int_::operator_call(&<TestFetchAdd_int_>::default()) });
        (unsafe {
            TestFetchAdd_unsigned_int_::operator_call(&<TestFetchAdd_unsigned_int_>::default())
        });
        (unsafe { TestFetchAdd_long_::operator_call(&<TestFetchAdd_long_>::default()) });
        (unsafe {
            TestFetchAdd_unsigned_long_::operator_call(&<TestFetchAdd_unsigned_long_>::default())
        });
        (unsafe { TestFetchAdd_long_long_::operator_call(&<TestFetchAdd_long_long_>::default()) });
        (unsafe {
            TestFetchAdd_unsigned_long_long_::operator_call(
                &<TestFetchAdd_unsigned_long_long_>::default(),
            )
        });
        (unsafe { TestFetchAdd_wchar_t_::operator_call(&<TestFetchAdd_wchar_t_>::default()) });
        (unsafe { TestFetchAdd_char8_t_::operator_call(&<TestFetchAdd_char8_t_>::default()) });
        (unsafe { TestFetchAdd_char16_t_::operator_call(&<TestFetchAdd_char16_t_>::default()) });
        (unsafe { TestFetchAdd_char32_t_::operator_call(&<TestFetchAdd_char32_t_>::default()) });
        (unsafe {
            TestFetchAdd_signed_char_::operator_call(&<TestFetchAdd_signed_char_>::default())
        });
        (unsafe {
            TestFetchAdd_unsigned_char_::operator_call(&<TestFetchAdd_unsigned_char_>::default())
        });
        (unsafe { TestFetchAdd_short_::operator_call(&<TestFetchAdd_short_>::default()) });
        (unsafe {
            TestFetchAdd_unsigned_short_::operator_call(&<TestFetchAdd_unsigned_short_>::default())
        });
        (unsafe { TestFetchAdd_int_::operator_call(&<TestFetchAdd_int_>::default()) });
        (unsafe {
            TestFetchAdd_unsigned_int_::operator_call(&<TestFetchAdd_unsigned_int_>::default())
        });
        (unsafe { TestFetchAdd_long_::operator_call(&<TestFetchAdd_long_>::default()) });
        (unsafe {
            TestFetchAdd_unsigned_long_::operator_call(&<TestFetchAdd_unsigned_long_>::default())
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestEachPointerType_TestFetchAdd_ {}
impl TestEachPointerType_TestFetchAdd_ {
    pub unsafe fn operator_call(&self) {
        (unsafe { TestFetchAdd_int_ptr_::operator_call(&<TestFetchAdd_int_ptr_>::default()) });
        (unsafe {
            TestFetchAdd_const_int_ptr_::operator_call(&<TestFetchAdd_const_int_ptr_>::default())
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
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_32(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_33(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_34(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_35(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_36(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_37(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_38(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_39(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_40(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_41(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_42(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_43(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_44(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_45(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_46(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_47(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_48(
    mut store_op: FnPtr<fn(*const (), *mut (), *mut ())>,
    mut load_op: FnPtr<fn(*const ()) -> *mut ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_49(
    mut store_op: FnPtr<fn(*const (), *mut (), *mut ())>,
    mut load_op: FnPtr<fn(*const ()) -> *mut ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_50(
    mut store_op: FnPtr<fn(*const (), *const (), *const ())>,
    mut load_op: FnPtr<fn(*const ()) -> *const ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_51(
    mut store_op: FnPtr<fn(*const (), *const (), *const ())>,
    mut load_op: FnPtr<fn(*const ()) -> *const ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_52(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_53(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_54(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_55(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_56(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_57(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_58(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_59(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_60(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_61(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_62(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_63(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_64(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_65(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_66(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_67(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_68(
    mut store_op: FnPtr<fn(*const (), (), ())>,
    mut load_op: FnPtr<fn(*const ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_69(
    mut store_op: FnPtr<fn(*const (), *mut (), *mut ())>,
    mut load_op: FnPtr<fn(*const ()) -> *mut ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_70(
    mut store_op: FnPtr<fn(*const (), *const (), *const ())>,
    mut load_op: FnPtr<fn(*const ()) -> *const ()>,
) {
    &(store_op);
    &(load_op);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchAdd_bool_ {}
impl TestDoesNotHaveFetchAdd_bool_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_add<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchAdd_UserAtomicType_ {}
impl TestDoesNotHaveFetchAdd_UserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_add<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestDoesNotHaveFetchAdd_LargeUserAtomicType_ {}
impl TestDoesNotHaveFetchAdd_LargeUserAtomicType_ {
    pub unsafe fn operator_call(&self) {
        const _: () = assert!(!(false), "!has_fetch_add<std::atomic_ref<T>>");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_float_ {}
impl TestFetchAdd_float_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_67(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_44(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_45(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_double_ {}
impl TestFetchAdd_double_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_68(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_46(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_47(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_char_ {}
impl TestFetchAdd_char_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_52(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_14(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_15(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_signed_char_ {}
impl TestFetchAdd_signed_char_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_53(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_16(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_17(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_unsigned_char_ {}
impl TestFetchAdd_unsigned_char_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_54(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_18(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_19(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_short_ {}
impl TestFetchAdd_short_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_55(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_20(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_21(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_unsigned_short_ {}
impl TestFetchAdd_unsigned_short_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_56(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_22(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_23(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_int_ {}
impl TestFetchAdd_int_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_57(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_24(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_25(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_unsigned_int_ {}
impl TestFetchAdd_unsigned_int_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_58(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_26(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_27(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_long_ {}
impl TestFetchAdd_long_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_59(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_28(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_29(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_unsigned_long_ {}
impl TestFetchAdd_unsigned_long_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_60(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_30(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_31(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_long_long_ {}
impl TestFetchAdd_long_long_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_61(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_32(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_33(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_unsigned_long_long_ {}
impl TestFetchAdd_unsigned_long_long_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_62(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_34(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_35(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_wchar_t_ {}
impl TestFetchAdd_wchar_t_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_63(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_36(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_37(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_char8_t_ {}
impl TestFetchAdd_char8_t_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_64(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_38(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_39(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_char16_t_ {}
impl TestFetchAdd_char16_t_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_65(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_40(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_41(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_char32_t_ {}
impl TestFetchAdd_char32_t_ {
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
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), (), ())> = FnPtr::<fn(*const (), (), ())>::new(
                |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_66(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), (), ())> =
                FnPtr::<fn(*const (), (), ())>::new(
                    |x: *const (), mut old_val: (), mut new_val: ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> ()> =
                FnPtr::<fn(*const ()) -> ()>::new(|x: *const ()| -> () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_42(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_43(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_int_ptr_ {}
impl TestFetchAdd_int_ptr_ {
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
            }
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), *mut (), *mut ())> =
                FnPtr::<fn(*const (), *mut (), *mut ())>::new(
                    |x: *const (), mut old_val: *mut (), mut new_val: *mut ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *mut ()> =
                FnPtr::<fn(*const ()) -> *mut ()>::new(|x: *const ()| -> *mut () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_69(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), *mut (), *mut ())> =
                FnPtr::<fn(*const (), *mut (), *mut ())>::new(
                    |x: *const (), mut old_val: *mut (), mut new_val: *mut ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), *mut (), *mut ())> =
                FnPtr::<fn(*const (), *mut (), *mut ())>::new(
                    |x: *const (), mut old_val: *mut (), mut new_val: *mut ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *mut ()> =
                FnPtr::<fn(*const ()) -> *mut ()>::new(|x: *const ()| -> *mut () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_48(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_49(fetch_add_with_order.copy_from(), load.copy_from()) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFetchAdd_const_int_ptr_ {}
impl TestFetchAdd_const_int_ptr_ {
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
            }
        }
        {
            let mut fetch_add: FnPtr<fn(*const (), *const (), *const ())> =
                FnPtr::<fn(*const (), *const (), *const ())>::new(
                    |x: *const (), mut old_val: *const (), mut new_val: *const ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *const ()> =
                FnPtr::<fn(*const ()) -> *const ()>::new(|x: *const ()| -> *const () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_acquire_release_70(fetch_add.copy_from(), load.copy_from()) });
        }
        {
            let mut fetch_add_no_arg: FnPtr<fn(*const (), *const (), *const ())> =
                FnPtr::<fn(*const (), *const (), *const ())>::new(
                    |x: *const (), mut old_val: *const (), mut new_val: *const ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut fetch_add_with_order: FnPtr<fn(*const (), *const (), *const ())> =
                FnPtr::<fn(*const (), *const (), *const ())>::new(
                    |x: *const (), mut old_val: *const (), mut new_val: *const ()| unsafe {
                        unimplemented!();
                    },
                );
            let mut load: FnPtr<fn(*const ()) -> *const ()> =
                FnPtr::<fn(*const ()) -> *const ()>::new(|x: *const ()| -> *const () {
                    unsafe {
                        return unimplemented!();
                    }
                });
            (unsafe { test_seq_cst_50(fetch_add_no_arg.copy_from(), load.copy_from()) });
            (unsafe { test_seq_cst_51(fetch_add_with_order.copy_from(), load.copy_from()) });
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
        TestEachIntegralType_TestFetchAdd_::operator_call(
            &<TestEachIntegralType_TestFetchAdd_>::default(),
        )
    });
    (unsafe { TestFetchAdd_float_::operator_call(&<TestFetchAdd_float_>::default()) });
    (unsafe { TestFetchAdd_double_::operator_call(&<TestFetchAdd_double_>::default()) });
    (unsafe {
        TestEachPointerType_TestFetchAdd_::operator_call(
            &<TestEachPointerType_TestFetchAdd_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveFetchAdd_bool_::operator_call(&<TestDoesNotHaveFetchAdd_bool_>::default())
    });
    (unsafe {
        TestDoesNotHaveFetchAdd_UserAtomicType_::operator_call(
            &<TestDoesNotHaveFetchAdd_UserAtomicType_>::default(),
        )
    });
    (unsafe {
        TestDoesNotHaveFetchAdd_LargeUserAtomicType_::operator_call(
            &<TestDoesNotHaveFetchAdd_LargeUserAtomicType_>::default(),
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
