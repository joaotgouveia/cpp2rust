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
pub struct testcase_comp_ellint_3_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub k: (),
    #[offset(16)]
    pub nu: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data001_9: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 2.2805491384227703_f64,
                k: -0.9_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.4295011187834885_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.607683574334841_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.8256506968858512_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.100068986857862_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.4591069002104677_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.954993988357023_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.6985482312992435_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.982074081364571_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 8.99425620318587_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.9953027776647294_f64,
                k: -0.8_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.1172616484005085_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.26247894341868_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.43920420027257_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.660403703552973_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.947878115823975_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.3418121892288055_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.9268876980046397_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.924642205819607_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 7.226325929863713_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.8456939983747234_f64,
                k: -0.7_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.9541347343119564_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0829290325820202_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.2392290510988535_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.434250291530788_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.6868019968236996_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.031457349674674_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.5408408771788564_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.404240572907696_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 6.379609417788775_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.7507538029157526_f64,
                k: -0.6_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.8508766487100685_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.9695980282802217_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.11341544050606_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.292503642098513_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.523900708449271_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.838872309951497_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.302973589839716_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.086703640926183_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.87099931162656_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.685750354812596_f64,
                k: -0.5_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.7803034946545482_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.892294761226402_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0277924458111314_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.1962905366178065_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.4136715042011945_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.709049186175356_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.143394529785923_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.875070188810807_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.535513209602646_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.6399998658645112_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.730696883684719_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.8380358826317627_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.967792413252014_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.1289968719280026_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.3367461373176512_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.618694020985019_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.0327078743873246_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.72895480021999_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.305553510287251_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.6080486199305128_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.6960848815118226_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.80021733722905_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.9260216862473254_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0822121773175533_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.283350588193397_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.5560975528589065_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.9562123549913877_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.628305048456717_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.1479514944016795_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.5868678474541662_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.6731552050562593_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.7751816279738935_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.8983924169967101_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0512956926676806_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.2481046259421302_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.5148333891629315_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.9058704854500967_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.5622166386422633_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.044826935620037_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data009_25: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.574745561517356_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.6600374067558428_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.760865611508342_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.8826015946315438_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.033636740307676_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.227986891296685_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.4913004919173827_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.8771910188009744_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.5246199613295617_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.986289041730551_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_26: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data010_27: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.5707963267948966_f64,
                k: 0_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.655764710966017_f64,
                k: 0_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.7562036827601817_f64,
                k: 0_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.877460709222638_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.027889337986806_f64,
                k: 0_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.221441469079183_f64,
                k: 0_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.4836470664490258_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.8678686047727386_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.5124073655203634_f64,
                k: 0_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.9672941328980516_f64,
                k: 0_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_28: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data011_29: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.574745561517356_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.6600374067558428_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.760865611508342_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.882601594631544_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.033636740307676_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.227986891296685_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.4913004919173827_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.8771910188009744_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.5246199613295617_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.986289041730551_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_30: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data012_31: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.5868678474541662_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.6731552050562593_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.7751816279738935_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.8983924169967101_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0512956926676806_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.2481046259421302_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.5148333891629315_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.9058704854500967_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.5622166386422633_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.044826935620037_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_32: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data013_33: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.6080486199305128_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.6960848815118228_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.80021733722905_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.9260216862473254_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0822121773175533_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.2833505881933975_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.5560975528589065_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.9562123549913877_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.6283050484567174_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.1479514944016795_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_34: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data014_35: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.6399998658645112_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.730696883684719_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.8380358826317629_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.9677924132520141_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.128996871928003_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.3367461373176512_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.6186940209850196_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.0327078743873246_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.7289548002199906_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.305553510287252_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_36: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data015_37: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.685750354812596_f64,
                k: 0.5_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.7803034946545482_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.892294761226402_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0277924458111314_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.1962905366178065_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.4136715042011945_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.709049186175356_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.143394529785923_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.875070188810807_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.535513209602646_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler015_38: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data016_39: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.7507538029157526_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.8508766487100687_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.9695980282802217_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.11341544050606_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.292503642098513_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.523900708449271_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.8388723099514976_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.302973589839716_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.086703640926183_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.870999311626561_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler016_40: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data017_41: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.8456939983747238_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 1.9541347343119566_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.0829290325820207_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.239229051098854_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.434250291530788_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.6868019968237_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.0314573496746746_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.540840877178857_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.404240572907697_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 6.379609417788776_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler017_42: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data018_43: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 1.9953027776647294_f64,
                k: 0.8_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.1172616484005085_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.26247894341868_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.43920420027257_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.660403703552973_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.947878115823975_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.3418121892288055_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.9268876980046397_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.924642205819607_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 7.226325929863713_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler018_44: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data019_45: Value<Box<[testcase_comp_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_comp_ellint_3_double_ {
                f0: 2.2805491384227707_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.429501118783489_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.607683574334842_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 2.825650696885852_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.100068986857863_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.4591069002104686_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 3.954993988357024_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 4.698548231299245_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 5.982074081364573_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                f: 0_f64,
            },
            testcase_comp_ellint_3_double_ {
                f0: 8.994256203185873_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler019_46: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
pub fn test_47(data: Ptr<testcase_comp_ellint_3_double_>, mut toler: ()) {
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
        test_47(
            data001_9.with(|v| v.as_pointer()),
            toler001_10.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data002_11.with(|v| v.as_pointer()),
            toler002_12.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data003_13.with(|v| v.as_pointer()),
            toler003_14.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data004_15.with(|v| v.as_pointer()),
            toler004_16.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data005_17.with(|v| v.as_pointer()),
            toler005_18.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data006_19.with(|v| v.as_pointer()),
            toler006_20.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data007_21.with(|v| v.as_pointer()),
            toler007_22.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data008_23.with(|v| v.as_pointer()),
            toler008_24.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data009_25.with(|v| v.as_pointer()),
            toler009_26.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data010_27.with(|v| v.as_pointer()),
            toler010_28.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data011_29.with(|v| v.as_pointer()),
            toler011_30.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data012_31.with(|v| v.as_pointer()),
            toler012_32.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data013_33.with(|v| v.as_pointer()),
            toler013_34.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data014_35.with(|v| v.as_pointer()),
            toler014_36.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data015_37.with(|v| v.as_pointer()),
            toler015_38.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data016_39.with(|v| v.as_pointer()),
            toler016_40.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data017_41.with(|v| v.as_pointer()),
            toler017_42.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data018_43.with(|v| v.as_pointer()),
            toler018_44.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_47(
            data019_45.with(|v| v.as_pointer()),
            toler019_46.with(|rc| *rc.borrow()),
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
pub fn set_memory_limits_48(size: Option<()>) {
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
pub fn set_file_limit_49(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_50(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_51(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_52(
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
pub fn test_tm_53(
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
    (*tmp_54.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_54.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_54.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_54: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
