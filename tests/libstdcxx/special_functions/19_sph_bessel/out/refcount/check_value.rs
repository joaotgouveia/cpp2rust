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
pub struct testcase_sph_bessel_double_ {
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
    pub static data001_10: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.9896158370180917_f64,
                n: 0_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.958851077208406_f64,
                n: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.9088516800311123_f64,
                n: 0_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.8414709848078965_f64,
                n: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.759187695484469_f64,
                n: 0_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.6649966577360363_f64,
                n: 0_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.562277683927964_f64,
                n: 0_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.4546487134128408_f64,
                n: 0_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.345810309727965_f64,
                n: 0_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.23938885764158258_f64,
                n: 0_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.13878581529175696_f64,
                n: 0_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0470400026866224_f64,
                n: 0_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03329081062464873_f64,
                n: 0_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.10022377933989138_f64,
                n: 0_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.15241635166462503_f64,
                n: 0_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.18920062382698205_f64,
                n: 0_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.21058573134790204_f64,
                n: 0_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.21722891503668823_f64,
                n: 0_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.2103774292579743_f64,
                n: 0_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.1917848549326277_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_11: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_12: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.08281366122978806_f64,
                n: 1_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1625370306360665_f64,
                n: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.236217081543055_f64,
                n: 1_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.30116867893975674_f64,
                n: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3550922664713602_f64,
                n: 1_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3961729707122223_f64,
                n: 1_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.42315642261568914_f64,
                n: 1_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.43539777497999166_f64,
                n: 1_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.4328817477558685_f64,
                n: 1_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.41621298927540656_f64,
                n: 1_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3865775250633529_f64,
                n: 1_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.34567749976235596_f64,
                n: 1_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.29564272783258383_f64,
                n: 1_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.23892368798597285_f64,
                n: 1_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1781714681799829_f64,
                n: 1_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.11611074925915747_f64,
                n: 1_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.05541217848609196_f64,
                n: 1_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0014295812457574522_f64,
                n: 1_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.05220622782020018_f64,
                n: 1_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0950894080791708_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_13: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data003_14: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004148097739361125_f64,
                n: 2_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.016371106607993412_f64,
                n: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.036016646141108236_f64,
                n: 2_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06203505201137386_f64,
                n: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.09303374404679562_f64,
                n: 2_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.12734928368840817_f64,
                n: 2_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1631333262703603_f64,
                n: 2_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1984479490571466_f64,
                n: 2_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.23136535394652627_f64,
                n: 2_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.26006672948890525_f64,
                n: 2_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.2829351211409916_f64,
                n: 2_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.29863749707573356_f64,
                n: 2_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.30619179016241843_f64,
                n: 2_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.3050155118992967_f64,
                n: 2_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.2949535262086113_f64,
                n: 2_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.27628368577135015_f64,
                n: 2_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.24970021027926106_f64,
                n: 2_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.21627586087284995_f64,
                n: 2_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.17740507484521628_f64,
                n: 2_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1347312100851252_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_15: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_16: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.371981123726823e-08_f64,
                n: 5_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.9774668754574457e-06_f64,
                n: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.2339447678335765e-05_f64,
                n: 5_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.256115861125816e-05_f64,
                n: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00027638888920123806_f64,
                n: 5_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0006696205962893247_f64,
                n: 5_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00140217290225728_f64,
                n: 5_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0026351697702441173_f64,
                n: 5_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004554003475056755_f64,
                n: 5_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.007357638737768938_f64,
                n: 5_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011244740276407147_f64,
                n: 5_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01639748095599911_f64,
                n: 5_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.02296411247484551_f64,
                n: 5_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.03104153653739119_f64,
                n: 5_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.04065918944094895_f64,
                n: 5_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.05176553975736347_f64,
                n: 5_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06421839577342563_f64,
                n: 5_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.07778003083289287_f64,
                n: 5_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.09211787059372922_f64,
                n: 5_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.10681116145650453_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_17: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_18: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.926742745370847e-17_f64,
                n: 10_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.064123963661874e-14_f64,
                n: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.045930747410929e-12_f64,
                n: 10_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.11655264004731e-11_f64,
                n: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.547073953019994e-10_f64,
                n: 10_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.99344069948363e-09_f64,
                n: 10_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.8327719460735247e-08_f64,
                n: 10_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.825300864974722e-08_f64,
                n: 10_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.1653870546846626e-07_f64,
                n: 10_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.050436229638538e-07_f64,
                n: 10_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.524648535215844e-06_f64,
                n: 10_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.5260038931752543e-06_f64,
                n: 10_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.583904002053146e-06_f64,
                n: 10_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.5327786999397103e-05_f64,
                n: 10_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.9348811002317664e-05_f64,
                n: 10_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.358986576863261e-05_f64,
                n: 10_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.381860241047799e-05_f64,
                n: 10_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.000158175163714558_f64,
                n: 10_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00025777607369970674_f64,
                n: 10_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0004073442442494605_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_19: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data006_20: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.930748707339934e-38_f64,
                n: 20_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.251588081015394e-32_f64,
                n: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.4025911398834722e-28_f64,
                n: 20_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.53779572223687e-26_f64,
                n: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.495343924359341e-24_f64,
                n: 20_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.470312039088405e-22_f64,
                n: 20_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.34046271382972e-21_f64,
                n: 20_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.632641100887607e-20_f64,
                n: 20_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.9496335952781085e-19_f64,
                n: 20_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.448853275957898e-18_f64,
                n: 20_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.2725223040880135e-17_f64,
                n: 20_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.3942249272752627e-16_f64,
                n: 20_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.165403374149986e-15_f64,
                n: 20_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.0303402625237494e-15_f64,
                n: 20_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.9572475798118565e-14_f64,
                n: 20_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.95598806449061e-14_f64,
                n: 20_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.2825949745670935e-13_f64,
                n: 20_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 6.978182302179282e-13_f64,
                n: 20_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.002415738866502e-12_f64,
                n: 20_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.42772676079321e-12_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_21: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data007_22: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.1917848549326277_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.054402111088936986_f64,
                n: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.04335252267714113_f64,
                n: 0_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.045647262536381385_f64,
                n: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005294070003910922_f64,
                n: 0_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03293438746976206_f64,
                n: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.012233790557032886_f64,
                n: 0_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01862782901198372_f64,
                n: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.018908967211869295_f64,
                n: 0_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005247497074078575_f64,
                n: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.018177366788338544_f64,
                n: 0_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005080177018370278_f64,
                n: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.012720441222924667_f64,
                n: 0_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011055581165112701_f64,
                n: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005170421805459072_f64,
                n: 0_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01242360817404219_f64,
                n: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0020714778817480834_f64,
                n: 0_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.009933296262228422_f64,
                n: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.007192228576169695_f64,
                n: 0_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005063656411097588_f64,
                n: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_23: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
thread_local!(
    pub static data008_24: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0950894080791708_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.07846694179875155_f64,
                n: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.05353602903573083_f64,
                n: 1_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01812173996385053_f64,
                n: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03985987527469538_f64,
                n: 1_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0062395279119115375_f64,
                n: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.02547024041527068_f64,
                n: 1_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01713914726660614_f64,
                n: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.011253622702352454_f64,
                n: 1_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01940427051132384_f64,
                n: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0007328022372780778_f64,
                n: 1_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0157888800566131_f64,
                n: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.008848835268632258_f64,
                n: 1_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.008889480313159816_f64,
                n: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.012358955887069445_f64,
                n: 1_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0012245454458125673_f64,
                n: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01155653135896816_f64,
                n: 1_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0050889656932377614_f64,
                n: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.007610329814933157_f64,
                n: 1_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.008673825286987815_f64,
                n: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_25: Value<()> = Rc::new(RefCell::new(5.000000000000003e-11_f64));
);
thread_local!(
    pub static data009_26: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.1347312100851252_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.07794219362856245_f64,
                n: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.032645316869994966_f64,
                n: 2_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.048365523530958965_f64,
                n: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0005108849709474761_f64,
                n: 2_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.03231043467857091_f64,
                n: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.014416954021198941_f64,
                n: 2_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.017342392966988262_f64,
                n: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01965920872535946_f64,
                n: 2_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004083240843399146_f64,
                n: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.018137395757214285_f64,
                n: 2_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.005869621021200933_f64,
                n: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01231203344129549_f64,
                n: 2_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01143655889281955_f64,
                n: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004676063569976294_f64,
                n: 2_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.012469528628260161_f64,
                n: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0024793554591234306_f64,
                n: 2_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.009763664072453828_f64,
                n: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.007432554780851794_f64,
                n: 2_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.004803441652487954_f64,
                n: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_27: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
thread_local!(
    pub static data010_28: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.10681116145650453_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.055534511621452155_f64,
                n: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06596800707652195_f64,
                n: 5_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.016683908063095682_f64,
                n: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03611779598972238_f64,
                n: 5_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.02050400873682749_f64,
                n: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01849948120681456_f64,
                n: 5_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.022448773791044995_f64,
                n: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.004855269484502013_f64,
                n: 5_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.020048300563664877_f64,
                n: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005299992445556574_f64,
                n: 5_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.014151556281331407_f64,
                n: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011354588594416778_f64,
                n: 5_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.006498378178532357_f64,
                n: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.013089909320064257_f64,
                n: 5_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0009620045007130245_f64,
                n: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011048668899130202_f64,
                n: 5_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0065639581708136045_f64,
                n: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.006464611936820277_f64,
                n: 5_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.009290148934907571_f64,
                n: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_29: Value<()> = Rc::new(RefCell::new(5.000000000000003e-11_f64));
);
thread_local!(
    pub static data011_30: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0004073442442494605_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.06460515449256427_f64,
                n: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0018969790010883577_f64,
                n: 10_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.039686698644626366_f64,
                n: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.03625328560112858_f64,
                n: 10_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.014529646403897799_f64,
                n: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.026281264603993857_f64,
                n: 10_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.013124803182748323_f64,
                n: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01760083138372898_f64,
                n: 10_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.015039221463465955_f64,
                n: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.009525628934916739_f64,
                n: 10_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01582271939400834_f64,
                n: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.0019391391708249754_f64,
                n: 10_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.014293389028395012_f64,
                n: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.004421028503169623_f64,
                n: 10_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.010516146958338813_f64,
                n: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.008673627513132573_f64,
                n: 10_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.005290506635723933_f64,
                n: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010258326955210768_f64,
                n: 10_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.00019565785971342414_f64,
                n: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_31: Value<()> = Rc::new(RefCell::new(2.5000000000000017e-10_f64));
);
thread_local!(
    pub static data012_32: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.42772676079321e-12_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.308371961319467e-06_f64,
                n: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0015467058510412498_f64,
                n: 20_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.03832485163980516_f64,
                n: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.028500071484154645_f64,
                n: 20_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.014711593353429081_f64,
                n: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01079765307026423_f64,
                n: 20_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.026535391837540293_f64,
                n: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.011582959134716391_f64,
                n: 20_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.01578502989826929_f64,
                n: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01388551918586274_f64,
                n: 20_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.011112458964023273_f64,
                n: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.011938384963927568_f64,
                n: 20_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010117695207156904_f64,
                n: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.008987121410238323_f64,
                n: 20_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.010400578884991936_f64,
                n: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.00553590206563267_f64,
                n: 20_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010639343320787522_f64,
                n: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.001805166145597953_f64,
                n: 20_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.010107671283873054_f64,
                n: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_33: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
thread_local!(
    pub static data013_34: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.857479350440152e-46_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.230696023218648e-31_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.68047166400808e-23_f64,
                n: 50_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.650080791872522e-17_f64,
                n: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.2540416973758975e-12_f64,
                n: 50_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.6901637185735326e-09_f64,
                n: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.0167148174422245e-06_f64,
                n: 50_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.394917403817907e-05_f64,
                n: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0024888927213794557_f64,
                n: 50_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.018829107369282647_f64,
                n: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.02637319843814549_f64,
                n: 50_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.021230978268739_f64,
                n: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.016539881802291313_f64,
                n: 50_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.015985416061436664_f64,
                n: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01546254898440559_f64,
                n: 50_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010638570118081819_f64,
                n: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00046961239784540793_f64,
                n: 50_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.009606588218992025_f64,
                n: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: -0.010613873910261154_f64,
                n: 50_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0005797140882277493_f64,
                n: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_35: Value<()> = Rc::new(RefCell::new(2.5000000000000017e-10_f64));
);
thread_local!(
    pub static data014_36: Value<Box<[testcase_sph_bessel_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_bessel_double_ {
                f0: 0_f64,
                n: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.535650303388993e-120_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.832040182005877e-90_f64,
                n: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.7406387750766626e-72_f64,
                n: 100_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.515271112531701e-60_f64,
                n: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.845545935381596e-51_f64,
                n: 100_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.088859674430158e-43_f64,
                n: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 8.897585491113394e-37_f64,
                n: 100_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.151349254773383e-31_f64,
                n: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 9.36735869945391e-27_f64,
                n: 100_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.0190122629310471e-22_f64,
                n: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 3.488780497769039e-19_f64,
                n: 100_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.444288342555559e-16_f64,
                n: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 2.3832619568710723e-13_f64,
                n: 100_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 5.894838417560799e-11_f64,
                n: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 7.188444635702228e-09_f64,
                n: 100_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 4.5247964400095e-07_f64,
                n: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 1.5096093228779032e-05_f64,
                n: 100_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.00026825172647807513_f64,
                n: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.0024744308520581117_f64,
                n: 100_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_bessel_double_ {
                f0: 0.01088047701143835_f64,
                n: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_37: Value<()> = Rc::new(RefCell::new(5.000000000000003e-12_f64));
);
pub fn test_38(data: Ptr<testcase_sph_bessel_double_>, mut toler: ()) {
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
                __builtin_fprintf_39 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"check_value.cc" )  ) .into() , ( 527_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test(const testcase_sph_bessel<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"max_abs_frac < toler" )  ) .into() , ] ) )
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
        test_38(
            data001_10.with(|v| v.as_pointer()),
            toler001_11.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data002_12.with(|v| v.as_pointer()),
            toler002_13.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data003_14.with(|v| v.as_pointer()),
            toler003_15.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data004_16.with(|v| v.as_pointer()),
            toler004_17.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data005_18.with(|v| v.as_pointer()),
            toler005_19.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data006_20.with(|v| v.as_pointer()),
            toler006_21.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data007_22.with(|v| v.as_pointer()),
            toler007_23.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data008_24.with(|v| v.as_pointer()),
            toler008_25.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data009_26.with(|v| v.as_pointer()),
            toler009_27.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data010_28.with(|v| v.as_pointer()),
            toler010_29.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data011_30.with(|v| v.as_pointer()),
            toler011_31.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data012_32.with(|v| v.as_pointer()),
            toler012_33.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data013_34.with(|v| v.as_pointer()),
            toler013_35.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_38(
            data014_36.with(|v| v.as_pointer()),
            toler014_37.with(|rc| *rc.borrow()),
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
        field!(this, pid_).write((({ getpid_40() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_41(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_42(
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
pub fn set_memory_limits_43(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_44((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_45((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_44((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_45((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_44((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_45((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_46(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_44((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_45((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_47(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_48(
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
pub fn run_tests_wrapped_locale_49(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                    __builtin_fprintf_39 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
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
pub fn run_tests_wrapped_env_50(
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
pub fn test_tm_52(
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
    (*tmp_53.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_53.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_53.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_53: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_40() })) {
            ({
                semctl_42(
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
            semop_51(
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
            semop_51(
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__4.with(|_| ());
    let _ = throw_on__5.with(|_| ());
    let _ = _M_count_6.with(|_| ());
    let _ = next_id__7.with(|_| ());
}
