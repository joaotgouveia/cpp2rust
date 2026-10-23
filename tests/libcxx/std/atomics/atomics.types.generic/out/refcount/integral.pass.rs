extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn cmpxchg_weak_loop_0(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_1(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_2(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_signed_char__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_3(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_signed_char__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_4(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_char__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_5(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_char__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_6(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_short__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_7(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_short__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_8(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_short__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_9(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_short__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_10(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_int__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_11(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_int__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_12(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_int__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_13(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_int__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_14(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_long__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_15(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_long__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_16(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_long__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_17(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_long__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_18(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_long_long__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_19(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_long_long__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_20(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_long_long__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_21(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_unsigned_long_long__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_22(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char8_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_23(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char8_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_24(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char16_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_25(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char16_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_26(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char32_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_27(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_char32_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_28(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_wchar_t__false_)
                .compare_exchange_weak_16(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn cmpxchg_weak_loop_29(atomic: Ptr<()>, expected: Ptr<()>, mut desired: ()) -> () {
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        if (({
            let ___e: Ptr<()> = (expected).clone();
            let ___d: () = desired;
            ((*atomic.upgrade().deref()) as std___atomic_base_wchar_t__false_)
                .compare_exchange_weak_15(___e, ___d, None)
        }) == true)
        {
            return true;
        }
        i.postfix_inc();
    }
    return false;
}
pub fn do_test_30() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_32() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_char__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_33() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_signed_char__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_signed_char__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_signed_char__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_34() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_signed_char__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_signed_char__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_signed_char__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_signed_char__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_35() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_char__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_char__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_char__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_36() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_unsigned_char__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_char__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_char__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_char__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_37() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_short__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_short__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_short__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_(), unimplemented!()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_38() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_short__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_short__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_short__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_short__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_(), unimplemented!()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_39() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_short__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_short__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_short__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_(), unimplemented!()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_40() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_unsigned_short__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_short__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_short__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_short__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_(), unimplemented!()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_41() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_int__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_int__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_int__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_42() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_int__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_int__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_int__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_int__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_43() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_int__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_int__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_int__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_44() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_unsigned_int__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_int__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_int__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_int__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_45() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_long__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_long__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_long__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_46() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_long__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_long__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_long__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_long__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_47() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_long__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_long__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_long__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_48() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_unsigned_long__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_long__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_long__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_long__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_49() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_long_long__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_long_long__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_long_long__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_50() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_long_long__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_long_long__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_long_long__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_long_long__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_51() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_long_long__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_long_long__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_long_long__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_52() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = ({
            ((*obj.borrow()) as std___atomic_base_unsigned_long_long__false_).is_lock_free_1()
        });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_unsigned_long_long__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_unsigned_long_long__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_unsigned_long_long__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_53() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char8_t__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char8_t__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char8_t__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_54() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_char8_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char8_t__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char8_t__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char8_t__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_55() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char16_t__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char16_t__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char16_t__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_(), unimplemented!()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_56() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_char16_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char16_t__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char16_t__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char16_t__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([23_(), unimplemented!()])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_57() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char32_t__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char32_t__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char32_t__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_58() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_char32_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_char32_t__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_char32_t__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_char32_t__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_59() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () = unimplemented!();
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_wchar_t__false_).store_4(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_wchar_t__false_)
            .store_4(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_wchar_t__false_).to__8() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn do_test_60() {
    let obj: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let mut lockfree: () =
            ({ ((*obj.borrow()) as std___atomic_base_wchar_t__false_).is_lock_free_1() });
        &(lockfree);
        {
            unimplemented!();
        }
    }
    ({ ((*obj.borrow()) as std___atomic_base_wchar_t__false_).store_3(0_(), None) });
    unimplemented!();
    ({
        ((*obj.borrow()) as std___atomic_base_wchar_t__false_)
            .store_3(1_(), Some(memory_order_release_31.with(|rc| *rc.borrow())))
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(
        ({ ((*obj.borrow()) as std___atomic_base_wchar_t__false_).to__7() }),
    ));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*x.borrow_mut()) = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*obj.borrow_mut()) = 2_();
    unimplemented!();
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
        let storage: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
            23_(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ])));
        let zero: Ptr<()> = Ptr::alloc(Rc::new(RefCell::new(unimplemented!())).as_pointer());
        unimplemented!();
    }
}
pub fn test_61() {
    ({ do_test_30() });
    ({ do_test_32() });
}
pub fn test_62() {
    ({ do_test_33() });
    ({ do_test_34() });
}
pub fn test_63() {
    ({ do_test_35() });
    ({ do_test_36() });
}
pub fn test_64() {
    ({ do_test_37() });
    ({ do_test_38() });
}
pub fn test_65() {
    ({ do_test_39() });
    ({ do_test_40() });
}
pub fn test_66() {
    ({ do_test_41() });
    ({ do_test_42() });
}
pub fn test_67() {
    ({ do_test_43() });
    ({ do_test_44() });
}
pub fn test_68() {
    ({ do_test_45() });
    ({ do_test_46() });
}
pub fn test_69() {
    ({ do_test_47() });
    ({ do_test_48() });
}
pub fn test_70() {
    ({ do_test_49() });
    ({ do_test_50() });
}
pub fn test_71() {
    ({ do_test_51() });
    ({ do_test_52() });
}
pub fn test_72() {
    ({ do_test_53() });
    ({ do_test_54() });
}
pub fn test_73() {
    ({ do_test_55() });
    ({ do_test_56() });
}
pub fn test_74() {
    ({ do_test_57() });
    ({ do_test_58() });
}
pub fn test_75() {
    ({ do_test_59() });
    ({ do_test_60() });
}
pub fn test_76() {
    ({ do_test_32() });
    ({ do_test_32() });
}
pub fn test_77() {
    ({ do_test_34() });
    ({ do_test_34() });
}
pub fn test_78() {
    ({ do_test_36() });
    ({ do_test_36() });
}
pub fn test_79() {
    ({ do_test_38() });
    ({ do_test_38() });
}
pub fn test_80() {
    ({ do_test_40() });
    ({ do_test_40() });
}
pub fn test_81() {
    ({ do_test_42() });
    ({ do_test_42() });
}
pub fn test_82() {
    ({ do_test_44() });
    ({ do_test_44() });
}
pub fn test_83() {
    ({ do_test_46() });
    ({ do_test_46() });
}
pub fn test_84() {
    ({ do_test_48() });
    ({ do_test_48() });
}
pub fn test_85() {
    ({ do_test_50() });
    ({ do_test_50() });
}
pub fn test_86() {
    ({ do_test_52() });
    ({ do_test_52() });
}
pub fn test_87() {
    ({ do_test_56() });
    ({ do_test_56() });
}
pub fn test_88() {
    ({ do_test_58() });
    ({ do_test_58() });
}
pub fn test_89() {
    ({ do_test_60() });
    ({ do_test_60() });
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_61() });
    ({ test_62() });
    ({ test_63() });
    ({ test_64() });
    ({ test_65() });
    ({ test_66() });
    ({ test_67() });
    ({ test_68() });
    ({ test_69() });
    ({ test_70() });
    ({ test_71() });
    ({ test_72() });
    ({ test_73() });
    ({ test_74() });
    ({ test_75() });
    ({ test_62() });
    ({ test_63() });
    ({ test_64() });
    ({ test_65() });
    ({ test_66() });
    ({ test_67() });
    ({ test_68() });
    ({ test_69() });
    ({ test_76() });
    ({ test_77() });
    ({ test_78() });
    ({ test_79() });
    ({ test_80() });
    ({ test_81() });
    ({ test_82() });
    ({ test_83() });
    ({ test_84() });
    ({ test_85() });
    ({ test_86() });
    ({ test_87() });
    ({ test_88() });
    ({ test_89() });
    ({ test_77() });
    ({ test_78() });
    ({ test_79() });
    ({ test_80() });
    ({ test_81() });
    ({ test_82() });
    ({ test_83() });
    ({ test_84() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
