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
pub struct testcase_laguerre_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub n: (),
    #[offset(16)]
    pub x: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data001_9: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -4_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -14_f64,
                n: 1_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -24_f64,
                n: 1_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -34_f64,
                n: 1_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -44_f64,
                n: 1_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -54_f64,
                n: 1_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -64_f64,
                n: 1_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -74_f64,
                n: 1_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -84_f64,
                n: 1_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -89_f64,
                n: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -94_f64,
                n: 1_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -99_f64,
                n: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3.5_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 31_f64,
                n: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 83.5_f64,
                n: 2_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 161_f64,
                n: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 263.5_f64,
                n: 2_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 391_f64,
                n: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 543.5_f64,
                n: 2_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 721_f64,
                n: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 923.5_f64,
                n: 2_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1151_f64,
                n: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1403.5_f64,
                n: 2_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1681_f64,
                n: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1983.5_f64,
                n: 2_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2311_f64,
                n: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2663.5_f64,
                n: 2_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3041_f64,
                n: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3443.5_f64,
                n: 2_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3871_f64,
                n: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 4323.5_f64,
                n: 2_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 4801_f64,
                n: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -3.1666666666666665_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 34.33333333333333_f64,
                n: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -355.25_f64,
                n: 5_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -4765.666666666667_f64,
                n: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -23040.666666666664_f64,
                n: 5_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -74399_f64,
                n: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -190559.41666666663_f64,
                n: 5_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -418865.6666666666_f64,
                n: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -825411.5_f64,
                n: 5_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -1498165.6666666665_f64,
                n: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -2550096.916666667_f64,
                n: 5_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -4122299_f64,
                n: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -6387115.666666667_f64,
                n: 5_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -9551265.666666668_f64,
                n: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -13858967.75_f64,
                n: 5_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -19595065.666666664_f64,
                n: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -27088153.166666668_f64,
                n: 5_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -36713699_f64,
                n: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -48897171.91666666_f64,
                n: 5_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -64117165.666666664_f64,
                n: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1.756276179453263_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 27.984126984126977_f64,
                n: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -237.51841517857147_f64,
                n: 10_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3227.8077601410932_f64,
                n: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -45786.19979745369_f64,
                n: 10_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 15129.571428571455_f64,
                n: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 7764800.817949459_f64,
                n: 10_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 79724066.60846558_f64,
                n: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 469865425.65122765_f64,
                n: 10_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2037190065.373898_f64,
                n: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 7187828002.982576_f64,
                n: 10_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 21804200401_f64,
                n: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 58854343015.61621_f64,
                n: 10_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 144688291819.51855_f64,
                n: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 329425241736.7004_f64,
                n: 10_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 703324772760.0828_f64,
                n: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1421627560118.6157_f64,
                n: 10_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2741055412243.857_f64,
                n: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 5071986977681.865_f64,
                n: 10_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 9051283795429.572_f64,
                n: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2.0202257444769134_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -11.961333867812119_f64,
                n: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -50.151037960139455_f64,
                n: 20_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2829.4728613531743_f64,
                n: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -11583.94789911354_f64,
                n: 20_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -18439.42450252094_f64,
                n: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -38838.223606979285_f64,
                n: 20_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 24799805.877530713_f64,
                n: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -673953823.5991328_f64,
                n: 20_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 7551960453.767255_f64,
                n: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 31286508510.614746_f64,
                n: 20_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -1379223608444.9155_f64,
                n: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -6692517968212.972_f64,
                n: 20_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 165423821874449.94_f64,
                n: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3082390018008546.5_f64,
                n: 20_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 29500368536981676_f64,
                n: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 203535263549741856_f64,
                n: 20_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1129230951443290112_f64,
                n: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 5323926285556310016_f64,
                n: 20_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 22061882785931735040_f64,
                n: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1.4735258819430543_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 17.534183446338233_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -195.6243661907738_f64,
                n: 50_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 980.2696188979103_f64,
                n: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 24812.277673870878_f64,
                n: 50_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 293000.5073596236_f64,
                n: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2316195.5013375278_f64,
                n: 50_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -14896937.968694873_f64,
                n: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -502066598.00813466_f64,
                n: 50_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2513677852.691687_f64,
                n: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 45129675503.53891_f64,
                n: 50_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -883876565337.9922_f64,
                n: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 9361319947203.842_f64,
                n: 50_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -80967880733583.23_f64,
                n: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 717391079438942.6_f64,
                n: 50_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -8217471769564841_f64,
                n: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 125952762290099776_f64,
                n: 50_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -2114003130804889088_f64,
                n: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 32438187475835133952_f64,
                n: 50_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -397101034870946922496_f64,
                n: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_laguerre_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1.45552716253288_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 13.27766284430345_f64,
                n: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 91.73703845434245_f64,
                n: 100_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1854.0367283243388_f64,
                n: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -11281.698886837261_f64,
                n: 100_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 170141.8698704655_f64,
                n: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -2950092.7025822806_f64,
                n: 100_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -7272442.315600696_f64,
                n: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 295697471.9087616_f64,
                n: 100_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 4847420871.269051_f64,
                n: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 59406998102.39229_f64,
                n: 100_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 693492765740.2969_f64,
                n: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 6606192010150.315_f64,
                n: 100_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 17125518672239.77_f64,
                n: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -870493767065150.1_f64,
                n: 100_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -13763178176383768_f64,
                n: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 30667078414479584_f64,
                n: 100_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2130722049038017280_f64,
                n: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -7270652300900782080_f64,
                n: 100_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -262922606930689163264_f64,
                n: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
pub fn test_25(data: Ptr<testcase_laguerre_double_>, mut toler: ()) {
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
