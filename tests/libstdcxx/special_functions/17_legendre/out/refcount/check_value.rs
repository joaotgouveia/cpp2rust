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
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"copy_constructor::mark_call",
                ))
            });
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
        return count__4.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__4.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__4.with(|rc| *rc.borrow()) == throw_on__5.with(|rc| *rc.borrow())) {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"assignment_operator::mark_call",
                ))
            });
        }
    }
    pub fn reset() {
        count__4.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__5.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__5.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> () {
        return _M_count_6.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_6.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_6.with(|rc| *rc.borrow_mut() = 0_());
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
            id.unwrap_or_else(|| (*next_id__7.with(Value::clone).borrow_mut()).postfix_dec());
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
impl std::cmp::Ord for __gnu_test_copy_tracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_8(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for __gnu_test_copy_tracker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for __gnu_test_copy_tracker {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_9(
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: self.id_.clone(),
                    throw_on_copy_: self.throw_on_copy_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(__gnu_test_copy_tracker {
                    id_: other.id_.clone(),
                    throw_on_copy_: other.throw_on_copy_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copy_tracker {}
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
pub fn operator_eq_9(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_8(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
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
    pid_: ::libc::pid_t,
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
    pub static data001_10: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler001_11: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_12: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler002_13: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_14: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler003_15: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_16: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler004_17: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_18: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler005_19: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_20: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler006_21: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_22: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler007_23: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_24: Value<Box<[testcase_legendre_double_]>> =
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
    pub static toler008_25: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
pub fn test_26(data: Ptr<testcase_legendre_double_>, mut toler: ()) {
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
            ({
                __builtin_fprintf_27 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"check_value.cc" )  ) .into() , ( 328_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test(const testcase_legendre<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"max_abs_frac < toler" )  ) .into() , ] ) )
            });
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
        test_26(
            data001_10.with(|v| v.as_pointer()),
            toler001_11.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_26(
            data002_12.with(|v| v.as_pointer()),
            toler002_13.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_26(
            data003_14.with(|v| v.as_pointer()),
            toler003_15.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_26(
            data004_16.with(|v| v.as_pointer()),
            toler004_17.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_26(
            data005_18.with(|v| v.as_pointer()),
            toler005_19.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_26(
            data006_20.with(|v| v.as_pointer()),
            toler006_21.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_26(
            data007_22.with(|v| v.as_pointer()),
            toler007_23.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_26(
            data008_24.with(|v| v.as_pointer()),
            toler008_25.with(|rc| *rc.borrow()),
        )
    });
    return 0_();
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write((({ getpid_28() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_29(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
        if (this.with(|__s| __s.sem_set_) == -1_()) {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not obtain semaphore set",
                ))
            });
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if (({
            semctl_30(
                this.with(|__s| __s.sem_set_),
                0_(),
                16_(),
                Box::new([((*val.borrow()).clone()).into()]),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not initialize semaphore",
                ))
            });
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_31(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_32((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_32((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_32((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_34(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_32((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_33((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_35(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_36(
            (mangled).clone(),
            Ptr::<()>::null(),
            Ptr::<()>::null(),
            (status.as_pointer()),
        )
    });
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
    let mut w: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { (wanted).clone() },
        None,
    );
    if unimplemented!() {
        ({ __throw_runtime_error_3((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_37(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<()> = unimplemented!();
    if !(res).is_null() {
        let mut preLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (res).clone() },
                None,
            );
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { unimplemented!() },
                None,
            );
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                ({
                    __builtin_fprintf_27 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { Ptr::<()>::from_string_literal(b"LC_ALL for ") },
            None,
        );
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
    }
}
pub fn run_tests_wrapped_env_38(
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
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { (env).clone() },
            None,
        );
        unimplemented!();
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
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
    static count__4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static _M_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static next_id__7: Value<()> = Rc::new(RefCell::new(0_()));
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
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<semid_ds>> {
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
pub fn test_tm_40(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> ::libc::tm {
    (*tmp_41.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_41.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_41.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_41: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_28() })) {
            ({
                semctl_30(
                    (*self).with(|__s| __s.sem_set_),
                    0_(),
                    0_(),
                    Box::new([((*val.borrow()).clone()).into()]),
                )
            });
        }
    }
    fn signal(&self) {
        let op: Value<Box<[sembuf]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (({
            semop_39(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not signal semaphore",
                ))
            });
        }
    }
    fn wait(&self) {
        let op: Value<Box<[sembuf]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (({
            semop_39(
                (*self).with(|__s| __s.sem_set_),
                (op.as_pointer() as Ptr<sembuf>),
                1_(),
            )
        }) == -1_())
        {
            ({
                __throw_runtime_error_3(Ptr::<()>::from_string_literal(
                    b"could not wait for semaphore",
                ))
            });
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = data001_10.with(|_| ());
    let _ = toler001_11.with(|_| ());
    let _ = data002_12.with(|_| ());
    let _ = toler002_13.with(|_| ());
    let _ = data003_14.with(|_| ());
    let _ = toler003_15.with(|_| ());
    let _ = data004_16.with(|_| ());
    let _ = toler004_17.with(|_| ());
    let _ = data005_18.with(|_| ());
    let _ = toler005_19.with(|_| ());
    let _ = data006_20.with(|_| ());
    let _ = toler006_21.with(|_| ());
    let _ = data007_22.with(|_| ());
    let _ = toler007_23.with(|_| ());
    let _ = data008_24.with(|_| ());
    let _ = toler008_25.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__4.with(|_| ());
    let _ = throw_on__5.with(|_| ());
    let _ = _M_count_6.with(|_| ());
    let _ = next_id__7.with(|_| ());
}
