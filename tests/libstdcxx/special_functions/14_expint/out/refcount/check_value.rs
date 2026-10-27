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
#[byte_size(24)]
pub struct testcase_expint_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub x: (),
    #[offset(16)]
    pub f: (),
}
thread_local!(
    pub static data001_10: Value<Box<[testcase_expint_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_expint_double_ {
                f0: -3.783264029550459e-24_f64,
                x: -50_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.0489811642368024e-23_f64,
                x: -49_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -2.9096641904058423e-23_f64,
                x: -48_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -8.074197842725813e-23_f64,
                x: -47_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -2.2415317597443e-22_f64,
                x: -46_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -6.225690809462385e-22_f64,
                x: -45_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.7299598742816476e-21_f64,
                x: -44_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -4.809496556950018e-21_f64,
                x: -43_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.3377908810011775e-20_f64,
                x: -42_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -3.723166776459978e-20_f64,
                x: -41_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.036773261451657e-19_f64,
                x: -40_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -2.8887793015227007e-19_f64,
                x: -39_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -8.05410691429075e-19_f64,
                x: -38_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -2.247020697588571e-18_f64,
                x: -37_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -6.273339009762242e-18_f64,
                x: -36_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.752705938994737e-17_f64,
                x: -35_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -4.9006761183927874e-17_f64,
                x: -34_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.3713843484487468e-16_f64,
                x: -33_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -3.840961801225067e-16_f64,
                x: -32_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.076767038616238e-15_f64,
                x: -31_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -3.0215520106888124e-15_f64,
                x: -30_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -8.487759778353563e-15_f64,
                x: -29_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -2.386941511933733e-14_f64,
                x: -28_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -6.720637435262039e-14_f64,
                x: -27_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.8946858856749785e-13_f64,
                x: -26_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -5.348899755340217e-13_f64,
                x: -25_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.5123058939997059e-12_f64,
                x: -24_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -4.282684795665672e-12_f64,
                x: -23_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.2149378956204371e-11_f64,
                x: -22_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -3.453201267146756e-11_f64,
                x: -21_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -9.835525290649882e-11_f64,
                x: -20_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -2.8078290970607954e-10_f64,
                x: -19_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -8.036090344828677e-10_f64,
                x: -18_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -2.3064319898216543e-09_f64,
                x: -17_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -6.640487249441043e-09_f64,
                x: -16_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.918627892147867e-08_f64,
                x: -15_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -5.5656311111451816e-08_f64,
                x: -14_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.6218662188014328e-07_f64,
                x: -13_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -4.751081824672493e-07_f64,
                x: -12_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.4003003042474418e-06_f64,
                x: -11_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -4.156968929685325e-06_f64,
                x: -10_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -1.2447354178006272e-05_f64,
                x: -9_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -3.7665622843924906e-05_f64,
                x: -8_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -0.0001154817316103382_f64,
                x: -7_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -0.0003600824521626586_f64,
                x: -6_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -0.0011482955912753257_f64,
                x: -5_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -0.0037793524098489063_f64,
                x: -4_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -0.013048381094197037_f64,
                x: -3_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -0.048900510708061125_f64,
                x: -2_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: -0.21938393439552029_f64,
                x: -1_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_11: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_12: Value<Box<[testcase_expint_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_expint_double_ {
                f0: 1.8951178163559366_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 4.954234356001891_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 9.933832570625416_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 19.630874470056217_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 40.18527535580318_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 85.9897621424392_f64,
                x: 6_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 191.50474333550136_f64,
                x: 7_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 440.37989953483833_f64,
                x: 8_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 1037.8782907170896_f64,
                x: 9_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 2492.228976241878_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 6071.406374098611_f64,
                x: 11_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 14959.532666397528_f64,
                x: 12_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 37197.68849068904_f64,
                x: 13_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 93192.51363396537_f64,
                x: 14_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 234955.8524907683_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 595560.9986708369_f64,
                x: 16_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 1516637.894042517_f64,
                x: 17_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 3877904.3305974435_f64,
                x: 18_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 9950907.251046844_f64,
                x: 19_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 25615652.664056588_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 66127186.35548493_f64,
                x: 21_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 171144671.3003637_f64,
                x: 22_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 443966369.8302712_f64,
                x: 23_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 1154115391.8491828_f64,
                x: 24_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 3005950906.5255494_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 7842940991.898188_f64,
                x: 26_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 20496497119.88081_f64,
                x: 27_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 53645118592.31468_f64,
                x: 28_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 140599195758.4069_f64,
                x: 29_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 368973209407.2742_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 969455575968.3939_f64,
                x: 31_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 2550043566357.787_f64,
                x: 32_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 6714640184076.497_f64,
                x: 33_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 17698037244116.266_f64,
                x: 34_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 46690550144661.6_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 123285207991209.75_f64,
                x: 36_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 325798899867226.5_f64,
                x: 37_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 861638819996578.8_f64,
                x: 38_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 2280446200301902.5_f64,
                x: 39_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 6039718263611242_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 16006649143245042_f64,
                x: 41_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 42447960921368512_f64,
                x: 42_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 112634829016696656_f64,
                x: 43_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 299044471863233664_f64,
                x: 44_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 794391603570445312_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 2111342388647823872_f64,
                x: 46_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 5614329680810342400_f64,
                x: 47_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 14936302131129929728_f64,
                x: 48_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 39754427479037444096_f64,
                x: 49_f64,
                f: 0_f64,
            },
            testcase_expint_double_ {
                f0: 105856368971316903936_f64,
                x: 50_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_13: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
pub fn test_14(data: Ptr<testcase_expint_double_>, mut toler: ()) {
    let mut test: () = true;
    let mut eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 50_();
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
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"check_value.cc" )  ) .into() , ( 191_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test(const testcase_expint<Ret> (&)[Num], Ret) [Ret = double, Num = 50U]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"max_abs_frac < toler" )  ) .into() , ] ) )
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
        test_14(
            data001_10.with(|v| v.as_pointer()),
            toler001_11.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_14(
            data002_12.with(|v| v.as_pointer()),
            toler002_13.with(|rc| *rc.borrow()),
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
        field!(this, pid_).write((({ getpid_16() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_17(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_18(
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
pub fn set_memory_limits_19(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_20((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_21((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_20((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_21((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_20((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_21((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_22(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_20((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_21((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_23(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_24(
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
    let mut w: () = unimplemented!();
    if unimplemented!() {
        ({ __throw_runtime_error_3((s).clone()) });
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_25(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                ({
                    __builtin_fprintf_15 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        ({ __throw_runtime_error_3(unimplemented!()) });
    }
}
pub fn run_tests_wrapped_env_26(
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
pub fn test_tm_28(
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
    (*tmp_29.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_29.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_29.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_29: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_16() })) {
            ({
                semctl_18(
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
            semop_27(
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
            semop_27(
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__4.with(|_| ());
    let _ = throw_on__5.with(|_| ());
    let _ = _M_count_6.with(|_| ());
    let _ = next_id__7.with(|_| ());
}
