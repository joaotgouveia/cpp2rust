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
pub struct testcase_assoc_legendre_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub l: (),
    #[offset(12)]
    pub m: (),
    #[offset(16)]
    pub x: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data001_9: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -1_f64,
                l: 1_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.9_f64,
                l: 1_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.8_f64,
                l: 1_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.7_f64,
                l: 1_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.6_f64,
                l: 1_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.5_f64,
                l: 1_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.3999999999999999_f64,
                l: 1_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.29999999999999993_f64,
                l: 1_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.19999999999999996_f64,
                l: 1_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.09999999999999998_f64,
                l: 1_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10000000000000009_f64,
                l: 1_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.20000000000000018_f64,
                l: 1_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.30000000000000004_f64,
                l: 1_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.40000000000000013_f64,
                l: 1_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5_f64,
                l: 1_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.6000000000000001_f64,
                l: 1_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7000000000000002_f64,
                l: 1_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8_f64,
                l: 1_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9000000000000001_f64,
                l: 1_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 1_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4358898943540673_f64,
                l: 1_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5999999999999999_f64,
                l: 1_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7141428428542851_f64,
                l: 1_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8_f64,
                l: 1_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8660254037844386_f64,
                l: 1_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.916515138991168_f64,
                l: 1_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9539392014169455_f64,
                l: 1_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9797958971132712_f64,
                l: 1_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.99498743710662_f64,
                l: 1_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 1_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.99498743710662_f64,
                l: 1_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9797958971132712_f64,
                l: 1_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9539392014169458_f64,
                l: 1_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.9165151389911679_f64,
                l: 1_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8660254037844386_f64,
                l: 1_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7999999999999999_f64,
                l: 1_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7141428428542848_f64,
                l: 1_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5999999999999999_f64,
                l: 1_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4358898943540671_f64,
                l: 1_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7150000000000001_f64,
                l: 2_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.23499999999999988_f64,
                l: 2_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.039999999999999925_f64,
                l: 2_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2600000000000001_f64,
                l: 2_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.36500000000000005_f64,
                l: 2_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.44000000000000006_f64,
                l: 2_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.5_f64,
                l: 2_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.4399999999999999_f64,
                l: 2_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.365_f64,
                l: 2_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.25999999999999984_f64,
                l: 2_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.04000000000000015_f64,
                l: 2_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.23500000000000032_f64,
                l: 2_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7150000000000003_f64,
                l: 2_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 2_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1769027147559816_f64,
                l: 2_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.44_f64,
                l: 2_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.4996999699939983_f64,
                l: 2_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.44_f64,
                l: 2_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.299038105676658_f64,
                l: 2_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.0998181667894014_f64,
                l: 2_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.8585452812752508_f64,
                l: 2_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.5878775382679626_f64,
                l: 2_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2984962311319859_f64,
                l: 2_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.29849623113198626_f64,
                l: 2_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5878775382679633_f64,
                l: 2_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8585452812752513_f64,
                l: 2_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0998181667894018_f64,
                l: 2_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.299038105676658_f64,
                l: 2_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.4400000000000002_f64,
                l: 2_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.4996999699939983_f64,
                l: 2_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.44_f64,
                l: 2_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.1769027147559812_f64,
                l: 2_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5699999999999998_f64,
                l: 2_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0799999999999996_f64,
                l: 2_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.5300000000000005_f64,
                l: 2_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.9200000000000004_f64,
                l: 2_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.25_f64,
                l: 2_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.52_f64,
                l: 2_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.7299999999999995_f64,
                l: 2_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.88_f64,
                l: 2_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.97_f64,
                l: 2_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3_f64,
                l: 2_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.97_f64,
                l: 2_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.88_f64,
                l: 2_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.7300000000000004_f64,
                l: 2_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.519999999999999_f64,
                l: 2_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.25_f64,
                l: 2_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.9199999999999997_f64,
                l: 2_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.529999999999999_f64,
                l: 2_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0799999999999996_f64,
                l: 2_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.5699999999999993_f64,
                l: 2_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 2_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -1_f64,
                l: 5_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.04114125000000009_f64,
                l: 5_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.39951999999999993_f64,
                l: 5_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.3651987499999999_f64,
                l: 5_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.15263999999999994_f64,
                l: 5_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.08984375_f64,
                l: 5_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2706400000000001_f64,
                l: 5_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.34538624999999995_f64,
                l: 5_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.30751999999999996_f64,
                l: 5_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.17882874999999995_f64,
                l: 5_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.17882875000000015_f64,
                l: 5_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.3075200000000001_f64,
                l: 5_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.34538625_f64,
                l: 5_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.2706399999999999_f64,
                l: 5_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08984375_f64,
                l: 5_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.15264000000000016_f64,
                l: 5_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.36519875000000024_f64,
                l: 5_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.39951999999999993_f64,
                l: 5_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.04114124999999915_f64,
                l: 5_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 5_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 5_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.809936960835098_f64,
                l: 5_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7218000000000009_f64,
                l: 5_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.0951826834447254_f64,
                l: 5_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9775999999999998_f64,
                l: 5_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9282596881137892_f64,
                l: 5_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.2070504380513671_f64,
                l: 5_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.160798376638843_f64,
                l: 5_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8700587566365854_f64,
                l: 5_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.6083350053680323_f64,
                l: 5_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.875_f64,
                l: 5_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.6083350053680314_f64,
                l: 5_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8700587566365833_f64,
                l: 5_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.16079837663884422_f64,
                l: 5_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.2070504380513694_f64,
                l: 5_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9282596881137892_f64,
                l: 5_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.9775999999999998_f64,
                l: 5_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.0951826834447216_f64,
                l: 5_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7218000000000009_f64,
                l: 5_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.8099369608351_f64,
                l: 5_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 5_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data009_25: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -12.837825_f64,
                l: 5_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -13.910400000000001_f64,
                l: 5_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.808974999999997_f64,
                l: 5_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.6128_f64,
                l: 5_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.921875_f64,
                l: 5_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.1728_f64,
                l: 5_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 10.462724999999997_f64,
                l: 5_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.870399999999998_f64,
                l: 5_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.041574999999999_f64,
                l: 5_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 5_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.041575000000004_f64,
                l: 5_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.870400000000005_f64,
                l: 5_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -10.462725_f64,
                l: 5_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9.172799999999997_f64,
                l: 5_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.921875_f64,
                l: 5_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.6128000000000047_f64,
                l: 5_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.80897500000001_f64,
                l: 5_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 13.910400000000001_f64,
                l: 5_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12.83782499999999_f64,
                l: 5_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_26: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data010_27: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14.870165800941818_f64,
                l: 5_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 73.48319999999993_f64,
                l: 5_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 175.53238298794764_f64,
                l: 5_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 309.65760000000006_f64,
                l: 5_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 460.3466286991656_f64,
                l: 5_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 611.1249625581988_f64,
                l: 5_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 746.509414795237_f64,
                l: 5_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 853.3160043467132_f64,
                l: 5_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 921.5518918172473_f64,
                l: 5_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 945_f64,
                l: 5_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 921.5518918172473_f64,
                l: 5_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 853.3160043467132_f64,
                l: 5_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 746.5094147952376_f64,
                l: 5_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 611.1249625581984_f64,
                l: 5_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 460.3466286991656_f64,
                l: 5_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 309.6575999999999_f64,
                l: 5_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 175.53238298794724_f64,
                l: 5_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 73.48319999999993_f64,
                l: 5_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14.87016580094179_f64,
                l: 5_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_28: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data011_29: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2631456178558596_f64,
                l: 10_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08580579553164033_f64,
                l: 10_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2436627456_f64,
                l: 10_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.09683906440000026_f64,
                l: 10_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.12907202559999983_f64,
                l: 10_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.12212499738710943_f64,
                l: 10_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.24609375_f64,
                l: 10_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.12212499738710922_f64,
                l: 10_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.12907202560000042_f64,
                l: 10_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.09683906439999963_f64,
                l: 10_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.24366274559999984_f64,
                l: 10_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08580579553164128_f64,
                l: 10_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.2631456178558601_f64,
                l: 10_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_30: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data012_31: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.043874878147904_f64,
                l: 10_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.8761426080000025_f64,
                l: 10_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.9685359952934527_f64,
                l: 10_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.2511825919999997_f64,
                l: 10_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.006687739436126_f64,
                l: 10_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.4822196173476647_f64,
                l: 10_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.12309508907433593_f64,
                l: 10_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.2468221751958413_f64,
                l: 10_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.2472659777983512_f64,
                l: 10_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2472659777983535_f64,
                l: 10_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2468221751958377_f64,
                l: 10_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1230950890743391_f64,
                l: 10_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.482219617347667_f64,
                l: 10_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.006687739436126_f64,
                l: 10_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.2511825920000037_f64,
                l: 10_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.9685359952934505_f64,
                l: 10_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.8761426080000025_f64,
                l: 10_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.043874878147898_f64,
                l: 10_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 10_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_32: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data013_33: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 16.376387762496137_f64,
                l: 10_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -35.394657804000005_f64,
                l: 10_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.619142942378865_f64,
                l: 10_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 28.679675904000014_f64,
                l: 10_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 18.388023376464844_f64,
                l: 10_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -12.818955996000021_f64,
                l: 10_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27.739821675972646_f64,
                l: 10_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -13.280661503999987_f64,
                l: 10_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 13.885467170308601_f64,
                l: 10_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 27.0703125_f64,
                l: 10_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 13.885467170308573_f64,
                l: 10_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -13.28066150400005_f64,
                l: 10_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27.739821675972664_f64,
                l: 10_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -12.81895599599996_f64,
                l: 10_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 18.388023376464844_f64,
                l: 10_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 28.679675904_f64,
                l: 10_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.6191429423789856_f64,
                l: 10_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -35.394657804000005_f64,
                l: 10_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 16.3763877624962_f64,
                l: 10_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_34: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data014_35: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 10_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -21343.61851816468_f64,
                l: 10_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -40457.01640780798_f64,
                l: 10_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -20321.279317331315_f64,
                l: 10_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14410.820616192004_f64,
                l: 10_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 30086.169706116176_f64,
                l: 10_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 17177.549337582834_f64,
                l: 10_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9272.511949541255_f64,
                l: 10_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -26591.511184414714_f64,
                l: 10_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -21961.95123850421_f64,
                l: 10_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 21961.95123850423_f64,
                l: 10_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 26591.511184414703_f64,
                l: 10_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9272.511949541236_f64,
                l: 10_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -17177.549337582877_f64,
                l: 10_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -30086.169706116176_f64,
                l: 10_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -14410.820616191972_f64,
                l: 10_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 20321.27931733139_f64,
                l: 10_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 40457.01640780798_f64,
                l: 10_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 21343.618518164636_f64,
                l: 10_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 10_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_36: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data015_37: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 162117.4007878422_f64,
                l: 10_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3958896.3481267113_f64,
                l: 10_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 22589806.343887307_f64,
                l: 10_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 70300999.12163332_f64,
                l: 10_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 155370278.540039_f64,
                l: 10_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 273815518.2015052_f64,
                l: 10_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 408571989.1315885_f64,
                l: 10_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 533848212.07990247_f64,
                l: 10_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 622640835.7052308_f64,
                l: 10_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 654729075_f64,
                l: 10_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 622640835.7052308_f64,
                l: 10_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 533848212.07990247_f64,
                l: 10_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 408571989.1315892_f64,
                l: 10_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 273815518.2015048_f64,
                l: 10_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 155370278.540039_f64,
                l: 10_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 70300999.12163322_f64,
                l: 10_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 22589806.343887202_f64,
                l: 10_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3958896.3481267113_f64,
                l: 10_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 162117.4007878416_f64,
                l: 10_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 10_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler015_38: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data016_39: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.14930823530984835_f64,
                l: 20_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.20457394463834172_f64,
                l: 20_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.1591675291009811_f64,
                l: 20_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.10159261558628112_f64,
                l: 20_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.18028715947998047_f64,
                l: 20_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0980421943445948_f64,
                l: 20_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.08207713094452766_f64,
                l: 20_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.17619705200195312_f64,
                l: 20_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.08207713094452802_f64,
                l: 20_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.09804219434459409_f64,
                l: 20_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.18028715947998042_f64,
                l: 20_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.10159261558628192_f64,
                l: 20_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.15916752910098075_f64,
                l: 20_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.20457394463834136_f64,
                l: 20_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.14930823530984758_f64,
                l: 20_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler016_40: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data017_41: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.38383348182205_f64,
                l: 20_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.6313829614634084_f64,
                l: 20_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.722748714133914_f64,
                l: 20_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.320352874382491_f64,
                l: 20_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.7399919228791405_f64,
                l: 20_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.1692202279270085_f64,
                l: 20_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1580446883534503_f64,
                l: 20_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.036618239327117_f64,
                l: 20_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.211552381558021_f64,
                l: 20_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.211552381558017_f64,
                l: 20_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.036618239327126_f64,
                l: 20_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.15804468835344135_f64,
                l: 20_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.169220227926997_f64,
                l: 20_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.7399919228791405_f64,
                l: 20_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.3203528743824995_f64,
                l: 20_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.7227487141339379_f64,
                l: 20_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.6313829614634084_f64,
                l: 20_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.383833481822069_f64,
                l: 20_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler017_42: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data018_43: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 80.8124255873101_f64,
                l: 20_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -95.84962217254937_f64,
                l: 20_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 87.33792763032551_f64,
                l: 20_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -70.33089153398583_f64,
                l: 20_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 24.62909073517949_f64,
                l: 20_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 39.90257633891225_f64,
                l: 20_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -75.62120147139655_f64,
                l: 20_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 42.417415829726494_f64,
                l: 20_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 33.82684867887129_f64,
                l: 20_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -74.00276184082031_f64,
                l: 20_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 33.826848678871464_f64,
                l: 20_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 42.41741582972619_f64,
                l: 20_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -75.6212014713966_f64,
                l: 20_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 39.90257633891255_f64,
                l: 20_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 24.62909073517949_f64,
                l: 20_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -70.33089153398572_f64,
                l: 20_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 87.33792763032545_f64,
                l: 20_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -95.84962217254937_f64,
                l: 20_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 80.81242558730975_f64,
                l: 20_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler018_44: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data019_45: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 315702.3271513422_f64,
                l: 20_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -503060.9148485259_f64,
                l: 20_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 298127.28360361955_f64,
                l: 20_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 114444.61447464029_f64,
                l: 20_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -543428.4091459285_f64,
                l: 20_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 613842.0772818548_f64,
                l: 20_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -143765.4241127106_f64,
                l: 20_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -472600.45321372285_f64,
                l: 20_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 563861.7677149649_f64,
                l: 20_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -563861.7677149646_f64,
                l: 20_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 472600.45321372483_f64,
                l: 20_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 143765.42411270936_f64,
                l: 20_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -613842.0772818535_f64,
                l: 20_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 543428.4091459285_f64,
                l: 20_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -114444.61447464178_f64,
                l: 20_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -298127.2836036152_f64,
                l: 20_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 503060.9148485259_f64,
                l: 20_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -315702.3271513362_f64,
                l: 20_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler019_46: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data020_47: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 20_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 990017476694.9908_f64,
                l: 20_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2392757933281.05_f64,
                l: 20_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1548364524949.5808_f64,
                l: 20_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -424471915195.0563_f64,
                l: 20_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1744502295946.2073_f64,
                l: 20_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -899973487310.5553_f64,
                l: 20_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1092420454297.7119_f64,
                l: 20_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1466609267659.8816_f64,
                l: 20_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 356041756390.71674_f64,
                l: 20_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1612052956674.3164_f64,
                l: 20_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 356041756390.71985_f64,
                l: 20_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1466609267659.8796_f64,
                l: 20_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1092420454297.7161_f64,
                l: 20_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -899973487310.5481_f64,
                l: 20_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1744502295946.2073_f64,
                l: 20_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -424471915195.05896_f64,
                l: 20_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1548364524949.573_f64,
                l: 20_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2392757933281.05_f64,
                l: 20_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 990017476694.9883_f64,
                l: 20_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler020_48: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data021_49: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 19609049712023808_f64,
                l: 20_(),
                m: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 11693527616833220608_f64,
                l: 20_(),
                m: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 380734558806206906368_f64,
                l: 20_(),
                m: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3687400224900792713216_f64,
                l: 20_(),
                m: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8010806978179592e+22_f64,
                l: 20_(),
                m: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.593883258401247e+22_f64,
                l: 20_(),
                m: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.2454734132297759e+23_f64,
                l: 20_(),
                m: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1263407800797497e+23_f64,
                l: 20_(),
                m: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.8924941146976873e+23_f64,
                l: 20_(),
                m: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.1983098677287775e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.8924941146976873e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1263407800797497e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.245473413229781e+23_f64,
                l: 20_(),
                m: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.5938832584012366e+22_f64,
                l: 20_(),
                m: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8010806978179592e+22_f64,
                l: 20_(),
                m: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3687400224900780654592_f64,
                l: 20_(),
                m: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 380734558806203432960_f64,
                l: 20_(),
                m: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 11693527616833220608_f64,
                l: 20_(),
                m: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 19609049712023672_f64,
                l: 20_(),
                m: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 20_(),
                m: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler021_50: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data022_51: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1700376599438367_f64,
                l: 50_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.014572731645892852_f64,
                l: 50_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.058860798844002096_f64,
                l: 50_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.041569033381824674_f64,
                l: 50_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.1091105157471479_f64,
                l: 50_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.0834322722041975_f64,
                l: 50_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0382058126613136_f64,
                l: 50_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.11227517265921705_f64,
                l: 50_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.038205812661314155_f64,
                l: 50_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.08343227220419656_f64,
                l: 50_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10911051574714797_f64,
                l: 50_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.04156903338182601_f64,
                l: 50_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.05886079884400143_f64,
                l: 50_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.014572731645890737_f64,
                l: 50_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.1700376599438368_f64,
                l: 50_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler022_52: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data023_53: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.1342414998444949_f64,
                l: 50_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.201121967241302_f64,
                l: 50_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -6.6622414993232_f64,
                l: 50_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.577284693691925_f64,
                l: 50_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.878714881560761_f64,
                l: 50_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.547345945863408_f64,
                l: 50_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.8444956647620248_f64,
                l: 50_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.8722014306642127_f64,
                l: 50_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.348875132228563_f64,
                l: 50_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.348875132228552_f64,
                l: 50_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.872201430664262_f64,
                l: 50_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.844495664761993_f64,
                l: 50_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.547345945863381_f64,
                l: 50_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.878714881560761_f64,
                l: 50_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.577284693691945_f64,
                l: 50_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.662241499323218_f64,
                l: 50_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.201121967241302_f64,
                l: 50_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.13424149984438935_f64,
                l: 50_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler023_54: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data024_55: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 433.0416848371351_f64,
                l: 50_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -348.06364372056424_f64,
                l: 50_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 50.221071418108444_f64,
                l: 50_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 158.46096409274352_f64,
                l: 50_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 85.98885829972146_f64,
                l: 50_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -101.15891460879088_f64,
                l: 50_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -277.07168105316526_f64,
                l: 50_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -214.333113735104_f64,
                l: 50_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 96.34965793095166_f64,
                l: 50_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 286.30169028100346_f64,
                l: 50_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 96.34965793095324_f64,
                l: 50_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -214.33311373510165_f64,
                l: 50_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -277.0716810531662_f64,
                l: 50_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -101.15891460879435_f64,
                l: 50_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 85.98885829972146_f64,
                l: 50_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 158.46096409274153_f64,
                l: 50_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 50.22107141810314_f64,
                l: 50_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -348.06364372056424_f64,
                l: 50_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 433.04168483713596_f64,
                l: 50_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler024_56: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data025_57: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 27340473.95213283_f64,
                l: 50_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27753716.768532373_f64,
                l: 50_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -40808153.91349363_f64,
                l: 50_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -32071189.03579009_f64,
                l: 50_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -36265736.218529105_f64,
                l: 50_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -37089596.70020493_f64,
                l: 50_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -14562029.629244687_f64,
                l: 50_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 23686895.21751719_f64,
                l: 50_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 34878992.965676_f64,
                l: 50_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -34878992.96567596_f64,
                l: 50_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -23686895.21751753_f64,
                l: 50_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 14562029.629244499_f64,
                l: 50_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 37089596.70020478_f64,
                l: 50_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 36265736.218529105_f64,
                l: 50_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 32071189.035790242_f64,
                l: 50_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 40808153.913493834_f64,
                l: 50_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 27753716.768532373_f64,
                l: 50_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -27340473.952132136_f64,
                l: 50_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler025_58: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data026_59: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 50_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8994661710093155_f64,
                l: 50_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 932311375306569.6_f64,
                l: 50_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12153535011507012_f64,
                l: 50_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12176690755542240_f64,
                l: 50_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9180035388465754_f64,
                l: 50_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 889201701866984_f64,
                l: 50_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9451384032851544_f64,
                l: 50_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9926439446673564_f64,
                l: 50_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2794368162749970.5_f64,
                l: 50_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 11452238249246346_f64,
                l: 50_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2794368162750031_f64,
                l: 50_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9926439446673506_f64,
                l: 50_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9451384032851604_f64,
                l: 50_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 889201701866835.2_f64,
                l: 50_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9180035388465754_f64,
                l: 50_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12176690755542214_f64,
                l: 50_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 12153535011506908_f64,
                l: 50_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 932311375306569.6_f64,
                l: 50_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8994661710093013_f64,
                l: 50_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler026_60: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data027_61: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.66309251586455e+33_f64,
                l: 50_(),
                m: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0622676657892052e+33_f64,
                l: 50_(),
                m: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.602252116471711e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.0860128756808466e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.016986081427446e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.2324325279774e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.0054067236247267e+31_f64,
                l: 50_(),
                m: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.930926605643431e+32_f64,
                l: 50_(),
                m: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.415135829089898e+31_f64,
                l: 50_(),
                m: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.873593569733221e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.415135829089492e+31_f64,
                l: 50_(),
                m: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.930926605643445e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.005406723624373e+31_f64,
                l: 50_(),
                m: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.23243252797739e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.016986081427446e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.0860128756807846e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.602252116471629e+32_f64,
                l: 50_(),
                m: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0622676657892052e+33_f64,
                l: 50_(),
                m: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.663092515864554e+33_f64,
                l: 50_(),
                m: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler027_62: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data028_63: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 50_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.536699497443134e+60_f64,
                l: 50_(),
                m: 50_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2028834403101213e+67_f64,
                l: 50_(),
                m: 50_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.332549655956665e+71_f64,
                l: 50_(),
                m: 50_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.889809643178197e+73_f64,
                l: 50_(),
                m: 50_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.0509760257037188e+75_f64,
                l: 50_(),
                m: 50_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.4866724533443283e+76_f64,
                l: 50_(),
                m: 50_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.5790740224149893e+77_f64,
                l: 50_(),
                m: 50_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.822223793168099e+77_f64,
                l: 50_(),
                m: 50_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1198682190366617e+78_f64,
                l: 50_(),
                m: 50_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.7253921397507295e+78_f64,
                l: 50_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.1198682190366617e+78_f64,
                l: 50_(),
                m: 50_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.822223793168099e+77_f64,
                l: 50_(),
                m: 50_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.5790740224150207e+77_f64,
                l: 50_(),
                m: 50_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.486672453344312e+76_f64,
                l: 50_(),
                m: 50_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.0509760257037188e+75_f64,
                l: 50_(),
                m: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.8898096431781724e+73_f64,
                l: 50_(),
                m: 50_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.3325496559566344e+71_f64,
                l: 50_(),
                m: 50_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.2028834403101213e+67_f64,
                l: 50_(),
                m: 50_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 2.5366994974430855e+60_f64,
                l: 50_(),
                m: 50_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 50_(),
                m: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler028_64: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data029_65: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                m: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10226582055871893_f64,
                l: 100_(),
                m: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                m: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.07713250719977864_f64,
                l: 100_(),
                m: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.02374702390513314_f64,
                l: 100_(),
                m: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                m: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.07225820212568503_f64,
                l: 100_(),
                m: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.057127392202801046_f64,
                l: 100_(),
                m: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.014681835355659706_f64,
                l: 100_(),
                m: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0638950984347502_f64,
                l: 100_(),
                m: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.07958923738717873_f64,
                l: 100_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.06389509843474976_f64,
                l: 100_(),
                m: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.014681835355657875_f64,
                l: 100_(),
                m: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.057127392202801566_f64,
                l: 100_(),
                m: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.07225820212568408_f64,
                l: 100_(),
                m: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                m: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.023747023905134217_f64,
                l: 100_(),
                m: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0.0771325071997805_f64,
                l: 100_(),
                m: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                m: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0.10226582055872063_f64,
                l: 100_(),
                m: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                m: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler029_66: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data030_67: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -6.5200167187780345_f64,
                l: 100_(),
                m: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -9.006517000702749_f64,
                l: 100_(),
                m: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.469090854118098_f64,
                l: 100_(),
                m: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.627543917043079_f64,
                l: 100_(),
                m: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.090903166344845_f64,
                l: 100_(),
                m: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.1160338699560395_f64,
                l: 100_(),
                m: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.849104301075863_f64,
                l: 100_(),
                m: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.943513872308983_f64,
                l: 100_(),
                m: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4.7996285823989355_f64,
                l: 100_(),
                m: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.79962858239901_f64,
                l: 100_(),
                m: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.9435138723090155_f64,
                l: 100_(),
                m: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.849104301075801_f64,
                l: 100_(),
                m: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.116033869956216_f64,
                l: 100_(),
                m: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -6.090903166344845_f64,
                l: 100_(),
                m: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.627543917043047_f64,
                l: 100_(),
                m: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -5.469090854117869_f64,
                l: 100_(),
                m: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.006517000702749_f64,
                l: 100_(),
                m: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.520016718777779_f64,
                l: 100_(),
                m: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler030_68: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data031_69: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1005.9604880761002_f64,
                l: 100_(),
                m: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -489.6804172586595_f64,
                l: 100_(),
                m: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 768.3167601166992_f64,
                l: 100_(),
                m: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 226.90362556627937_f64,
                l: 100_(),
                m: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 604.1988930494034_f64,
                l: 100_(),
                m: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 733.4006103783852_f64,
                l: 100_(),
                m: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -573.3077448399563_f64,
                l: 100_(),
                m: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -151.5294630508088_f64,
                l: 100_(),
                m: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 646.3052558358798_f64,
                l: 100_(),
                m: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -803.8512976105052_f64,
                l: 100_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 646.3052558358744_f64,
                l: 100_(),
                m: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -151.52946305079013_f64,
                l: 100_(),
                m: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -573.3077448399639_f64,
                l: 100_(),
                m: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 733.4006103783755_f64,
                l: 100_(),
                m: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 604.1988930494034_f64,
                l: 100_(),
                m: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 226.90362556629168_f64,
                l: 100_(),
                m: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 768.3167601167177_f64,
                l: 100_(),
                m: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -489.6804172586595_f64,
                l: 100_(),
                m: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1005.9604880761161_f64,
                l: 100_(),
                m: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler031_70: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data032_71: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -900551126.0965381_f64,
                l: 100_(),
                m: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -988567431.5575614_f64,
                l: 100_(),
                m: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 645646451.9034462_f64,
                l: 100_(),
                m: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 897114585.2992069_f64,
                l: 100_(),
                m: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 661710744.4248385_f64,
                l: 100_(),
                m: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -380163158.51424754_f64,
                l: 100_(),
                m: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -617391071.3663319_f64,
                l: 100_(),
                m: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 805288801.8550911_f64,
                l: 100_(),
                m: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -481041740.1672865_f64,
                l: 100_(),
                m: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 481041740.1672939_f64,
                l: 100_(),
                m: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -805288801.8550951_f64,
                l: 100_(),
                m: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 617391071.3663257_f64,
                l: 100_(),
                m: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 380163158.51426536_f64,
                l: 100_(),
                m: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -661710744.4248385_f64,
                l: 100_(),
                m: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -897114585.2992048_f64,
                l: 100_(),
                m: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -645646451.9034228_f64,
                l: 100_(),
                m: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 988567431.5575614_f64,
                l: 100_(),
                m: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 900551126.0965164_f64,
                l: 100_(),
                m: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -0_f64,
                l: 100_(),
                m: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler032_72: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data033_73: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 256433959576586016_f64,
                l: 100_(),
                m: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1577867354567348480_f64,
                l: 100_(),
                m: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4435504848749680128_f64,
                l: 100_(),
                m: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -959361116591242880_f64,
                l: 100_(),
                m: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4238712302196343808_f64,
                l: 100_(),
                m: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8237083461842676736_f64,
                l: 100_(),
                m: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4908935838805197824_f64,
                l: 100_(),
                m: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2346881035809127424_f64,
                l: 100_(),
                m: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6862785522503456768_f64,
                l: 100_(),
                m: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8249459718167037952_f64,
                l: 100_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6862785522503405568_f64,
                l: 100_(),
                m: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2346881035808951808_f64,
                l: 100_(),
                m: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -4908935838805294080_f64,
                l: 100_(),
                m: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8237083461842611200_f64,
                l: 100_(),
                m: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4238712302196343808_f64,
                l: 100_(),
                m: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -959361116591126400_f64,
                l: 100_(),
                m: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4435504848749966848_f64,
                l: 100_(),
                m: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1577867354567348480_f64,
                l: 100_(),
                m: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 256433959576300576_f64,
                l: 100_(),
                m: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler033_74: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data034_75: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.160434487878013e+37_f64,
                l: 100_(),
                m: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.396389511696223e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.902223685311014e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.268000557412101e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.0750497039999552e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.612058604384356e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.147449689190092e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.396625130744424e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.050326645170259e+38_f64,
                l: 100_(),
                m: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.772743983615958e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.050326645170221e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.396625130744278e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.1474496891901797e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -7.612058604384418e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.0750497039999552e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.26800055741204e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.902223685310842e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.396389511696223e+38_f64,
                l: 100_(),
                m: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.160434487875185e+37_f64,
                l: 100_(),
                m: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler034_76: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data035_77: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 50_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.323127851689372e+96_f64,
                l: 100_(),
                m: 50_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1029797977454281e+98_f64,
                l: 100_(),
                m: 50_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8089333903465606e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.9364045925669405e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.225262033972712e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.143138509374073e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.3520602067479935e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.7791149588121382e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.011933855018042e+97_f64,
                l: 100_(),
                m: 50_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1712145031578381e+98_f64,
                l: 100_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.01193385501812e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -2.7791149588123644e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -3.3520602067479344e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 7.143138509373882e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -8.225262033972712e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.936404592566802e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.8089333903469005e+97_f64,
                l: 100_(),
                m: 50_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: -1.1029797977454281e+98_f64,
                l: 100_(),
                m: 50_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 9.323127851689294e+96_f64,
                l: 100_(),
                m: 50_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler035_78: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data036_79: Value<Box<[testcase_assoc_legendre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 100_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.7751792255758316e+150_f64,
                l: 100_(),
                m: 100_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.3552236041585515e+164_f64,
                l: 100_(),
                m: 100_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.5936546850595123e+172_f64,
                l: 100_(),
                m: 100_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.3579510590289176e+177_f64,
                l: 100_(),
                m: 100_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.7752749682889513e+180_f64,
                l: 100_(),
                m: 100_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0910627330458913e+183_f64,
                l: 100_(),
                m: 100_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.9697347526821064e+184_f64,
                l: 100_(),
                m: 100_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.658587914752671e+185_f64,
                l: 100_(),
                m: 100_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.033157190805701e+186_f64,
                l: 100_(),
                m: 100_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 6.666308670072954e+186_f64,
                l: 100_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.033157190805701e+186_f64,
                l: 100_(),
                m: 100_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 8.658587914752671e+185_f64,
                l: 100_(),
                m: 100_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.969734752682248e+184_f64,
                l: 100_(),
                m: 100_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.0910627330458797e+183_f64,
                l: 100_(),
                m: 100_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 3.7752749682889513e+180_f64,
                l: 100_(),
                m: 100_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.3579510590289e+177_f64,
                l: 100_(),
                m: 100_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 1.5936546850594382e+172_f64,
                l: 100_(),
                m: 100_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 4.3552236041585515e+164_f64,
                l: 100_(),
                m: 100_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 5.775179225575613e+150_f64,
                l: 100_(),
                m: 100_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_assoc_legendre_double_ {
                f0: 0_f64,
                l: 100_(),
                m: 100_(),
                x: 1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler036_80: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
pub fn test_81(data: Ptr<testcase_assoc_legendre_double_>, mut toler: ()) {
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
        test_81(
            data001_9.with(|v| v.as_pointer()),
            toler001_10.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data002_11.with(|v| v.as_pointer()),
            toler002_12.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data003_13.with(|v| v.as_pointer()),
            toler003_14.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data004_15.with(|v| v.as_pointer()),
            toler004_16.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data005_17.with(|v| v.as_pointer()),
            toler005_18.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data006_19.with(|v| v.as_pointer()),
            toler006_20.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data007_21.with(|v| v.as_pointer()),
            toler007_22.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data008_23.with(|v| v.as_pointer()),
            toler008_24.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data009_25.with(|v| v.as_pointer()),
            toler009_26.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data010_27.with(|v| v.as_pointer()),
            toler010_28.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data011_29.with(|v| v.as_pointer()),
            toler011_30.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data012_31.with(|v| v.as_pointer()),
            toler012_32.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data013_33.with(|v| v.as_pointer()),
            toler013_34.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data014_35.with(|v| v.as_pointer()),
            toler014_36.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data015_37.with(|v| v.as_pointer()),
            toler015_38.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data016_39.with(|v| v.as_pointer()),
            toler016_40.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data017_41.with(|v| v.as_pointer()),
            toler017_42.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data018_43.with(|v| v.as_pointer()),
            toler018_44.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data019_45.with(|v| v.as_pointer()),
            toler019_46.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data020_47.with(|v| v.as_pointer()),
            toler020_48.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data021_49.with(|v| v.as_pointer()),
            toler021_50.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data022_51.with(|v| v.as_pointer()),
            toler022_52.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data023_53.with(|v| v.as_pointer()),
            toler023_54.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data024_55.with(|v| v.as_pointer()),
            toler024_56.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data025_57.with(|v| v.as_pointer()),
            toler025_58.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data026_59.with(|v| v.as_pointer()),
            toler026_60.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data027_61.with(|v| v.as_pointer()),
            toler027_62.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data028_63.with(|v| v.as_pointer()),
            toler028_64.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data029_65.with(|v| v.as_pointer()),
            toler029_66.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data030_67.with(|v| v.as_pointer()),
            toler030_68.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data031_69.with(|v| v.as_pointer()),
            toler031_70.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data032_71.with(|v| v.as_pointer()),
            toler032_72.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data033_73.with(|v| v.as_pointer()),
            toler033_74.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data034_75.with(|v| v.as_pointer()),
            toler034_76.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data035_77.with(|v| v.as_pointer()),
            toler035_78.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_81(
            data036_79.with(|v| v.as_pointer()),
            toler036_80.with(|rc| *rc.borrow()),
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
pub fn set_memory_limits_82(size: Option<()>) {
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
pub fn set_file_limit_83(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_84(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_85(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_86(
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
pub fn test_tm_87(
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
    (*tmp_88.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_88.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_88.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_88: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
