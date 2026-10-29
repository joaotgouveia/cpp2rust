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
pub struct testcase_legendre_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub l: (),
    #[offset(16)]
    pub x: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data001_9: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: -1_f64,
                l: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.9_f64,
                l: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.8_f64,
                l: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.7_f64,
                l: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.6_f64,
                l: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.5_f64,
                l: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.3999999999999999_f64,
                l: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.29999999999999993_f64,
                l: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.19999999999999996_f64,
                l: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.09999999999999998_f64,
                l: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10000000000000009_f64,
                l: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.20000000000000018_f64,
                l: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.30000000000000004_f64,
                l: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.40000000000000013_f64,
                l: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.5_f64,
                l: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.6000000000000001_f64,
                l: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.7000000000000002_f64,
                l: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.8_f64,
                l: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.9000000000000001_f64,
                l: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.7150000000000001_f64,
                l: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.23499999999999988_f64,
                l: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.039999999999999925_f64,
                l: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2600000000000001_f64,
                l: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.36500000000000005_f64,
                l: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.44000000000000006_f64,
                l: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.5_f64,
                l: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.4399999999999999_f64,
                l: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.365_f64,
                l: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.25999999999999984_f64,
                l: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.04000000000000015_f64,
                l: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.23500000000000032_f64,
                l: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.7150000000000003_f64,
                l: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: -1_f64,
                l: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.04114125000000009_f64,
                l: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.39951999999999993_f64,
                l: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.3651987499999999_f64,
                l: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.15263999999999994_f64,
                l: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.08984375_f64,
                l: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2706400000000001_f64,
                l: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.34538624999999995_f64,
                l: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.30751999999999996_f64,
                l: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.17882874999999995_f64,
                l: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.17882875000000015_f64,
                l: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.3075200000000001_f64,
                l: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.34538625_f64,
                l: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.2706399999999999_f64,
                l: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08984375_f64,
                l: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.15264000000000016_f64,
                l: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.36519875000000024_f64,
                l: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.39951999999999993_f64,
                l: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.04114124999999915_f64,
                l: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2631456178558596_f64,
                l: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08580579553164033_f64,
                l: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2436627456_f64,
                l: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.09683906440000026_f64,
                l: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.12907202559999983_f64,
                l: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.12212499738710943_f64,
                l: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.24609375_f64,
                l: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.12212499738710922_f64,
                l: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.12907202560000042_f64,
                l: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.09683906439999963_f64,
                l: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.24366274559999984_f64,
                l: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08580579553164128_f64,
                l: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2631456178558601_f64,
                l: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.14930823530984835_f64,
                l: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.20457394463834172_f64,
                l: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.1591675291009811_f64,
                l: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.10159261558628112_f64,
                l: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.18028715947998047_f64,
                l: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0980421943445948_f64,
                l: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.08207713094452766_f64,
                l: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.17619705200195312_f64,
                l: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.08207713094452802_f64,
                l: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.09804219434459409_f64,
                l: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.18028715947998042_f64,
                l: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.10159261558628192_f64,
                l: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.15916752910098075_f64,
                l: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.20457394463834136_f64,
                l: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.14930823530984758_f64,
                l: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.1700376599438367_f64,
                l: 50_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.014572731645892852_f64,
                l: 50_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.058860798844002096_f64,
                l: 50_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.041569033381824674_f64,
                l: 50_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.1091105157471479_f64,
                l: 50_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.0834322722041975_f64,
                l: 50_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0382058126613136_f64,
                l: 50_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.11227517265921705_f64,
                l: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.038205812661314155_f64,
                l: 50_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08343227220419656_f64,
                l: 50_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10911051574714797_f64,
                l: 50_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.04156903338182601_f64,
                l: 50_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.05886079884400143_f64,
                l: 50_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.014572731645890737_f64,
                l: 50_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.1700376599438368_f64,
                l: 50_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10226582055871893_f64,
                l: 100_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.07713250719977864_f64,
                l: 100_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.02374702390513314_f64,
                l: 100_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.07225820212568503_f64,
                l: 100_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.057127392202801046_f64,
                l: 100_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.014681835355659706_f64,
                l: 100_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0638950984347502_f64,
                l: 100_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.07958923738717873_f64,
                l: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.06389509843474976_f64,
                l: 100_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.014681835355657875_f64,
                l: 100_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.057127392202801566_f64,
                l: 100_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.07225820212568408_f64,
                l: 100_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.023747023905134217_f64,
                l: 100_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0771325071997805_f64,
                l: 100_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10226582055872063_f64,
                l: 100_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
pub fn test_25(data: Ptr<testcase_legendre_double_>, mut toler: ()) {
    let mut test: () = true;
    let mut eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 21_();
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
        test_25(
            data001_9.with(|v| v.as_pointer()),
            toler001_10.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_25(
            data002_11.with(|v| v.as_pointer()),
            toler002_12.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_25(
            data003_13.with(|v| v.as_pointer()),
            toler003_14.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_25(
            data004_15.with(|v| v.as_pointer()),
            toler004_16.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_25(
            data005_17.with(|v| v.as_pointer()),
            toler005_18.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_25(
            data006_19.with(|v| v.as_pointer()),
            toler006_20.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_25(
            data007_21.with(|v| v.as_pointer()),
            toler007_22.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_25(
            data008_23.with(|v| v.as_pointer()),
            toler008_24.with(|rc| *rc.borrow()),
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
pub fn set_memory_limits_26(size: Option<()>) {
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
pub fn set_file_limit_27(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_28(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_29(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_30(
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
pub fn test_tm_31(
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
    (*tmp_32.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_32.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_32.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_32: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
