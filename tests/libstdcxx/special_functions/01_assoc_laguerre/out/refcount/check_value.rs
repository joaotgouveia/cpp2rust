extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(128)]
pub struct __gnu_test_func_callback {
    #[offset(0)]
    _M_size: (),
    #[offset(8)]
    #[byte_size(120)]
    _M_tests: Box<[FnPtr<fn()>]>,
}
impl __gnu_test_func_callback {
    pub fn new() -> Self {
        Self {
            _M_size: 0_(),
            _M_tests: (0..15)
                .map(|_| FnPtr::<fn()>::null())
                .collect::<Box<[FnPtr<fn()>]>>(),
        }
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        { __gnu_test_func_callback::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub fn new() -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<__gnu_test_object_counter>) -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_object_counter> =
            Rc::new(RefCell::new(__gnu_test_object_counter {}));
        __gnu_test_object_counter::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        { __gnu_test_object_counter::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub fn count() -> () {
        return count__1.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__1.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__1.with(|rc| *rc.borrow()) == throw_on__2.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__1.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__2.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__2.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub fn count() -> () {
        return count__3.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__3.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__3.with(|rc| *rc.borrow()) == throw_on__4.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__3.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__4.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__4.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> () {
        return _M_count_5.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_5.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_5.with(|rc| *rc.borrow_mut() = 0_());
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct __gnu_test_copy_tracker {
    #[offset(0)]
    id_: (),
    #[offset(4)]
    throw_on_copy_: (),
}
impl __gnu_test_copy_tracker {
    pub fn new(id: Option<()>, throw_on_copy: Option<()>) -> Self {
        let mut id: () =
            id.unwrap_or_else(|| (*next_id__6.with(Value::clone).borrow_mut()).postfix_dec());
        let mut throw_on_copy: () = throw_on_copy.unwrap_or_else(|| false);
        Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        }
    }
    pub fn copy_from(rhs: Ptr<__gnu_test_copy_tracker>) -> Self {
        let __this: Value<__gnu_test_copy_tracker> = Rc::new(RefCell::new(Self {
            id_: ({ __gnu_test_copy_trackerImpl::id(&rhs) }),
            throw_on_copy_: rhs.with(|__s| __s.throw_on_copy_),
        }));
        let this: Ptr<__gnu_test_copy_tracker> = __this.as_pointer();
        if this.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_copy_constructor::throw_on(
                    ({ __gnu_test_copy_constructor::count() }).wrapping_add(1_()),
                )
            });
        }
        ({ __gnu_test_copy_constructor::mark_call() });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn reset() {
        ({ __gnu_test_copy_constructor::reset() });
        ({ __gnu_test_assignment_operator::reset() });
        ({ __gnu_test_destructor::reset() });
    }
}
impl CppLess for __gnu_test_copy_tracker {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for __gnu_test_copy_tracker {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_copy_tracker> =
            Rc::new(RefCell::new(__gnu_test_copy_tracker {
                id_: self.id_.clone(),
                throw_on_copy_: self.throw_on_copy_.clone(),
            }));
        __gnu_test_copy_tracker::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub fn operator_eq_8(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_7(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } < {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_semaphore {
    #[offset(0)]
    sem_set_: (),
    #[offset(4)]
    pid_: (),
}
impl __gnu_test_semaphore {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct testcase_assoc_laguerre_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub n: (),
    #[offset(12)]
    pub m: (),
    #[offset(16)]
    pub x: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data001_9: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data009_25: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -89_f64,
                n: 1_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -99_f64,
                n: 1_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_26: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data010_27: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 2_f64,
                n: 1_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8_f64,
                n: 1_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18_f64,
                n: 1_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -28_f64,
                n: 1_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -38_f64,
                n: 1_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48_f64,
                n: 1_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -58_f64,
                n: 1_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -68_f64,
                n: 1_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -78_f64,
                n: 1_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -88_f64,
                n: 1_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -98_f64,
                n: 1_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_28: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data011_29: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 3_f64,
                n: 1_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7_f64,
                n: 1_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -17_f64,
                n: 1_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -27_f64,
                n: 1_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -37_f64,
                n: 1_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -47_f64,
                n: 1_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -57_f64,
                n: 1_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -67_f64,
                n: 1_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -77_f64,
                n: 1_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -87_f64,
                n: 1_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -97_f64,
                n: 1_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_30: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data012_31: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 6_f64,
                n: 1_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4_f64,
                n: 1_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14_f64,
                n: 1_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -24_f64,
                n: 1_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -34_f64,
                n: 1_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -44_f64,
                n: 1_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -54_f64,
                n: 1_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -64_f64,
                n: 1_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -74_f64,
                n: 1_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -84_f64,
                n: 1_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -94_f64,
                n: 1_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_32: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data013_33: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -89_f64,
                n: 1_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_34: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data014_35: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 1_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_36: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data015_37: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 51_f64,
                n: 1_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 41_f64,
                n: 1_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31_f64,
                n: 1_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 1_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler015_38: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data016_39: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 101_f64,
                n: 1_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 91_f64,
                n: 1_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 81_f64,
                n: 1_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 71_f64,
                n: 1_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 61_f64,
                n: 1_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 51_f64,
                n: 1_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 41_f64,
                n: 1_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31_f64,
                n: 1_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 1_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler016_40: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data017_41: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 2_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31_f64,
                n: 2_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 161_f64,
                n: 2_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 391_f64,
                n: 2_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 721_f64,
                n: 2_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1151_f64,
                n: 2_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1681_f64,
                n: 2_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2311_f64,
                n: 2_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3041_f64,
                n: 2_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3871_f64,
                n: 2_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4801_f64,
                n: 2_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler017_42: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data018_43: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 3_f64,
                n: 2_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 23_f64,
                n: 2_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 143_f64,
                n: 2_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 363_f64,
                n: 2_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 683_f64,
                n: 2_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1103_f64,
                n: 2_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1623_f64,
                n: 2_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2243_f64,
                n: 2_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2963_f64,
                n: 2_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3783_f64,
                n: 2_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4703_f64,
                n: 2_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler018_44: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data019_45: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 6_f64,
                n: 2_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16_f64,
                n: 2_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 126_f64,
                n: 2_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 336_f64,
                n: 2_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 646_f64,
                n: 2_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1056_f64,
                n: 2_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1566_f64,
                n: 2_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2176_f64,
                n: 2_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2886_f64,
                n: 2_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3696_f64,
                n: 2_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4606_f64,
                n: 2_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler019_46: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data020_47: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 2_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 0.9999999999999954_f64,
                n: 2_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 81_f64,
                n: 2_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 261_f64,
                n: 2_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 541_f64,
                n: 2_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 921_f64,
                n: 2_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1401_f64,
                n: 2_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1981_f64,
                n: 2_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2661_f64,
                n: 2_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3441_f64,
                n: 2_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4321_f64,
                n: 2_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler020_48: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data021_49: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 66_f64,
                n: 2_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4.00000000000001_f64,
                n: 2_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 25.999999999999964_f64,
                n: 2_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 156_f64,
                n: 2_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 386_f64,
                n: 2_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 716_f64,
                n: 2_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1146_f64,
                n: 2_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1676_f64,
                n: 2_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2306_f64,
                n: 2_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3036_f64,
                n: 2_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3866_f64,
                n: 2_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler021_50: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data022_51: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 231_f64,
                n: 2_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 61.000000000000206_f64,
                n: 2_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9.000000000000005_f64,
                n: 2_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21.000000000000124_f64,
                n: 2_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 151.00000000000057_f64,
                n: 2_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 381_f64,
                n: 2_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 711_f64,
                n: 2_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1141_f64,
                n: 2_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1670.9999999999998_f64,
                n: 2_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2301_f64,
                n: 2_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3031_f64,
                n: 2_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler022_52: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data023_53: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1326_f64,
                n: 2_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 855.9999999999969_f64,
                n: 2_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 485.99999999999835_f64,
                n: 2_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 215.99999999999937_f64,
                n: 2_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 45.99999999999983_f64,
                n: 2_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -23.999999999999538_f64,
                n: 2_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6.000000000000106_f64,
                n: 2_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 135.99999999999963_f64,
                n: 2_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 365.9999999999989_f64,
                n: 2_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 695.9999999999978_f64,
                n: 2_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1125.9999999999964_f64,
                n: 2_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler023_54: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data024_55: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 5151_f64,
                n: 2_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4181.0000000000655_f64,
                n: 2_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3311.000000000052_f64,
                n: 2_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2541.00000000004_f64,
                n: 2_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1871.000000000029_f64,
                n: 2_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1301.0000000000207_f64,
                n: 2_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 831.0000000000136_f64,
                n: 2_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 461.0000000000068_f64,
                n: 2_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 191.0000000000025_f64,
                n: 2_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21.000000000000046_f64,
                n: 2_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48.999999999999915_f64,
                n: 2_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler024_56: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data025_57: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 5_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 34.33333333333333_f64,
                n: 5_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4765.666666666667_f64,
                n: 5_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -74399_f64,
                n: 5_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -418865.6666666666_f64,
                n: 5_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1498165.6666666665_f64,
                n: 5_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4122299_f64,
                n: 5_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9551265.666666668_f64,
                n: 5_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19595065.666666664_f64,
                n: 5_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -36713699_f64,
                n: 5_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -64117165.666666664_f64,
                n: 5_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler025_58: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data026_59: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 6_f64,
                n: 5_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 22.66666666666666_f64,
                n: 5_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2960.666666666666_f64,
                n: 5_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -58944_f64,
                n: 5_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -357927.33333333326_f64,
                n: 5_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1329910.6666666665_f64,
                n: 5_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3744894_f64,
                n: 5_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8812877.333333332_f64,
                n: 5_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18283860.666666664_f64,
                n: 5_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -34547844_f64,
                n: 5_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -60734827.333333336_f64,
                n: 5_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler026_60: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data027_61: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 5_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4.333333333333334_f64,
                n: 5_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1679_f64,
                n: 5_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -46029_f64,
                n: 5_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -304045.6666666667_f64,
                n: 5_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1176729.0000000002_f64,
                n: 5_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3395079_f64,
                n: 5_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8120095.666666666_f64,
                n: 5_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -17042778.999999996_f64,
                n: 5_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -32484129_f64,
                n: 5_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -57495145.666666664_f64,
                n: 5_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler027_62: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data028_63: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 252_f64,
                n: 5_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14.666666666666654_f64,
                n: 5_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 51.99999999999991_f64,
                n: 5_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19548_f64,
                n: 5_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -178814.6666666666_f64,
                n: 5_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -797747.9999999998_f64,
                n: 5_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2496348_f64,
                n: 5_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6294614.666666666_f64,
                n: 5_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13712547.999999996_f64,
                n: 5_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -26870147.999999993_f64,
                n: 5_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48587414.66666667_f64,
                n: 5_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler028_64: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data029_65: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 3003_f64,
                n: 5_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 19.666666666666668_f64,
                n: 5_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 36.33333333333327_f64,
                n: 5_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1947_f64,
                n: 5_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -60930.333333333314_f64,
                n: 5_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -381913.6666666665_f64,
                n: 5_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1419897_f64,
                n: 5_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3979880.333333333_f64,
                n: 5_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9316863.666666664_f64,
                n: 5_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19235847_f64,
                n: 5_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -36191830.33333333_f64,
                n: 5_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler029_66: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data030_67: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 53130_f64,
                n: 5_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1213.3333333333335_f64,
                n: 5_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 129.99999999999963_f64,
                n: 5_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -119.99999999999974_f64,
                n: 5_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 463.3333333333332_f64,
                n: 5_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48120.000000000015_f64,
                n: 5_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -345870.0000000002_f64,
                n: 5_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1342786.6666666667_f64,
                n: 5_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3838870.000000001_f64,
                n: 5_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9084120_f64,
                n: 5_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18878536.666666668_f64,
                n: 5_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler030_68: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data031_69: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 3478761_f64,
                n: 5_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1154544.3333333335_f64,
                n: 5_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 264661.00000000006_f64,
                n: 5_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 24111.000000000033_f64,
                n: 5_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2105.6666666666665_f64,
                n: 5_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1010.9999999999916_f64,
                n: 5_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1538.9999999999955_f64,
                n: 5_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5244.333333333345_f64,
                n: 5_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13639.000000000011_f64,
                n: 5_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -243189.00000000006_f64,
                n: 5_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1118405.6666666667_f64,
                n: 5_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler031_70: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data032_71: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 96560646_f64,
                n: 5_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 57264262.66666665_f64,
                n: 5_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31841379.333333332_f64,
                n: 5_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16281996_f64,
                n: 5_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7426112.666666667_f64,
                n: 5_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2863729.333333333_f64,
                n: 5_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 834846_f64,
                n: 5_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 129462.66666666663_f64,
                n: 5_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -12420.666666666668_f64,
                n: 5_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -804_f64,
                n: 5_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4312.666666666667_f64,
                n: 5_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler032_72: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data033_73: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 10_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 27.984126984126977_f64,
                n: 10_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3227.8077601410932_f64,
                n: 10_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 15129.571428571455_f64,
                n: 10_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 79724066.60846558_f64,
                n: 10_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2037190065.373898_f64,
                n: 10_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21804200401_f64,
                n: 10_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 144688291819.51855_f64,
                n: 10_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 703324772760.0828_f64,
                n: 10_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2741055412243.857_f64,
                n: 10_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 9051283795429.572_f64,
                n: 10_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler033_74: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data034_75: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 10_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 14.791887125220455_f64,
                n: 10_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2704.6507936507933_f64,
                n: 10_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -182924.71428571423_f64,
                n: 10_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 48066036.749559075_f64,
                n: 10_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1486264192.216931_f64,
                n: 10_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17239562282.428574_f64,
                n: 10_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 119837491630.13579_f64,
                n: 10_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 600681375251.2117_f64,
                n: 10_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2392908405632.4287_f64,
                n: 10_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 8033035722509.237_f64,
                n: 10_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler034_76: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data035_77: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 66_f64,
                n: 10_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14.51146384479718_f64,
                n: 10_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1064.5890652557316_f64,
                n: 10_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -194569.7142857143_f64,
                n: 10_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 27343569.350970022_f64,
                n: 10_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1067807661.6790125_f64,
                n: 10_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 13529451580.285711_f64,
                n: 10_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 98812724224.64194_f64,
                n: 10_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 511482736187.3402_f64,
                n: 10_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2084478393087.4285_f64,
                n: 10_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7117724862237.075_f64,
                n: 10_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler035_78: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data036_79: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 3003_f64,
                n: 10_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11.641975308642031_f64,
                n: 10_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1137.564373897707_f64,
                n: 10_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9254.14285714286_f64,
                n: 10_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2121878.8377425023_f64,
                n: 10_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 352060171.430335_f64,
                n: 10_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6212028560.142858_f64,
                n: 10_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 53782171674.60492_f64,
                n: 10_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 309720255837.56775_f64,
                n: 10_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1359043035731.5713_f64,
                n: 10_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4900625954398.943_f64,
                n: 10_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler036_80: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data037_81: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 184756_f64,
                n: 10_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -210.84303350970018_f64,
                n: 10_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 508.38095238095184_f64,
                n: 10_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2098.857142857143_f64,
                n: 10_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -536338.8853615518_f64,
                n: 10_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 24865988.804232784_f64,
                n: 10_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1343756013.142857_f64,
                n: 10_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17298791247.358025_f64,
                n: 10_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 124528450897.79892_f64,
                n: 10_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 632674413641.7142_f64,
                n: 10_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2533008935405.03_f64,
                n: 10_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler037_82: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data038_83: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 30045014.999999993_f64,
                n: 10_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -23087.733686067022_f64,
                n: 10_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 207.2398589065233_f64,
                n: 10_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1407.8571428571508_f64,
                n: 10_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -44618.15696649032_f64,
                n: 10_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 158690.044091711_f64,
                n: 10_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6870413.571428573_f64,
                n: 10_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 793841351.4197533_f64,
                n: 10_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 13358288958.562618_f64,
                n: 10_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 106073722407.85715_f64,
                n: 10_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 566337213392.4249_f64,
                n: 10_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler038_84: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data039_85: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 75394027566_f64,
                n: 10_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 8048106183.372136_f64,
                n: 10_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 328045023.8483246_f64,
                n: 10_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2568769.714285718_f64,
                n: 10_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6971.996472663153_f64,
                n: 10_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 136111.41446207993_f64,
                n: 10_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -62462.57142857024_f64,
                n: 10_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -248167.95061728527_f64,
                n: 10_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1941270.409171077_f64,
                n: 10_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8643512.571428576_f64,
                n: 10_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -140863522.18342152_f64,
                n: 10_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler039_86: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data040_87: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 46897636623981_f64,
                n: 10_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16444031323272.084_f64,
                n: 10_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5020343986463.539_f64,
                n: 10_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1270977490645.286_f64,
                n: 10_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 244835756822.62262_f64,
                n: 10_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 29786827693.96296_f64,
                n: 10_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1127612095.2857144_f64,
                n: 10_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -66370555.419753075_f64,
                n: 10_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 10420852.95767187_f64,
                n: 10_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3373097.571428572_f64,
                n: 10_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2065423.6807760145_f64,
                n: 10_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler040_88: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data041_89: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 20_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -11.961333867812119_f64,
                n: 20_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2829.4728613531743_f64,
                n: 20_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18439.42450252094_f64,
                n: 20_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 24799805.877530713_f64,
                n: 20_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7551960453.767255_f64,
                n: 20_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1379223608444.9155_f64,
                n: 20_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 165423821874449.94_f64,
                n: 20_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 29500368536981676_f64,
                n: 20_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1129230951443290112_f64,
                n: 20_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 22061882785931735040_f64,
                n: 20_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler041_90: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data042_91: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 20_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 19.90048812973408_f64,
                n: 20_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2208.0318569557585_f64,
                n: 20_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 263690.96303121914_f64,
                n: 20_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 40667285.630564235_f64,
                n: 20_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1737442572.81154_f64,
                n: 20_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -588280953643.2812_f64,
                n: 20_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 45617733778241.33_f64,
                n: 20_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17293487114876864_f64,
                n: 20_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 762191358585850624_f64,
                n: 20_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16037288204336758784_f64,
                n: 20_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler042_92: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data043_93: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 231.00000000000003_f64,
                n: 20_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 47.00933806511292_f64,
                n: 20_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -652.5130546172859_f64,
                n: 20_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 285388.2589506911_f64,
                n: 20_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 28664069.68562473_f64,
                n: 20_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1399631966.3144732_f64,
                n: 20_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -115357373248.28194_f64,
                n: 20_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3357730872975.875_f64,
                n: 20_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 9765808962855122_f64,
                n: 20_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 507172929455591808_f64,
                n: 20_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11564665701334456320_f64,
                n: 20_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler043_94: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data044_95: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 53130_f64,
                n: 20_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -158.69554500944142_f64,
                n: 20_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 334.0801228803895_f64,
                n: 20_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -198372.47662554163_f64,
                n: 20_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13627144.088579426_f64,
                n: 20_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -780579985.4473151_f64,
                n: 20_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 116648634237.73535_f64,
                n: 20_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -12347348707739.742_f64,
                n: 20_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1199516248034090.8_f64,
                n: 20_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 134515031950785312_f64,
                n: 20_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4105890427611148288_f64,
                n: 20_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler044_96: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data045_97: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 30045015_f64,
                n: 20_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1755.6226861258601_f64,
                n: 20_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9081.67266447379_f64,
                n: 20_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 95771.65091211311_f64,
                n: 20_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5089151.927277924_f64,
                n: 20_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 97400399.45020671_f64,
                n: 20_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -16009352450.477026_f64,
                n: 20_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 842271286905.0105_f64,
                n: 20_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79901725466796.94_f64,
                n: 20_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7944103675858637_f64,
                n: 20_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 574298218933882880_f64,
                n: 20_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler045_98: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data046_99: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 137846528819.99994_f64,
                n: 20_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -136976.4957133323_f64,
                n: 20_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 113878.49908041643_f64,
                n: 20_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -342529.2177879618_f64,
                n: 20_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -350112.66981443466_f64,
                n: 20_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -10791735.172977809_f64,
                n: 20_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1038073940.0811402_f64,
                n: 20_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 667312550.6361634_f64,
                n: 20_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 741537869902.2903_f64,
                n: 20_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -32378376755737.418_f64,
                n: 20_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -601760332167937.6_f64,
                n: 20_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler046_100: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data047_101: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 161884603662657888_f64,
                n: 20_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1599011936804291.5_f64,
                n: 20_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -131273880831.42432_f64,
                n: 20_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3133213093.690355_f64,
                n: 20_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -213935628.04985175_f64,
                n: 20_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -47375578.49592106_f64,
                n: 20_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -115731015.14034876_f64,
                n: 20_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -737415147.2942029_f64,
                n: 20_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2123455626.862177_f64,
                n: 20_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 29801266858.60893_f64,
                n: 20_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -132886631026.82553_f64,
                n: 20_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler047_102: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data048_103: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 2.9462227291176643e+22_f64,
                n: 20_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3577789074870124412928_f64,
                n: 20_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 315849255214567587840_f64,
                n: 20_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17389599388424863744_f64,
                n: 20_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 414013427459806336_f64,
                n: 20_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79359706102062.6_f64,
                n: 20_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 22736203650743.145_f64,
                n: 20_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 65679006380.0957_f64,
                n: 20_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -236263257610.77792_f64,
                n: 20_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -38072644585.3031_f64,
                n: 20_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 68236474365.17397_f64,
                n: 20_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler048_104: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data049_105: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 50_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17.534183446338233_f64,
                n: 50_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 980.2696188979103_f64,
                n: 50_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 293000.5073596236_f64,
                n: 50_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14896937.968694873_f64,
                n: 50_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2513677852.691687_f64,
                n: 50_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -883876565337.9922_f64,
                n: 50_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -80967880733583.23_f64,
                n: 50_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8217471769564841_f64,
                n: 50_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2114003130804889088_f64,
                n: 50_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -397101034870946922496_f64,
                n: 50_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler049_106: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data050_107: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 51.00000000000002_f64,
                n: 50_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.4214573271639575_f64,
                n: 50_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2574.8072295127827_f64,
                n: 50_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 35846.479728359205_f64,
                n: 50_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48263698.76831853_f64,
                n: 50_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6161525870.273853_f64,
                n: 50_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -382655486658.47125_f64,
                n: 50_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -109635579833241.72_f64,
                n: 50_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14623805817283490_f64,
                n: 50_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2066684719087815168_f64,
                n: 50_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -143851879539976257536_f64,
                n: 50_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler050_108: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data051_109: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1326_f64,
                n: 50_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -87.86073251644453_f64,
                n: 50_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -5203.235119178092_f64,
                n: 50_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -461059.5001253887_f64,
                n: 50_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -30476695.32744059_f64,
                n: 50_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3720804977.9338136_f64,
                n: 50_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 362262002434.5145_f64,
                n: 50_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -52210917867820.23_f64,
                n: 50_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9567965136901914_f64,
                n: 50_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -891712775177128832_f64,
                n: 50_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 57231129448806981632_f64,
                n: 50_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler051_110: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data052_111: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 3478761_f64,
                n: 50_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1055.8381917651498_f64,
                n: 50_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 15264.646660345055_f64,
                n: 50_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1229651.8966600848_f64,
                n: 50_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 39270451.82365684_f64,
                n: 50_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4424062601.115203_f64,
                n: 50_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -186017434284.19223_f64,
                n: 50_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 50972853949302.61_f64,
                n: 50_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6530702754012517_f64,
                n: 50_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 683875927146780288_f64,
                n: 50_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 39198742504338391040_f64,
                n: 50_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler052_112: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data053_113: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 75394027565.99998_f64,
                n: 50_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 91833.92409877018_f64,
                n: 50_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 330501.8792977826_f64,
                n: 50_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3625088.1635972536_f64,
                n: 50_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 213954727.28632012_f64,
                n: 50_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9381006937.751768_f64,
                n: 50_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 535333683777.48615_f64,
                n: 50_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 18824406573722.17_f64,
                n: 50_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -533858276780013.1_f64,
                n: 50_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -52995774666704016_f64,
                n: 50_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6050418286244878336_f64,
                n: 50_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler053_114: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data054_115: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 161884603662657792_f64,
                n: 50_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -307637087.2516941_f64,
                n: 50_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 12524651.102974586_f64,
                n: 50_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -315460483.8621056_f64,
                n: 50_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1889683587.3459988_f64,
                n: 50_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 37457044404.20035_f64,
                n: 50_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -843831858224.718_f64,
                n: 50_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -92231643172.3075_f64,
                n: 50_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 904211757769501_f64,
                n: 50_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 46508193600283272_f64,
                n: 50_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2321688792816271872_f64,
                n: 50_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler054_116: Value<()> = Rc::new(RefCell::new(5.000000000000003e-12_f64));
);
thread_local!(
    pub static data055_117: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1.0089134454556417e+29_f64,
                n: 50_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.3822795753070493e+23_f64,
                n: 50_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 95817260381628336_f64,
                n: 50_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -910798580856015.4_f64,
                n: 50_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 50513254049166.92_f64,
                n: 50_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 84159703903348.94_f64,
                n: 50_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -138805244691822.72_f64,
                n: 50_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 181046391269246.25_f64,
                n: 50_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2086884905317107.5_f64,
                n: 50_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2765620139862428_f64,
                n: 50_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -137067516781462896_f64,
                n: 50_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler055_118: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data056_119: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 2.0128660909731929e+40_f64,
                n: 50_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 9.367509480769547e+37_f64,
                n: 50_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.3009321481877196e+35_f64,
                n: 50_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7.372002689323382e+30_f64,
                n: 50_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6.082467907963467e+25_f64,
                n: 50_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6.005318879354345e+23_f64,
                n: 50_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.4178129287264692e+22_f64,
                n: 50_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -546520993415667056640_f64,
                n: 50_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -108172717592632737792_f64,
                n: 50_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 38058734007924195328_f64,
                n: 50_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 47439240848028344320_f64,
                n: 50_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler056_120: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data057_121: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 100_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 13.27766284430345_f64,
                n: 100_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1854.0367283243388_f64,
                n: 100_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 170141.8698704655_f64,
                n: 100_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7272442.315600696_f64,
                n: 100_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4847420871.269051_f64,
                n: 100_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 693492765740.2969_f64,
                n: 100_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17125518672239.77_f64,
                n: 100_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13763178176383768_f64,
                n: 100_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2130722049038017280_f64,
                n: 100_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -262922606930689163264_f64,
                n: 100_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler057_122: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data058_123: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 101.00000000000003_f64,
                n: 100_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14.65066198368042_f64,
                n: 100_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1626.5010939361582_f64,
                n: 100_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 417884.7765826873_f64,
                n: 100_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -55617646.95164984_f64,
                n: 100_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 884829874.2662622_f64,
                n: 100_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 154466082750.32202_f64,
                n: 100_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -101423973484646_f64,
                n: 100_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1388352348671756.8_f64,
                n: 100_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 780487055132685824_f64,
                n: 100_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16948925059042754560_f64,
                n: 100_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler058_124: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data059_125: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 5151.0000000000055_f64,
                n: 100_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -150.22012290951324_f64,
                n: 100_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7655.059329404945_f64,
                n: 100_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -140996.6927617909_f64,
                n: 100_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -38645171.27854935_f64,
                n: 100_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8889263688.211893_f64,
                n: 100_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1010338971533.34_f64,
                n: 100_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -127582564332943.9_f64,
                n: 100_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 15970305694654312_f64,
                n: 100_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1601984499286281984_f64,
                n: 100_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 282670247309629554688_f64,
                n: 100_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler059_126: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data060_127: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 96560646.00000003_f64,
                n: 100_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2430.6732236677612_f64,
                n: 100_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 111162.32026994647_f64,
                n: 100_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4036708.259941363_f64,
                n: 100_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -34055982.66440544_f64,
                n: 100_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 30110688343.56233_f64,
                n: 100_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2651429940558.2974_f64,
                n: 100_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 192108556058943.1_f64,
                n: 100_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -25410533973455528_f64,
                n: 100_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2107295563356443136_f64,
                n: 100_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -294340053558772891648_f64,
                n: 100_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler060_128: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data061_129: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 46897636623981.04_f64,
                n: 100_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 529208.1155099033_f64,
                n: 100_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7402892.174880382_f64,
                n: 100_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 88369632.08324358_f64,
                n: 100_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 822187797.5909656_f64,
                n: 100_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 180231446033.06866_f64,
                n: 100_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7922942703798.131_f64,
                n: 100_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 784424250559042.1_f64,
                n: 100_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -16325634720239370_f64,
                n: 100_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1087958830744316160_f64,
                n: 100_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 30114394463610642432_f64,
                n: 100_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler061_130: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data062_131: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 2.9462227291176614e+22_f64,
                n: 100_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 313694958939.90405_f64,
                n: 100_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 45396489338.09619_f64,
                n: 100_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7215826758.008125_f64,
                n: 100_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 825949194005.8885_f64,
                n: 100_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2764742119971.081_f64,
                n: 100_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -219802198273516.03_f64,
                n: 100_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1699053306145262_f64,
                n: 100_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 354957093450238464_f64,
                n: 100_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9612867511029241856_f64,
                n: 100_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 436198684220722118656_f64,
                n: 100_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler062_132: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data063_133: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 2.012866090973193e+40_f64,
                n: 100_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4.015144391347337e+28_f64,
                n: 100_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3.2199632594551924e+22_f64,
                n: 100_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -275687020926597562368_f64,
                n: 100_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 75553066015421562880_f64,
                n: 100_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -27651625252387733504_f64,
                n: 100_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -58963680147283804160_f64,
                n: 100_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -180827981630331060224_f64,
                n: 100_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -390442769868172492800_f64,
                n: 100_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6.99263107004019e+21_f64,
                n: 100_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -5.572727280992365e+22_f64,
                n: 100_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler063_134: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data064_135: Value<Box<[testcase_assoc_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_laguerre_double_ {
                f0: 9.054851465610322e+58_f64,
                n: 100_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.3334078033060556e+54_f64,
                n: 100_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2.100263925421134e+46_f64,
                n: 100_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1.1073158068796292e+39_f64,
                n: 100_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8.364093736398135e+35_f64,
                n: 100_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6.587933942931269e+32_f64,
                n: 100_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2.419064507769877e+30_f64,
                n: 100_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7.922496046566217e+29_f64,
                n: 100_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2.8605772478408694e+29_f64,
                n: 100_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2.4149589189609957e+28_f64,
                n: 100_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5.114647601485902e+28_f64,
                n: 100_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler064_136: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
pub fn test_137(data: Ptr<testcase_assoc_laguerre_double_>, mut toler: ()) {
    let mut test: () = true;
    let mut eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 11_();
    let mut i: () = 0_();
    'loop_: while (i < num_datum) {
        let mut f: () = unimplemented!();
        let mut f0: () = elem!((data), i).with(|__s| __s.f0);
        let mut diff: () = (f - f0);
        if (unimplemented!() > max_abs_diff) {
            max_abs_diff = unimplemented!();
        }
        if (unimplemented!() > (10_() * eps)) && (unimplemented!() > (10_() * eps)) {
            let mut frac: () = (diff / f0);
            if (unimplemented!() > max_abs_frac) {
                max_abs_frac = unimplemented!();
            }
        }
        i.prefix_inc();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !(max_abs_frac < toler) {
            unimplemented!();
            unimplemented!();
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({
        test_137(
            data001_9.with(|v| v.as_pointer()),
            toler001_10.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data002_11.with(|v| v.as_pointer()),
            toler002_12.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data003_13.with(|v| v.as_pointer()),
            toler003_14.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data004_15.with(|v| v.as_pointer()),
            toler004_16.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data005_17.with(|v| v.as_pointer()),
            toler005_18.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data006_19.with(|v| v.as_pointer()),
            toler006_20.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data007_21.with(|v| v.as_pointer()),
            toler007_22.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data008_23.with(|v| v.as_pointer()),
            toler008_24.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data009_25.with(|v| v.as_pointer()),
            toler009_26.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data010_27.with(|v| v.as_pointer()),
            toler010_28.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data011_29.with(|v| v.as_pointer()),
            toler011_30.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data012_31.with(|v| v.as_pointer()),
            toler012_32.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data013_33.with(|v| v.as_pointer()),
            toler013_34.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data014_35.with(|v| v.as_pointer()),
            toler014_36.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data015_37.with(|v| v.as_pointer()),
            toler015_38.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data016_39.with(|v| v.as_pointer()),
            toler016_40.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data017_41.with(|v| v.as_pointer()),
            toler017_42.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data018_43.with(|v| v.as_pointer()),
            toler018_44.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data019_45.with(|v| v.as_pointer()),
            toler019_46.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data020_47.with(|v| v.as_pointer()),
            toler020_48.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data021_49.with(|v| v.as_pointer()),
            toler021_50.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data022_51.with(|v| v.as_pointer()),
            toler022_52.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data023_53.with(|v| v.as_pointer()),
            toler023_54.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data024_55.with(|v| v.as_pointer()),
            toler024_56.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data025_57.with(|v| v.as_pointer()),
            toler025_58.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data026_59.with(|v| v.as_pointer()),
            toler026_60.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data027_61.with(|v| v.as_pointer()),
            toler027_62.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data028_63.with(|v| v.as_pointer()),
            toler028_64.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data029_65.with(|v| v.as_pointer()),
            toler029_66.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data030_67.with(|v| v.as_pointer()),
            toler030_68.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data031_69.with(|v| v.as_pointer()),
            toler031_70.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data032_71.with(|v| v.as_pointer()),
            toler032_72.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data033_73.with(|v| v.as_pointer()),
            toler033_74.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data034_75.with(|v| v.as_pointer()),
            toler034_76.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data035_77.with(|v| v.as_pointer()),
            toler035_78.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data036_79.with(|v| v.as_pointer()),
            toler036_80.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data037_81.with(|v| v.as_pointer()),
            toler037_82.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data038_83.with(|v| v.as_pointer()),
            toler038_84.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data039_85.with(|v| v.as_pointer()),
            toler039_86.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data040_87.with(|v| v.as_pointer()),
            toler040_88.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data041_89.with(|v| v.as_pointer()),
            toler041_90.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data042_91.with(|v| v.as_pointer()),
            toler042_92.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data043_93.with(|v| v.as_pointer()),
            toler043_94.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data044_95.with(|v| v.as_pointer()),
            toler044_96.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data045_97.with(|v| v.as_pointer()),
            toler045_98.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data046_99.with(|v| v.as_pointer()),
            toler046_100.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data047_101.with(|v| v.as_pointer()),
            toler047_102.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data048_103.with(|v| v.as_pointer()),
            toler048_104.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data049_105.with(|v| v.as_pointer()),
            toler049_106.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data050_107.with(|v| v.as_pointer()),
            toler050_108.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data051_109.with(|v| v.as_pointer()),
            toler051_110.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data052_111.with(|v| v.as_pointer()),
            toler052_112.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data053_113.with(|v| v.as_pointer()),
            toler053_114.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data054_115.with(|v| v.as_pointer()),
            toler054_116.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data055_117.with(|v| v.as_pointer()),
            toler055_118.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data056_119.with(|v| v.as_pointer()),
            toler056_120.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data057_121.with(|v| v.as_pointer()),
            toler057_122.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data058_123.with(|v| v.as_pointer()),
            toler058_124.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data059_125.with(|v| v.as_pointer()),
            toler059_126.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data060_127.with(|v| v.as_pointer()),
            toler060_128.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data061_129.with(|v| v.as_pointer()),
            toler061_130.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data062_131.with(|v| v.as_pointer()),
            toler062_132.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data063_133.with(|v| v.as_pointer()),
            toler063_134.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_137(
            data064_135.with(|v| v.as_pointer()),
            toler064_136.with(|rc| *rc.borrow()),
        )
    });
    return 0_();
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: unimplemented!(),
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write(unimplemented!());
        field!(this, sem_set_).write(unimplemented!());
        if (this.with(|__s| __s.sem_set_) == -1_()) {
            unimplemented!();
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_138(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = (size * 1048576_());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn set_file_limit_139(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_140(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = unimplemented!();
    if !(demangled).is_null() {
        s = (demangled).clone();
    } else {
        switch!(match (*status.borrow()) {
            __v if __v == 0_() => {
                s = Ptr::<()>::from_string_literal(b"error code = 0: success");
                break;
            }
            __v if __v == -1_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -1: memory allocation failure");
                break;
            }
            __v if __v == -2_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -2: invalid mangled name");
                break;
            }
            __v if __v == -3_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -3: invalid arguments");
                break;
            }
            _ => {
                s = Ptr::<()>::from_string_literal(b"error code unknown - who knows what happened");
            }
        });
    }
    let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if !(unimplemented!()) {
        unimplemented!();
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_141(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<()> = unimplemented!();
    if !(res).is_null() {
        let mut preLC_ALL: () = unimplemented!();
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn run_tests_wrapped_env_142(
    mut name: Ptr<()>,
    mut env: Ptr<()>,
    l: Ptr<__gnu_test_func_callback>,
) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut oldENV: Ptr<()> = unimplemented!();
    if !(unimplemented!() != 0) {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        unimplemented!();
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static _M_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static next_id__6: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct __gnu_test_semun {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl __gnu_test_semun {
    pub fn val(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
    pub fn array(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        __gnu_test_semun {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
impl __gnu_test_semaphore {}
pub fn test_tm_143(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> () {
    (*tmp_144.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_144.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_144.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_144: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub trait __gnu_test_copy_trackerImpl {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker>;
    fn destructor(&self);
    fn id(&self) -> ();
}
impl __gnu_test_copy_trackerImpl for Ptr<__gnu_test_copy_tracker> {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker> {
        field!((*self), id_).write(({ __gnu_test_copy_trackerImpl::id(&rhs) }));
        if rhs.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_assignment_operator::throw_on(
                    ({ __gnu_test_assignment_operator::count() }).wrapping_add(1_()),
                )
            });
        }
        ({ __gnu_test_assignment_operator::mark_call() });
        return (*self).clone();
    }
    fn destructor(&self) {
        ({ __gnu_test_destructor::mark_call() });
    }
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait __gnu_test_func_callbackImpl {
    fn copy_assign(&self, _a0: Ptr<__gnu_test_func_callback>) -> Ptr<__gnu_test_func_callback> {
        unimplemented!()
    }
    fn size(&self) -> ();
    fn tests(&self) -> Ptr<FnPtr<fn()>>;
    fn push_back(&self, test: FnPtr<fn()>);
}
impl __gnu_test_func_callbackImpl for Ptr<__gnu_test_func_callback> {
    fn size(&self) -> () {
        return (*self).with(|__s| __s._M_size);
    }
    fn tests(&self) -> Ptr<FnPtr<fn()>> {
        return (array_field_ptr!((*self), _M_tests) as Ptr<FnPtr<fn()>>);
    }
    fn push_back(&self, mut test: FnPtr<fn()>) {
        array_elem!((*self), _M_tests, (*self).with(|__s| __s._M_size)).write((test).clone());
        field!((*self), _M_size).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait __gnu_test_object_counterImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter>;
}
impl __gnu_test_object_counterImpl for Ptr<__gnu_test_object_counter> {
    fn destructor(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter> {
        return (*self).clone();
    }
}
pub trait __gnu_test_semaphoreImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn signal(&self) {
        unimplemented!()
    }
    fn wait(&self) {
        unimplemented!()
    }
}
impl __gnu_test_semaphoreImpl for Ptr<__gnu_test_semaphore> {
    fn destructor(&self) {
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if ((*self).with(|__s| __s.pid_) == unimplemented!()) {
            unimplemented!();
        }
    }
    fn signal(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
    }
    fn wait(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = data001_9.with(|_| ());
    let _ = toler001_10.with(|_| ());
    let _ = data002_11.with(|_| ());
    let _ = toler002_12.with(|_| ());
    let _ = data003_13.with(|_| ());
    let _ = toler003_14.with(|_| ());
    let _ = data004_15.with(|_| ());
    let _ = toler004_16.with(|_| ());
    let _ = data005_17.with(|_| ());
    let _ = toler005_18.with(|_| ());
    let _ = data006_19.with(|_| ());
    let _ = toler006_20.with(|_| ());
    let _ = data007_21.with(|_| ());
    let _ = toler007_22.with(|_| ());
    let _ = data008_23.with(|_| ());
    let _ = toler008_24.with(|_| ());
    let _ = data009_25.with(|_| ());
    let _ = toler009_26.with(|_| ());
    let _ = data010_27.with(|_| ());
    let _ = toler010_28.with(|_| ());
    let _ = data011_29.with(|_| ());
    let _ = toler011_30.with(|_| ());
    let _ = data012_31.with(|_| ());
    let _ = toler012_32.with(|_| ());
    let _ = data013_33.with(|_| ());
    let _ = toler013_34.with(|_| ());
    let _ = data014_35.with(|_| ());
    let _ = toler014_36.with(|_| ());
    let _ = data015_37.with(|_| ());
    let _ = toler015_38.with(|_| ());
    let _ = data016_39.with(|_| ());
    let _ = toler016_40.with(|_| ());
    let _ = data017_41.with(|_| ());
    let _ = toler017_42.with(|_| ());
    let _ = data018_43.with(|_| ());
    let _ = toler018_44.with(|_| ());
    let _ = data019_45.with(|_| ());
    let _ = toler019_46.with(|_| ());
    let _ = data020_47.with(|_| ());
    let _ = toler020_48.with(|_| ());
    let _ = data021_49.with(|_| ());
    let _ = toler021_50.with(|_| ());
    let _ = data022_51.with(|_| ());
    let _ = toler022_52.with(|_| ());
    let _ = data023_53.with(|_| ());
    let _ = toler023_54.with(|_| ());
    let _ = data024_55.with(|_| ());
    let _ = toler024_56.with(|_| ());
    let _ = data025_57.with(|_| ());
    let _ = toler025_58.with(|_| ());
    let _ = data026_59.with(|_| ());
    let _ = toler026_60.with(|_| ());
    let _ = data027_61.with(|_| ());
    let _ = toler027_62.with(|_| ());
    let _ = data028_63.with(|_| ());
    let _ = toler028_64.with(|_| ());
    let _ = data029_65.with(|_| ());
    let _ = toler029_66.with(|_| ());
    let _ = data030_67.with(|_| ());
    let _ = toler030_68.with(|_| ());
    let _ = data031_69.with(|_| ());
    let _ = toler031_70.with(|_| ());
    let _ = data032_71.with(|_| ());
    let _ = toler032_72.with(|_| ());
    let _ = data033_73.with(|_| ());
    let _ = toler033_74.with(|_| ());
    let _ = data034_75.with(|_| ());
    let _ = toler034_76.with(|_| ());
    let _ = data035_77.with(|_| ());
    let _ = toler035_78.with(|_| ());
    let _ = data036_79.with(|_| ());
    let _ = toler036_80.with(|_| ());
    let _ = data037_81.with(|_| ());
    let _ = toler037_82.with(|_| ());
    let _ = data038_83.with(|_| ());
    let _ = toler038_84.with(|_| ());
    let _ = data039_85.with(|_| ());
    let _ = toler039_86.with(|_| ());
    let _ = data040_87.with(|_| ());
    let _ = toler040_88.with(|_| ());
    let _ = data041_89.with(|_| ());
    let _ = toler041_90.with(|_| ());
    let _ = data042_91.with(|_| ());
    let _ = toler042_92.with(|_| ());
    let _ = data043_93.with(|_| ());
    let _ = toler043_94.with(|_| ());
    let _ = data044_95.with(|_| ());
    let _ = toler044_96.with(|_| ());
    let _ = data045_97.with(|_| ());
    let _ = toler045_98.with(|_| ());
    let _ = data046_99.with(|_| ());
    let _ = toler046_100.with(|_| ());
    let _ = data047_101.with(|_| ());
    let _ = toler047_102.with(|_| ());
    let _ = data048_103.with(|_| ());
    let _ = toler048_104.with(|_| ());
    let _ = data049_105.with(|_| ());
    let _ = toler049_106.with(|_| ());
    let _ = data050_107.with(|_| ());
    let _ = toler050_108.with(|_| ());
    let _ = data051_109.with(|_| ());
    let _ = toler051_110.with(|_| ());
    let _ = data052_111.with(|_| ());
    let _ = toler052_112.with(|_| ());
    let _ = data053_113.with(|_| ());
    let _ = toler053_114.with(|_| ());
    let _ = data054_115.with(|_| ());
    let _ = toler054_116.with(|_| ());
    let _ = data055_117.with(|_| ());
    let _ = toler055_118.with(|_| ());
    let _ = data056_119.with(|_| ());
    let _ = toler056_120.with(|_| ());
    let _ = data057_121.with(|_| ());
    let _ = toler057_122.with(|_| ());
    let _ = data058_123.with(|_| ());
    let _ = toler058_124.with(|_| ());
    let _ = data059_125.with(|_| ());
    let _ = toler059_126.with(|_| ());
    let _ = data060_127.with(|_| ());
    let _ = toler060_128.with(|_| ());
    let _ = data061_129.with(|_| ());
    let _ = toler061_130.with(|_| ());
    let _ = data062_131.with(|_| ());
    let _ = toler062_132.with(|_| ());
    let _ = data063_133.with(|_| ());
    let _ = toler063_134.with(|_| ());
    let _ = data064_135.with(|_| ());
    let _ = toler064_136.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
