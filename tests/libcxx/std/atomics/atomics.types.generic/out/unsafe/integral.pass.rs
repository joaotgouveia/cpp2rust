extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn cmpxchg_weak_loop_0(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char__false_).compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_1(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char__false_).compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_2(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_signed_char__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_3(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_signed_char__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_4(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_char__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_5(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_char__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_6(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_short__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_7(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_short__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_8(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_short__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_9(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_short__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_10(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_int__false_).compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_11(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_int__false_).compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_12(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_int__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_13(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_int__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_14(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_long__false_).compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_15(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_long__false_).compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_16(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_long__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_17(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_long__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_18(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_long_long__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_19(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_long_long__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_20(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_long_long__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_21(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_unsigned_long_long__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_22(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char8_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_23(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char8_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_24(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char16_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_25(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char16_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_26(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char32_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_27(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_char32_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_28(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_wchar_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn cmpxchg_weak_loop_29(atomic: *mut (), expected: *mut (), mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        if ((unsafe {
            let ___e: *mut () = expected;
            let ___d: () = desired;
            ((*atomic) as std___atomic_base_wchar_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == (true))
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub unsafe fn do_test_30() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_32() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_char__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_33() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_signed_char__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_signed_char__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_signed_char__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_34() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_signed_char__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_signed_char__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_signed_char__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_signed_char__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_35() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_char__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_char__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_char__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_36() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_unsigned_char__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_char__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_char__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_char__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_37() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_short__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_short__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_short__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 2] = [23_(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_38() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_short__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_short__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_short__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_short__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 2] = [23_(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_39() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_short__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_short__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_short__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 2] = [23_(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_40() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_unsigned_short__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_short__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_short__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_short__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 2] = [23_(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_41() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_int__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_int__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_int__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_42() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = (unsafe { (obj as std___atomic_base_int__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_int__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_int__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_int__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_43() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_int__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_int__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_int__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_44() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_unsigned_int__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_int__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_int__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_int__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_45() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_long__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_long__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_long__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_46() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_long__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_long__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_long__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_long__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_47() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_long__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_long__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_long__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_48() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_unsigned_long__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_long__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_long__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_long__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_49() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_long_long__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_long_long__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_long_long__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_50() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_long_long__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_long_long__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_long_long__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_long_long__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_51() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_long_long__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_long_long__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_long_long__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_52() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_unsigned_long_long__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_unsigned_long_long__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_unsigned_long_long__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_unsigned_long_long__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 8] = [
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_53() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char8_t__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char8_t__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char8_t__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_54() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_char8_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char8_t__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char8_t__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char8_t__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 1] = [23_()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_55() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char16_t__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char16_t__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char16_t__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 2] = [23_(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_56() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_char16_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char16_t__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char16_t__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char16_t__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 2] = [23_(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_57() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char32_t__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char32_t__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char32_t__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_58() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_char32_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_char32_t__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_char32_t__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_char32_t__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_59() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_wchar_t__false_).store_4(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_wchar_t__false_).store_4(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_wchar_t__false_).to__8() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn do_test_60() {
    let mut obj: () = unimplemented!();
    unimplemented!();
    {
        let mut lockfree: () =
            (unsafe { (obj as std___atomic_base_wchar_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    (unsafe { (obj as std___atomic_base_wchar_t__false_).store_3(0_(), None) });
    unimplemented!();
    (unsafe {
        (obj as std___atomic_base_wchar_t__false_).store_3(
            1_(),
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut memory_order_release_31))),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = (unsafe { (obj as std___atomic_base_wchar_t__false_).to__7() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    x = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    obj = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut storage: [(); 4] = [23_(), unimplemented!(), unimplemented!(), unimplemented!()];
        let zero: *mut () = &mut (*(Box::leak(Box::new(unimplemented!())) as *mut ()));
        unimplemented!();
    }
}
pub unsafe fn test_61() {
    (unsafe { do_test_30() });
    (unsafe { do_test_32() });
}
pub unsafe fn test_62() {
    (unsafe { do_test_33() });
    (unsafe { do_test_34() });
}
pub unsafe fn test_63() {
    (unsafe { do_test_35() });
    (unsafe { do_test_36() });
}
pub unsafe fn test_64() {
    (unsafe { do_test_37() });
    (unsafe { do_test_38() });
}
pub unsafe fn test_65() {
    (unsafe { do_test_39() });
    (unsafe { do_test_40() });
}
pub unsafe fn test_66() {
    (unsafe { do_test_41() });
    (unsafe { do_test_42() });
}
pub unsafe fn test_67() {
    (unsafe { do_test_43() });
    (unsafe { do_test_44() });
}
pub unsafe fn test_68() {
    (unsafe { do_test_45() });
    (unsafe { do_test_46() });
}
pub unsafe fn test_69() {
    (unsafe { do_test_47() });
    (unsafe { do_test_48() });
}
pub unsafe fn test_70() {
    (unsafe { do_test_49() });
    (unsafe { do_test_50() });
}
pub unsafe fn test_71() {
    (unsafe { do_test_51() });
    (unsafe { do_test_52() });
}
pub unsafe fn test_72() {
    (unsafe { do_test_53() });
    (unsafe { do_test_54() });
}
pub unsafe fn test_73() {
    (unsafe { do_test_55() });
    (unsafe { do_test_56() });
}
pub unsafe fn test_74() {
    (unsafe { do_test_57() });
    (unsafe { do_test_58() });
}
pub unsafe fn test_75() {
    (unsafe { do_test_59() });
    (unsafe { do_test_60() });
}
pub unsafe fn test_76() {
    (unsafe { do_test_32() });
    (unsafe { do_test_32() });
}
pub unsafe fn test_77() {
    (unsafe { do_test_34() });
    (unsafe { do_test_34() });
}
pub unsafe fn test_78() {
    (unsafe { do_test_36() });
    (unsafe { do_test_36() });
}
pub unsafe fn test_79() {
    (unsafe { do_test_38() });
    (unsafe { do_test_38() });
}
pub unsafe fn test_80() {
    (unsafe { do_test_40() });
    (unsafe { do_test_40() });
}
pub unsafe fn test_81() {
    (unsafe { do_test_42() });
    (unsafe { do_test_42() });
}
pub unsafe fn test_82() {
    (unsafe { do_test_44() });
    (unsafe { do_test_44() });
}
pub unsafe fn test_83() {
    (unsafe { do_test_46() });
    (unsafe { do_test_46() });
}
pub unsafe fn test_84() {
    (unsafe { do_test_48() });
    (unsafe { do_test_48() });
}
pub unsafe fn test_85() {
    (unsafe { do_test_50() });
    (unsafe { do_test_50() });
}
pub unsafe fn test_86() {
    (unsafe { do_test_52() });
    (unsafe { do_test_52() });
}
pub unsafe fn test_87() {
    (unsafe { do_test_56() });
    (unsafe { do_test_56() });
}
pub unsafe fn test_88() {
    (unsafe { do_test_58() });
    (unsafe { do_test_58() });
}
pub unsafe fn test_89() {
    (unsafe { do_test_60() });
    (unsafe { do_test_60() });
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
    (unsafe { test_61() });
    (unsafe { test_62() });
    (unsafe { test_63() });
    (unsafe { test_64() });
    (unsafe { test_65() });
    (unsafe { test_66() });
    (unsafe { test_67() });
    (unsafe { test_68() });
    (unsafe { test_69() });
    (unsafe { test_70() });
    (unsafe { test_71() });
    (unsafe { test_72() });
    (unsafe { test_73() });
    (unsafe { test_74() });
    (unsafe { test_75() });
    (unsafe { test_62() });
    (unsafe { test_63() });
    (unsafe { test_64() });
    (unsafe { test_65() });
    (unsafe { test_66() });
    (unsafe { test_67() });
    (unsafe { test_68() });
    (unsafe { test_69() });
    (unsafe { test_76() });
    (unsafe { test_77() });
    (unsafe { test_78() });
    (unsafe { test_79() });
    (unsafe { test_80() });
    (unsafe { test_81() });
    (unsafe { test_82() });
    (unsafe { test_83() });
    (unsafe { test_84() });
    (unsafe { test_85() });
    (unsafe { test_86() });
    (unsafe { test_87() });
    (unsafe { test_88() });
    (unsafe { test_89() });
    (unsafe { test_77() });
    (unsafe { test_78() });
    (unsafe { test_79() });
    (unsafe { test_80() });
    (unsafe { test_81() });
    (unsafe { test_82() });
    (unsafe { test_83() });
    (unsafe { test_84() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
