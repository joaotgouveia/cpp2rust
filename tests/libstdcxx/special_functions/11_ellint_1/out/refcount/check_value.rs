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
pub struct testcase_ellint_1_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub k: (),
    #[offset(16)]
    pub phi: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data001_10: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17525427376115027_f64,
                k: -0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.35492464591297446_f64,
                k: -0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5438822141615712_f64,
                k: -0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7479740042353251_f64,
                k: -0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9746389845196645_f64,
                k: -0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2334463254523438_f64,
                k: -0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.5355247765594913_f64,
                k: -0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.8882928567775126_f64,
                k: -0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 2.2805491384227703_f64,
                k: -0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_11: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_12: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.175101542413389_f64,
                k: -0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.35365068839779396_f64,
                k: -0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5392680440908455_f64,
                k: -0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7358792602807037_f64,
                k: -0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9477094297007117_f64,
                k: -0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.1789022995388236_f64,
                k: -0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4323027881876012_f64,
                k: -0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.7069629739121677_f64,
                k: -0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.9953027776647294_f64,
                k: -0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_13: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_14: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17496737466916723_f64,
                k: -0.7_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3525468753567793_f64,
                k: -0.7_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5353674027599713_f64,
                k: -0.7_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7260379765168445_f64,
                k: -0.7_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9269829634831345_f64,
                k: -0.7_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.1400447527693316_f64,
                k: -0.7_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.365766811719407_f64,
                k: -0.7_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.6024686895959162_f64,
                k: -0.7_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.8456939983747234_f64,
                k: -0.7_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_15: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_16: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.1748515436298836_f64,
                k: -0.6_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3516050986554432_f64,
                k: -0.6_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5321065257844614_f64,
                k: -0.6_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7180530466448566_f64,
                k: -0.6_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9108275903019598_f64,
                k: -0.6_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.1112333229323361_f64,
                k: -0.6_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.319146119036527_f64,
                k: -0.6_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.533202210508478_f64,
                k: -0.6_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.7507538029157523_f64,
                k: -0.6_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_17: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_18: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17475385514035785_f64,
                k: -0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3508186847010158_f64,
                k: -0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5294286270519057_f64,
                k: -0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7116472756263033_f64,
                k: -0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8982452359422777_f64,
                k: -0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0895506700518853_f64,
                k: -0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2853005857432933_f64,
                k: -0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4845545520549488_f64,
                k: -0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.685750354812596_f64,
                k: -0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_19: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_20: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17467414669441528_f64,
                k: -0.3999999999999999_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3501822277248344_f64,
                k: -0.3999999999999999_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5272901591750875_f64,
                k: -0.3999999999999999_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7066237440734124_f64,
                k: -0.3999999999999999_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8885921049760216_f64,
                k: -0.3999999999999999_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0733136290471381_f64,
                k: -0.3999999999999999_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2605612170157066_f64,
                k: -0.3999999999999999_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4497513956433439_f64,
                k: -0.3999999999999999_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.6399998658645112_f64,
                k: -0.3999999999999999_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_21: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_22: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.174612286530001_f64,
                k: -0.29999999999999993_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3496914610279842_f64,
                k: -0.29999999999999993_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5256582287372631_f64,
                k: -0.29999999999999993_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7028422651240854_f64,
                k: -0.29999999999999993_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8814413919511117_f64,
                k: -0.29999999999999993_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.061489706726052_f64,
                k: -0.29999999999999993_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.242841682417422_f64,
                k: -0.29999999999999993_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.425179587701593_f64,
                k: -0.29999999999999993_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.6080486199305126_f64,
                k: -0.29999999999999993_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_23: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_24: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17456817290292806_f64,
                k: -0.19999999999999996_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.34934315932086796_f64,
                k: -0.19999999999999996_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5245088052944399_f64,
                k: -0.19999999999999996_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7002049100984489_f64,
                k: -0.19999999999999996_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8765100664996798_f64,
                k: -0.19999999999999996_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0534305870298994_f64,
                k: -0.19999999999999996_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2308975521670789_f64,
                k: -0.19999999999999996_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4087733584990738_f64,
                k: -0.19999999999999996_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.5868678474541662_f64,
                k: -0.19999999999999996_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_25: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data009_26: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17454173353063662_f64,
                k: -0.09999999999999998_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.34913506721468096_f64,
                k: -0.09999999999999998_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5238255001653895_f64,
                k: -0.09999999999999998_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.6986470085417702_f64,
                k: -0.09999999999999998_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8736179258696486_f64,
                k: -0.09999999999999998_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0487386319621685_f64,
                k: -0.09999999999999998_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.223991375207876_f64,
                k: -0.09999999999999998_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.3993423113684051_f64,
                k: -0.09999999999999998_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.5747455615173558_f64,
                k: -0.09999999999999998_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_27: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data010_28: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17453292519943295_f64,
                k: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3490658503988659_f64,
                k: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5235987755982988_f64,
                k: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.6981317007977318_f64,
                k: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8726646259971648_f64,
                k: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0471975511965976_f64,
                k: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2217304763960306_f64,
                k: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.3962634015954636_f64,
                k: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.5707963267948966_f64,
                k: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_29: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data011_30: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17454173353063662_f64,
                k: 0.10000000000000009_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.34913506721468096_f64,
                k: 0.10000000000000009_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5238255001653895_f64,
                k: 0.10000000000000009_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.6986470085417702_f64,
                k: 0.10000000000000009_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8736179258696486_f64,
                k: 0.10000000000000009_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0487386319621685_f64,
                k: 0.10000000000000009_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.223991375207876_f64,
                k: 0.10000000000000009_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.3993423113684051_f64,
                k: 0.10000000000000009_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.574745561517356_f64,
                k: 0.10000000000000009_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_31: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data012_32: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17456817290292806_f64,
                k: 0.20000000000000018_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.34934315932086796_f64,
                k: 0.20000000000000018_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5245088052944399_f64,
                k: 0.20000000000000018_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7002049100984489_f64,
                k: 0.20000000000000018_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8765100664996798_f64,
                k: 0.20000000000000018_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0534305870298994_f64,
                k: 0.20000000000000018_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2308975521670789_f64,
                k: 0.20000000000000018_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4087733584990738_f64,
                k: 0.20000000000000018_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.5868678474541662_f64,
                k: 0.20000000000000018_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_33: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data013_34: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.174612286530001_f64,
                k: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3496914610279842_f64,
                k: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5256582287372631_f64,
                k: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7028422651240854_f64,
                k: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8814413919511117_f64,
                k: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.061489706726052_f64,
                k: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.242841682417422_f64,
                k: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.425179587701593_f64,
                k: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.6080486199305128_f64,
                k: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_35: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data014_36: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17467414669441528_f64,
                k: 0.40000000000000013_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3501822277248344_f64,
                k: 0.40000000000000013_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5272901591750875_f64,
                k: 0.40000000000000013_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7066237440734124_f64,
                k: 0.40000000000000013_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8885921049760216_f64,
                k: 0.40000000000000013_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0733136290471381_f64,
                k: 0.40000000000000013_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2605612170157066_f64,
                k: 0.40000000000000013_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4497513956433439_f64,
                k: 0.40000000000000013_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.6399998658645112_f64,
                k: 0.40000000000000013_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_37: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data015_38: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17475385514035785_f64,
                k: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3508186847010158_f64,
                k: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5294286270519057_f64,
                k: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7116472756263033_f64,
                k: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.8982452359422777_f64,
                k: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.0895506700518853_f64,
                k: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.2853005857432933_f64,
                k: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4845545520549488_f64,
                k: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.685750354812596_f64,
                k: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler015_39: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data016_40: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.1748515436298836_f64,
                k: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3516050986554432_f64,
                k: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5321065257844614_f64,
                k: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7180530466448566_f64,
                k: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9108275903019598_f64,
                k: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.1112333229323361_f64,
                k: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.319146119036527_f64,
                k: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.533202210508478_f64,
                k: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.7507538029157526_f64,
                k: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler016_41: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data017_42: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17496737466916723_f64,
                k: 0.7000000000000002_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.3525468753567793_f64,
                k: 0.7000000000000002_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5353674027599713_f64,
                k: 0.7000000000000002_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7260379765168445_f64,
                k: 0.7000000000000002_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9269829634831345_f64,
                k: 0.7000000000000002_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.1400447527693318_f64,
                k: 0.7000000000000002_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.3657668117194073_f64,
                k: 0.7000000000000002_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.6024686895959164_f64,
                k: 0.7000000000000002_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.8456939983747236_f64,
                k: 0.7000000000000002_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler017_43: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data018_44: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.175101542413389_f64,
                k: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.35365068839779396_f64,
                k: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5392680440908455_f64,
                k: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7358792602807037_f64,
                k: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9477094297007117_f64,
                k: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.1789022995388236_f64,
                k: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.4323027881876012_f64,
                k: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.7069629739121677_f64,
                k: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.9953027776647294_f64,
                k: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler018_45: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data019_46: Value<Box<[testcase_ellint_1_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_1_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.17525427376115027_f64,
                k: 0.9000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.35492464591297446_f64,
                k: 0.9000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.5438822141615712_f64,
                k: 0.9000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.7479740042353251_f64,
                k: 0.9000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 0.9746389845196645_f64,
                k: 0.9000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.233446325452344_f64,
                k: 0.9000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.5355247765594915_f64,
                k: 0.9000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 1.8882928567775128_f64,
                k: 0.9000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_1_double_ {
                f0: 2.2805491384227707_f64,
                k: 0.9000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler019_47: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
pub fn test_48(data: Ptr<testcase_ellint_1_double_>, mut toler: ()) {
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
            ({
                __builtin_fprintf_49 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"check_value.cc" )  ) .into() , ( 482_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test(const testcase_ellint_1<Ret> (&)[Num], Ret) [Ret = double, Num = 10U]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"max_abs_frac < toler" )  ) .into() , ] ) )
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
        test_48(
            data001_10.with(|v| v.as_pointer()),
            toler001_11.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data002_12.with(|v| v.as_pointer()),
            toler002_13.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data003_14.with(|v| v.as_pointer()),
            toler003_15.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data004_16.with(|v| v.as_pointer()),
            toler004_17.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data005_18.with(|v| v.as_pointer()),
            toler005_19.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data006_20.with(|v| v.as_pointer()),
            toler006_21.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data007_22.with(|v| v.as_pointer()),
            toler007_23.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data008_24.with(|v| v.as_pointer()),
            toler008_25.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data009_26.with(|v| v.as_pointer()),
            toler009_27.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data010_28.with(|v| v.as_pointer()),
            toler010_29.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data011_30.with(|v| v.as_pointer()),
            toler011_31.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data012_32.with(|v| v.as_pointer()),
            toler012_33.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data013_34.with(|v| v.as_pointer()),
            toler013_35.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data014_36.with(|v| v.as_pointer()),
            toler014_37.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data015_38.with(|v| v.as_pointer()),
            toler015_39.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data016_40.with(|v| v.as_pointer()),
            toler016_41.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data017_42.with(|v| v.as_pointer()),
            toler017_43.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data018_44.with(|v| v.as_pointer()),
            toler018_45.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_48(
            data019_46.with(|v| v.as_pointer()),
            toler019_47.with(|rc| *rc.borrow()),
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
        field!(this, pid_).write((({ getpid_50() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_51(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_52(
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
pub fn set_memory_limits_53(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_54((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_55((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_54((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_55((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_54((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_55((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_56(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_54((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_55((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_57(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_58(
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
pub fn run_tests_wrapped_locale_59(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                    __builtin_fprintf_49 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
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
pub fn run_tests_wrapped_env_60(
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
pub fn test_tm_62(
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
    (*tmp_63.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_63.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_63.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_63: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_50() })) {
            ({
                semctl_52(
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
            semop_61(
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
            semop_61(
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
    let _ = data009_26.with(|_| ());
    let _ = toler009_27.with(|_| ());
    let _ = data010_28.with(|_| ());
    let _ = toler010_29.with(|_| ());
    let _ = data011_30.with(|_| ());
    let _ = toler011_31.with(|_| ());
    let _ = data012_32.with(|_| ());
    let _ = toler012_33.with(|_| ());
    let _ = data013_34.with(|_| ());
    let _ = toler013_35.with(|_| ());
    let _ = data014_36.with(|_| ());
    let _ = toler014_37.with(|_| ());
    let _ = data015_38.with(|_| ());
    let _ = toler015_39.with(|_| ());
    let _ = data016_40.with(|_| ());
    let _ = toler016_41.with(|_| ());
    let _ = data017_42.with(|_| ());
    let _ = toler017_43.with(|_| ());
    let _ = data018_44.with(|_| ());
    let _ = toler018_45.with(|_| ());
    let _ = data019_46.with(|_| ());
    let _ = toler019_47.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__4.with(|_| ());
    let _ = throw_on__5.with(|_| ());
    let _ = _M_count_6.with(|_| ());
    let _ = next_id__7.with(|_| ());
}
