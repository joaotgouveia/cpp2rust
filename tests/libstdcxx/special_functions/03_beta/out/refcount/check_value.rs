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
pub struct testcase_beta_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub x: (),
    #[offset(16)]
    pub y: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data001_9: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 1.0825088224469029e-06_f64,
            x: 10_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.992508740634678e-09_f64,
            x: 10_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.5729567312509485e-10_f64,
            x: 10_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.2168673582561288e-11_f64,
            x: 10_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.5916380099863291e-12_f64,
            x: 10_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.9408957938463963e-13_f64,
            x: 10_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.941163798069168e-14_f64,
            x: 10_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.966561297250265e-14_f64,
            x: 10_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.41878248281544e-15_f64,
            x: 10_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.3455339739604842e-15_f64,
            x: 10_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 4.992508740634678e-09_f64,
            x: 20_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 7.254444551924844e-13_f64,
            x: 20_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.7681885473062028e-15_f64,
            x: 20_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.7891885039182335e-17_f64,
            x: 20_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.3240677875623635e-19_f64,
            x: 20_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.8857342309689053e-20_f64,
            x: 20_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.2609804003539998e-21_f64,
            x: 20_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.1660809542079041e-22_f64,
            x: 20_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.390794427972907e-23_f64,
            x: 20_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.0365059099917614e-24_f64,
            x: 20_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 1.5729567312509485e-10_f64,
            x: 30_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.7681885473062028e-15_f64,
            x: 30_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 5.637077964048245e-19_f64,
            x: 30_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.0539424603796547e-21_f64,
            x: 30_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.011819777727384e-24_f64,
            x: 30_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 7.427952855326015e-26_f64,
            x: 30_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.6212207780604767e-27_f64,
            x: 30_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 5.478372971531762e-29_f64,
            x: 30_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.6183005659681346e-30_f64,
            x: 30_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.658794822212223e-31_f64,
            x: 30_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 1.2168673582561288e-11_f64,
            x: 40_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.7891885039182335e-17_f64,
            x: 40_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.0539424603796547e-21_f64,
            x: 40_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.650850914009066e-25_f64,
            x: 40_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 7.516171211855773e-28_f64,
            x: 40_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.031133197988607e-30_f64,
            x: 40_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.4175035070466313e-32_f64,
            x: 40_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.273483914275837e-34_f64,
            x: 40_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.769062960131558e-36_f64,
            x: 40_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.9797337118810115e-37_f64,
            x: 40_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 1.5916380099863291e-12_f64,
            x: 50_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.3240677875623635e-19_f64,
            x: 50_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.011819777727384e-24_f64,
            x: 50_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 7.516171211855773e-28_f64,
            x: 50_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.964661208567414e-31_f64,
            x: 50_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 5.84256439064184e-34_f64,
            x: 50_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.8672362180783552e-36_f64,
            x: 50_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.0939382296458963e-38_f64,
            x: 50_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.0442781609879874e-40_f64,
            x: 50_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.490412111095437e-42_f64,
            x: 50_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 2.9408957938463963e-13_f64,
            x: 60_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.8857342309689053e-20_f64,
            x: 60_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 7.427952855326015e-26_f64,
            x: 60_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.031133197988607e-30_f64,
            x: 60_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 5.84256439064184e-34_f64,
            x: 60_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.450123146978223e-37_f64,
            x: 60_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.77068553860866e-40_f64,
            x: 60_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.2902663809721126e-42_f64,
            x: 60_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.010557105857051e-45_f64,
            x: 60_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.392289889834721e-47_f64,
            x: 60_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 6.941163798069168e-14_f64,
            x: 70_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.2609804003539998e-21_f64,
            x: 70_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.6212207780604767e-27_f64,
            x: 70_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.4175035070466313e-32_f64,
            x: 70_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.8672362180783552e-36_f64,
            x: 70_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.77068553860866e-40_f64,
            x: 70_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.045313714348291e-43_f64,
            x: 70_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.019227408201378e-46_f64,
            x: 70_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 9.58658700635018e-49_f64,
            x: 70_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.74091273058198e-51_f64,
            x: 70_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 1.966561297250265e-14_f64,
            x: 80_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.1660809542079041e-22_f64,
            x: 80_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 5.478372971531762e-29_f64,
            x: 80_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.273483914275837e-34_f64,
            x: 80_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.0939382296458963e-38_f64,
            x: 80_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.2902663809721126e-42_f64,
            x: 80_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.019227408201378e-46_f64,
            x: 80_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.716059082866941e-49_f64,
            x: 80_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.459377390212537e-52_f64,
            x: 80_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 7.480703996850347e-55_f64,
            x: 80_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data009_25: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 6.41878248281544e-15_f64,
            x: 90_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.390794427972907e-23_f64,
            x: 90_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.6183005659681346e-30_f64,
            x: 90_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.769062960131558e-36_f64,
            x: 90_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.0442781609879874e-40_f64,
            x: 90_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 6.010557105857051e-45_f64,
            x: 90_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 9.58658700635018e-49_f64,
            x: 90_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.459377390212537e-52_f64,
            x: 90_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.4416737907558036e-55_f64,
            x: 90_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.023853191656425e-58_f64,
            x: 90_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler009_26: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data010_27: Value<Box<[testcase_beta_double_]>> = Rc::new(RefCell::new(Box::new([
        testcase_beta_double_ {
            f0: 2.3455339739604842e-15_f64,
            x: 100_f64,
            y: 10_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.0365059099917614e-24_f64,
            x: 100_f64,
            y: 20_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.658794822212223e-31_f64,
            x: 100_f64,
            y: 30_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.9797337118810115e-37_f64,
            x: 100_f64,
            y: 40_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 1.490412111095437e-42_f64,
            x: 100_f64,
            y: 50_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 4.392289889834721e-47_f64,
            x: 100_f64,
            y: 60_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.74091273058198e-51_f64,
            x: 100_f64,
            y: 70_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 7.480703996850347e-55_f64,
            x: 100_f64,
            y: 80_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 3.023853191656425e-58_f64,
            x: 100_f64,
            y: 90_f64,
            f: 0_f64,
        },
        testcase_beta_double_ {
            f0: 2.208760693199185e-61_f64,
            x: 100_f64,
            y: 100_f64,
            f: 0_f64,
        },
    ])));
);
thread_local!(
    pub static toler010_28: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
pub fn test_29(data: Ptr<testcase_beta_double_>, mut toler: ()) {
    let mut test: () = true;
    let mut eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 10_();
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
        test_29(
            data001_9.with(|v| v.as_pointer()),
            toler001_10.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data002_11.with(|v| v.as_pointer()),
            toler002_12.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data003_13.with(|v| v.as_pointer()),
            toler003_14.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data004_15.with(|v| v.as_pointer()),
            toler004_16.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data005_17.with(|v| v.as_pointer()),
            toler005_18.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data006_19.with(|v| v.as_pointer()),
            toler006_20.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data007_21.with(|v| v.as_pointer()),
            toler007_22.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data008_23.with(|v| v.as_pointer()),
            toler008_24.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data009_25.with(|v| v.as_pointer()),
            toler009_26.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_29(
            data010_27.with(|v| v.as_pointer()),
            toler010_28.with(|rc| *rc.borrow()),
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
pub fn set_memory_limits_30(size: Option<()>) {
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
pub fn set_file_limit_31(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_32(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_33(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_34(
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
pub fn test_tm_35(
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
    (*tmp_36.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_36.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_36.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_36: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
