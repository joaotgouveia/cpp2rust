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
pub struct testcase_riemann_zeta_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub s: (),
    #[offset(16)]
    pub f: (),
}
thread_local!(
    pub static data001_10: Value<Box<[testcase_riemann_zeta_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -10_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003366982045101957_f64,
                s: -9.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.005812951776731901_f64,
                s: -9.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007290873229055696_f64,
                s: -9.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007842091065448439_f64,
                s: -9.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007575757575757576_f64,
                s: -9_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.006647655567755189_f64,
                s: -8.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.00524000953508594_f64,
                s: -8.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0035434308017674907_f64,
                s: -8.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0017417330388368587_f64,
                s: -8.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0015440036789213937_f64,
                s: -7.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.00278521310864974_f64,
                s: -7.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0036537321227995885_f64,
                s: -7.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.004114793081705353_f64,
                s: -7.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.004166666666666667_f64,
                s: -7_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.003836997503273836_f64,
                s: -6.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0031780270571783_f64,
                s: -6.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0022611282027338573_f64,
                s: -6.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0011710237049390457_f64,
                s: -6.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0011576366649881868_f64,
                s: -5.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.002210678431856433_f64,
                s: -5.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0030755853460586913_f64,
                s: -5.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003680438047793476_f64,
                s: -5.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003968253968253968_f64,
                s: -5_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.003899689130199977_f64,
                s: -4.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0034551830834302698_f64,
                s: -4.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.0026366345018725037_f64,
                s: -4.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.001468720930505692_f64,
                s: -4.199999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0016960463875825209_f64,
                s: -3.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.0035198355903356634_f64,
                s: -3.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.005344150320651346_f64,
                s: -3.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.007011972077091057_f64,
                s: -3.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.008333333333333333_f64,
                s: -3_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.00908072948568525_f64,
                s: -2.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.008982462378839656_f64,
                s: -2.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.007713023987424338_f64,
                s: -2.3999999999999995_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0.004879212359303582_f64,
                s: -2.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 0_f64,
                s: -2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.007522934776596874_f64,
                s: -1.799999999999999_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.01844898667896372_f64,
                s: -1.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.033764987694047545_f64,
                s: -1.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.05478844124388051_f64,
                s: -1.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.08333333333333333_f64,
                s: -1_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.12198707766977138_f64,
                s: -0.7999999999999989_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.1745957119380135_f64,
                s: -0.5999999999999996_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.24716546083171545_f64,
                s: -0.3999999999999986_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.34966628059831456_f64,
                s: -0.1999999999999993_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.5_f64,
                s: 0_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -0.7339209248963422_f64,
                s: 0.20000000000000107_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -1.1347977838669825_f64,
                s: 0.40000000000000036_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -1.9526614482240094_f64,
                s: 0.6000000000000014_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: -4.437538415895569_f64,
                s: 0.8000000000000007_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_11: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_12: Value<Box<[testcase_riemann_zeta_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_riemann_zeta_double_ {
                f0: 5.591582441177752_f64,
                s: 1.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 3.105547277977581_f64,
                s: 1.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 2.2857656656801297_f64,
                s: 1.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.882229618102822_f64,
                s: 1.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.6449340668482264_f64,
                s: 2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.4905432565068935_f64,
                s: 2.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.3833428588407355_f64,
                s: 2.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.3054778090727805_f64,
                s: 2.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.2470314223172534_f64,
                s: 2.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.2020569031595942_f64,
                s: 3_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.166773370984467_f64,
                s: 3.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.1386637757280416_f64,
                s: 3.4000000000000004_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.1159890791233376_f64,
                s: 3.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0975105764590043_f64,
                s: 3.8000000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0823232337111381_f64,
                s: 4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0697514772338095_f64,
                s: 4.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0592817259798355_f64,
                s: 4.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0505173825665735_f64,
                s: 4.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.043148013335179_f64,
                s: 4.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.03692775514337_f64,
                s: 5_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0316598766779166_f64,
                s: 5.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.027185538920354_f64,
                s: 5.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.02337547922703_f64,
                s: 5.6000000000000005_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0201237683883446_f64,
                s: 5.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0173430619844492_f64,
                s: 6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.014960945185223_f64,
                s: 6.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.012917088712184_f64,
                s: 6.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.011161014154271_f64,
                s: 6.6000000000000005_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.009650322344712_f64,
                s: 6.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.008349277381923_f64,
                s: 7_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0072276664807172_f64,
                s: 7.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0062598756930512_f64,
                s: 7.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0054241359879634_f64,
                s: 7.6000000000000005_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0047019048164696_f64,
                s: 7.800000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0040773561979444_f64,
                s: 8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0035369583062013_f64,
                s: 8.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0030691220374448_f64,
                s: 8.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0026639074861505_f64,
                s: 8.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.002312777909822_f64,
                s: 8.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0020083928260821_f64,
                s: 9_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0017444334995897_f64,
                s: 9.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0015154553480514_f64,
                s: 9.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0013167628052648_f64,
                s: 9.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0011443029840295_f64,
                s: 9.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000994575127818_f64,
                s: 10_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0008645533615088_f64,
                s: 10.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000751620674465_f64,
                s: 10.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000653512414085_f64,
                s: 10.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0005682678503411_f64,
                s: 10.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0004941886041194_f64,
                s: 11_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0004298029239942_f64,
                s: 11.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0003738349551166_f64,
                s: 11.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0003251782761946_f64,
                s: 11.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000282873090999_f64,
                s: 11.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000246086553308_f64,
                s: 12_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000214095781875_f64,
                s: 12.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001862731874054_f64,
                s: 12.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000162073788746_f64,
                s: 12.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000141024242209_f64,
                s: 12.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001227133475785_f64,
                s: 13_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0001067838280169_f64,
                s: 13.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000929252097515_f64,
                s: 13.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000080867651872_f64,
                s: 13.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000703765974504_f64,
                s: 13.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000612481350588_f64,
                s: 14_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000533049750668_f64,
                s: 14.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000463929582293_f64,
                s: 14.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000403780253397_f64,
                s: 14.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000351435864272_f64,
                s: 14.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000030588236307_f64,
                s: 15_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000266237704787_f64,
                s: 15.200000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000231734615617_f64,
                s: 15.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000201705617975_f64,
                s: 15.600000000000001_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000017557001761_f64,
                s: 15.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000152822594086_f64,
                s: 16_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000133023770335_f64,
                s: 16.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000115791066833_f64,
                s: 16.4_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000100791671644_f64,
                s: 16.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000087735980012_f64,
                s: 16.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000076371976379_f64,
                s: 17_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000066480348633_f64,
                s: 17.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000057870238737_f64,
                s: 17.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000005037554661_f64,
                s: 17.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000043851715013_f64,
                s: 17.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000003817293265_f64,
                s: 18_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000033229700953_f64,
                s: 18.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000002892671715_f64,
                s: 18.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000002518103242_f64,
                s: 18.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000021920449285_f64,
                s: 18.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000019082127165_f64,
                s: 19_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000016611368951_f64,
                s: 19.2_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000014460565094_f64,
                s: 19.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000012588274738_f64,
                s: 19.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000010958426055_f64,
                s: 19.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000009539620338_f64,
                s: 20_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000008304526344_f64,
                s: 20.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000007229353187_f64,
                s: 20.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000006293391575_f64,
                s: 20.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000005478614529_f64,
                s: 20.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000004769329869_f64,
                s: 21_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000415187772_f64,
                s: 21.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000003614367252_f64,
                s: 21.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000003146447527_f64,
                s: 21.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000273910802_f64,
                s: 21.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000002384505027_f64,
                s: 22_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000002075810521_f64,
                s: 22.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001807080625_f64,
                s: 22.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001573141093_f64,
                s: 22.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001369487659_f64,
                s: 22.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000119219926_f64,
                s: 23_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000001037862518_f64,
                s: 23.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000903506006_f64,
                s: 23.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000078654301_f64,
                s: 23.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000684721728_f64,
                s: 23.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000059608189_f64,
                s: 24_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000051891702_f64,
                s: 24.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000451741575_f64,
                s: 24.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000393262332_f64,
                s: 24.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.00000003423535_f64,
                s: 24.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000298035034_f64,
                s: 25_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000259453767_f64,
                s: 25.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000225866978_f64,
                s: 25.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000019662811_f64,
                s: 25.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000171174297_f64,
                s: 25.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000149015549_f64,
                s: 26_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000129725302_f64,
                s: 26.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000112932221_f64,
                s: 26.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000098313035_f64,
                s: 26.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000085586331_f64,
                s: 26.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000074507118_f64,
                s: 27_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000064862125_f64,
                s: 27.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000056465688_f64,
                s: 27.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000004915618_f64,
                s: 27.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000042792894_f64,
                s: 27.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000003725334_f64,
                s: 28_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000032430887_f64,
                s: 28.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000028232703_f64,
                s: 28.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000024577975_f64,
                s: 28.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000021396356_f64,
                s: 28.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000018626598_f64,
                s: 29_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000016215385_f64,
                s: 29.200000000000003_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000014116304_f64,
                s: 29.400000000000002_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.000000001228895_f64,
                s: 29.6_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000010698147_f64,
                s: 29.8_f64,
                f: 0_f64,
            },
            testcase_riemann_zeta_double_ {
                f0: 1.0000000009313275_f64,
                s: 30_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_13: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
pub fn test_14(data: Ptr<testcase_riemann_zeta_double_>, mut toler: ()) {
    let mut test: () = true;
    let mut eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = ((1320usize as ()).wrapping_div((24usize as ())));
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
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"check_value_neg.cc" )  ) .into() , ( 296_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test(const testcase_riemann_zeta<Ret> (&)[Num], Ret) [Ret = double, Num = 55U]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"max_abs_frac < toler" )  ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
}
pub fn test_16(data: Ptr<testcase_riemann_zeta_double_>, mut toler: ()) {
    let mut test: () = true;
    let mut eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = ((1320usize as ()).wrapping_div((24usize as ())));
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
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"check_value_neg.cc" )  ) .into() , ( 296_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test(const testcase_riemann_zeta<Ret> (&)[Num], Ret) [Ret = double, Num = 145U]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"max_abs_frac < toler" )  ) .into() , ] ) )
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
        test_16(
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
        field!(this, pid_).write((({ getpid_17() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_18(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_19(
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
pub fn set_memory_limits_20(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_21((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_22((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_21((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_22((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_21((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_22((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_23(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_21((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_22((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_24(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_25(
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
pub fn run_tests_wrapped_locale_26(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                    __builtin_fprintf_15 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
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
pub fn run_tests_wrapped_env_27(
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
pub fn test_tm_29(
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
    (*tmp_30.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_30.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_30.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_30: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_17() })) {
            ({
                semctl_19(
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
            semop_28(
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
            semop_28(
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
