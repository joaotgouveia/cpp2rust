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
pub struct testcase_cyl_bessel_i_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub nu: (),
    #[offset(16)]
    pub x: (),
    #[offset(24)]
    pub f: (),
}
thread_local!(
    pub static data007_10: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 1_f64,
                nu: 0_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0156861412236078_f64,
                nu: 0_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0634833707413236_f64,
                nu: 0_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.1456467780440014_f64,
                nu: 0_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2660658777520082_f64,
                nu: 0_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.4304687177218294_f64,
                nu: 0_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.6467231897728904_f64,
                nu: 0_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.9252521538585021_f64,
                nu: 0_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.279585302336067_f64,
                nu: 0_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.727078307190795_f64,
                nu: 0_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.289839144050123_f64,
                nu: 0_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.995913107237655_f64,
                nu: 0_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.8807925858650245_f64,
                nu: 0_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.989335997939514_f64,
                nu: 0_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.378203432225475_f64,
                nu: 0_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 9.118945860844565_f64,
                nu: 0_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 11.301921952136325_f64,
                nu: 0_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 14.041263683000595_f64,
                nu: 0_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 17.481171855609272_f64,
                nu: 0_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 21.80389874090212_f64,
                nu: 0_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 27.23987182360444_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_11: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data008_12: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 0.3333333333333333_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.5665068655780866_f64,
                nu: 0.3333333333333333_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.7389731564251186_f64,
                nu: 0.3333333333333333_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.8953232036583681_f64,
                nu: 0.3333333333333333_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0646313978895283_f64,
                nu: 0.3333333333333333_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.262377673260525_f64,
                nu: 0.3333333333333333_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.5014290000224382_f64,
                nu: 0.3333333333333333_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.795119552594604_f64,
                nu: 0.3333333333333333_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.1587825813728614_f64,
                nu: 0.3333333333333333_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.6109134564811405_f64,
                nu: 0.3333333333333333_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.174324229724194_f64,
                nu: 0.3333333333333333_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.8774551722182093_f64,
                nu: 0.3333333333333333_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.755956937164695_f64,
                nu: 0.3333333333333333_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.8546499652731825_f64,
                nu: 0.3333333333333333_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.229979861917113_f64,
                nu: 0.3333333333333333_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.953111435550632_f64,
                nu: 0.3333333333333333_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 11.113838389991479_f64,
                nu: 0.3333333333333333_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 13.825531136529117_f64,
                nu: 0.3333333333333333_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 17.23140396847832_f64,
                nu: 0.3333333333333333_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 21.512458099556554_f64,
                nu: 0.3333333333333333_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 26.897553069268362_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_13: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data009_14: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 0.5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.40311093489975897_f64,
                nu: 0.5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.5879930867904157_f64,
                nu: 0.5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.757614986389913_f64,
                nu: 0.5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.937674888245487_f64,
                nu: 0.5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.1432089853159872_f64,
                nu: 0.5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.387161720403476_f64,
                nu: 0.5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.6830217804556813_f64,
                nu: 0.5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.0462368630890526_f64,
                nu: 0.5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.4953405089360046_f64,
                nu: 0.5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.0530935381967175_f64,
                nu: 0.5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.747788249487945_f64,
                nu: 0.5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.614822903407597_f64,
                nu: 0.5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.698650532533549_f64,
                nu: 0.5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.055219408691186_f64,
                nu: 0.5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.755046784118898_f64,
                nu: 0.5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 10.887101798588422_f64,
                nu: 0.5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 13.563718712579764_f64,
                nu: 0.5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 16.926820080158183_f64,
                nu: 0.5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 21.155804306570005_f64,
                nu: 0.5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 26.477547497559065_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_15: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data010_16: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 0.6666666666666666_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.27953690613200444_f64,
                nu: 0.6666666666666666_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.4562832311355689_f64,
                nu: 0.6666666666666666_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.6259456983818262_f64,
                nu: 0.6666666666666666_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.8075212886061297_f64,
                nu: 0.6666666666666666_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0139484513577173_f64,
                nu: 0.6666666666666666_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2572918396962993_f64,
                nu: 0.6666666666666666_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.5505806938325581_f64,
                nu: 0.6666666666666666_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.908949296823621_f64,
                nu: 0.6666666666666666_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.350646349030034_f64,
                nu: 0.6666666666666666_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.8981161894224896_f64,
                nu: 0.6666666666666666_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.5792654911068724_f64,
                nu: 0.6666666666666666_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.429008721354951_f64,
                nu: 0.6666666666666666_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.4911895720097705_f64,
                nu: 0.6666666666666666_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.8209918044137305_f64,
                nu: 0.6666666666666666_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.487978424961979_f64,
                nu: 0.6666666666666666_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 10.579932774013004_f64,
                nu: 0.6666666666666666_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 13.207720355482458_f64,
                nu: 0.6666666666666666_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 16.511448404200543_f64,
                nu: 0.6666666666666666_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 20.668274532832392_f64,
                nu: 0.6666666666666666_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 25.902310583215122_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_17: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data011_18: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.12597910894546793_f64,
                nu: 1_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.2578943053908963_f64,
                nu: 1_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.4019924615809223_f64,
                nu: 1_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.565159103992485_f64,
                nu: 1_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.7552814183407472_f64,
                nu: 1_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.9816664285779072_f64,
                nu: 1_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2555375122401728_f64,
                nu: 1_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.590636854637329_f64,
                nu: 1_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.003967456929593_f64,
                nu: 1_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.5167162452886984_f64,
                nu: 1_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.1554101386190028_f64,
                nu: 1_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.9533702174026097_f64,
                nu: 1_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.952546165908549_f64,
                nu: 1_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.205834922258364_f64,
                nu: 1_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.780015229824417_f64,
                nu: 1_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 9.759465153704452_f64,
                nu: 1_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 12.250874667409304_f64,
                nu: 1_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 15.389222753735924_f64,
                nu: 1_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 19.345361447520226_f64,
                nu: 1_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 24.335642142450524_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_19: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data012_20: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.007853269659864517_f64,
                nu: 2_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.03190614917773825_f64,
                nu: 2_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.07366688049487544_f64,
                nu: 2_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.1357476697670383_f64,
                nu: 2_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.22201844837663415_f64,
                nu: 2_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.3378346183356807_f64,
                nu: 2_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.49035213986973314_f64,
                nu: 2_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.6889484476987383_f64,
                nu: 2_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.9457739010311572_f64,
                nu: 2_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2764661478191643_f64,
                nu: 2_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.701069370060199_f64,
                nu: 2_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.245212440929951_f64,
                nu: 2_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.9416152804573357_f64,
                nu: 2_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.8320120480778415_f64,
                nu: 2_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.969604404938211_f64,
                nu: 2_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.4221893752841055_f64,
                nu: 2_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.276146192455055_f64,
                nu: 2_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 10.641517298393307_f64,
                nu: 2_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 13.658483394577813_f64,
                nu: 2_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 17.505614966624233_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_21: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data013_22: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.5497616449882785e-07_f64,
                nu: 5_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.223171313109265e-06_f64,
                nu: 5_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.326112273981173e-05_f64,
                nu: 5_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.00027146315595697195_f64,
                nu: 5_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0008479361361668686_f64,
                nu: 5_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0021705595690975554_f64,
                nu: 5_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0048504513371845385_f64,
                nu: 5_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.009825679323131702_f64,
                nu: 5_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.01848657794104583_f64,
                nu: 5_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.03284347517202322_f64,
                nu: 5_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.05575088275422194_f64,
                nu: 5_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.09120647766151334_f64,
                nu: 5_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.14474880546308083_f64,
                nu: 5_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.2239849547019078_f64,
                nu: 5_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.33928899170999877_f64,
                nu: 5_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.5047243631131664_f64,
                nu: 5_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.7392596181668296_f64,
                nu: 5_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0683677743764701_f64,
                nu: 5_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.5261268693599621_f64,
                nu: 5_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.1579745473225476_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_23: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data014_24: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.5701232848571186e-16_f64,
                nu: 10_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.6430419258812784e-13_f64,
                nu: 10_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.5349659676120412e-11_f64,
                nu: 10_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.752948039836873e-10_f64,
                nu: 10_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.5967897782035928e-09_f64,
                nu: 10_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.6330924437799743e-08_f64,
                nu: 10_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.770667683461408e-08_f64,
                nu: 10_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.016963879350684e-07_f64,
                nu: 10_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.003445905777448e-06_f64,
                nu: 10_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.955743610968058e-06_f64,
                nu: 10_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.895560377408272e-06_f64,
                nu: 10_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.946439347061297e-05_f64,
                nu: 10_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.4875369479742435e-05_f64,
                nu: 10_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 9.776084851452892e-05_f64,
                nu: 10_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.00020289011210063496_f64,
                nu: 10_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0004037889613269306_f64,
                nu: 10_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0007747851955166989_f64,
                nu: 10_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0014397060684919682_f64,
                nu: 10_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.002600448601618945_f64,
                nu: 10_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0045800444191760525_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_25: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data015_26: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 20_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.5677858077910353e-37_f64,
                nu: 20_f64,
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.7494538480790194e-31_f64,
                nu: 20_f64,
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2514356342425337e-27_f64,
                nu: 20_f64,
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.9668359858190197e-25_f64,
                nu: 20_f64,
                x: 1_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.4637832909868234e-23_f64,
                nu: 20_f64,
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.3388331839683472e-21_f64,
                nu: 20_f64,
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.950237673267975e-20_f64,
                nu: 20_f64,
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.310560576109548e-19_f64,
                nu: 20_f64,
                x: 2_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.603245140643306e-18_f64,
                nu: 20_f64,
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.840031724417031e-17_f64,
                nu: 20_f64,
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.6239115263043263e-16_f64,
                nu: 20_f64,
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.5209660019426689e-15_f64,
                nu: 20_f64,
                x: 3_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.680645072824995e-15_f64,
                nu: 20_f64,
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.4495528847222945e-14_f64,
                nu: 20_f64,
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.4006589294850677e-13_f64,
                nu: 20_f64,
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.210073422199305e-13_f64,
                nu: 20_f64,
                x: 4_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.7946903269488168e-12_f64,
                nu: 20_f64,
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.77638305622797e-12_f64,
                nu: 20_f64,
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.7502433074548735e-11_f64,
                nu: 20_f64,
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.0242393579718066e-11_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler015_27: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data016_28: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 1_f64,
                nu: 0_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 27.23987182360444_f64,
                nu: 0_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2815.7166284662558_f64,
                nu: 0_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 339649.3732979138_f64,
                nu: 0_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 43558282.559553474_f64,
                nu: 0_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5774560606.466312_f64,
                nu: 0_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 781672297823.9792_f64,
                nu: 0_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 107338818494514.42_f64,
                nu: 0_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 14894774793419916_f64,
                nu: 0_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2083414075177316352_f64,
                nu: 0_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 293255378384934567936_f64,
                nu: 0_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.148789560733216e+22_f64,
                nu: 0_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.894077055609822e+24_f64,
                nu: 0_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.403039845625558e+26_f64,
                nu: 0_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2015889579125424e+29_f64,
                nu: 0_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.7226390780357976e+31_f64,
                nu: 0_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.475178404334166e+33_f64,
                nu: 0_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.56347763040814e+35_f64,
                nu: 0_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.139238345508648e+37_f64,
                nu: 0_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.423325861875207e+39_f64,
                nu: 0_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0737517071310986e+42_f64,
                nu: 0_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler016_29: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data017_30: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 0.3333333333333333_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 26.897553069268362_f64,
                nu: 0.3333333333333333_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2799.239609705679_f64,
                nu: 0.3333333333333333_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 338348.63146593666_f64,
                nu: 0.3333333333333333_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 43434263.927938424_f64,
                nu: 0.3333333333333333_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5761474759.621364_f64,
                nu: 0.3333333333333333_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 780201111830.3024_f64,
                nu: 0.3333333333333333_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 107166066959051.9_f64,
                nu: 0.3333333333333333_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 14873836574083762_f64,
                nu: 0.3333333333333333_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2080814302021708544_f64,
                nu: 0.3333333333333333_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 292926393656442257408_f64,
                nu: 0.3333333333333333_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.144562162412049e+22_f64,
                nu: 0.3333333333333333_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.888575837436592e+24_f64,
                nu: 0.3333333333333333_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.395804702108394e+26_f64,
                nu: 0.3333333333333333_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2006287819446431e+29_f64,
                nu: 0.3333333333333333_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.7213548977150022e+31_f64,
                nu: 0.3333333333333333_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.473449245844445e+33_f64,
                nu: 0.3333333333333333_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.561135454785712e+35_f64,
                nu: 0.3333333333333333_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.136049129555185e+37_f64,
                nu: 0.3333333333333333_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.418962909760043e+39_f64,
                nu: 0.3333333333333333_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.073152330835837e+42_f64,
                nu: 0.3333333333333333_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler017_31: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data018_32: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 0.5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 26.477547497559065_f64,
                nu: 0.5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2778.784603874571_f64,
                nu: 0.5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 336729.887187064_f64,
                nu: 0.5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 43279746.27242892_f64,
                nu: 0.5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5745159748.346468_f64,
                nu: 0.5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 778366068840.4458_f64,
                nu: 0.5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 106950522408567.66_f64,
                nu: 0.5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 14847705549021960_f64,
                nu: 0.5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2077569182462566144_f64,
                nu: 0.5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 292515685299129843712_f64,
                nu: 0.5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.139284009478122e+22_f64,
                nu: 0.5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.881706576075195e+24_f64,
                nu: 0.5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.386769578727724e+26_f64,
                nu: 0.5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.1994296461653203e+29_f64,
                nu: 0.5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.7197510246063334e+31_f64,
                nu: 0.5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.4712895036230794e+33_f64,
                nu: 0.5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.558209908675777e+35_f64,
                nu: 0.5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.132065403123113e+37_f64,
                nu: 0.5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.413512838349524e+39_f64,
                nu: 0.5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.072403582542318e+42_f64,
                nu: 0.5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler018_33: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data019_34: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 0.6666666666666666_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 25.902310583215122_f64,
                nu: 0.6666666666666666_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2750.4090423459315_f64,
                nu: 0.6666666666666666_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 334476.98138574377_f64,
                nu: 0.6666666666666666_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 43064361.68691231_f64,
                nu: 0.6666666666666666_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5722397441.960388_f64,
                nu: 0.6666666666666666_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 775804343498.0266_f64,
                nu: 0.6666666666666666_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 106649495512800.88_f64,
                nu: 0.6666666666666666_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 14811199896983754_f64,
                nu: 0.6666666666666666_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2073034581435696128_f64,
                nu: 0.6666666666666666_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 291941667552574668800_f64,
                nu: 0.6666666666666666_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.131905956993537e+22_f64,
                nu: 0.6666666666666666_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.872103147638622e+24_f64,
                nu: 0.6666666666666666_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.374136824821783e+26_f64,
                nu: 0.6666666666666666_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.1977528777008688e+29_f64,
                nu: 0.6666666666666666_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.7175081240014333e+31_f64,
                nu: 0.6666666666666666_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.4682690458513916e+33_f64,
                nu: 0.6666666666666666_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.5541181975850724e+35_f64,
                nu: 0.6666666666666666_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.126493396322889e+37_f64,
                nu: 0.6666666666666666_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.405889488013406e+39_f64,
                nu: 0.6666666666666666_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0713562154788124e+42_f64,
                nu: 0.6666666666666666_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler019_35: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data020_36: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 1_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 24.335642142450524_f64,
                nu: 1_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2670.988303701256_f64,
                nu: 1_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 328124.9219702065_f64,
                nu: 1_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 42454973.38512778_f64,
                nu: 1_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5657865129.878702_f64,
                nu: 1_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 768532038938.9567_f64,
                nu: 1_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 105794126051896.17_f64,
                nu: 1_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 14707396163259352_f64,
                nu: 1_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2060133462081577984_f64,
                nu: 1_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 290307859010356379648_f64,
                nu: 1_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.110898645299281e+22_f64,
                nu: 1_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.844751588390453e+24_f64,
                nu: 1_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.338148547150129e+26_f64,
                nu: 1_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.1929750788892366e+29_f64,
                nu: 1_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.7111160152965382e+31_f64,
                nu: 1_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.4596595795675343e+33_f64,
                nu: 1_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.5424536064404024e+35_f64,
                nu: 1_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.110606815256613e+37_f64,
                nu: 1_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.384151809136018e+39_f64,
                nu: 1_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.068369390338157e+42_f64,
                nu: 1_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler020_37: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data021_38: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 2_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 17.505614966624233_f64,
                nu: 2_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2281.5189677260046_f64,
                nu: 2_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 295899.3837018863_f64,
                nu: 2_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 39312785.221040756_f64,
                nu: 2_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5321931396.076015_f64,
                nu: 2_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 730436828561.3801_f64,
                nu: 2_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 101293439862977.19_f64,
                nu: 2_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 14159404985256920_f64,
                nu: 2_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1991852587973688320_f64,
                nu: 2_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 281643064024519376896_f64,
                nu: 2_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.999302337267754e+22_f64,
                nu: 2_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.699252002663443e+24_f64,
                nu: 2_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.146481428790036e+26_f64,
                nu: 2_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.1675039556585663e+29_f64,
                nu: 2_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.6770093176278926e+31_f64,
                nu: 2_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.413686914844988e+33_f64,
                nu: 2_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.4801257808448186e+35_f64,
                nu: 2_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.025669305169631e+37_f64,
                nu: 2_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.267870034314582e+39_f64,
                nu: 2_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0523843193243042e+42_f64,
                nu: 2_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler021_39: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data022_40: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 5_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.1579745473225476_f64,
                nu: 5_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 777.1882864032601_f64,
                nu: 5_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 144572.0112006341_f64,
                nu: 5_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 23018392.21341367_f64,
                nu: 5_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3472466208.7419176_f64,
                nu: 5_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 512151465476.93494_f64,
                nu: 5_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 74756743552251.55_f64,
                nu: 5_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 10858318337624278_f64,
                nu: 5_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1573608739924591104_f64,
                nu: 5_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 227854830791128252416_f64,
                nu: 5_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.2989391052963687e+22_f64,
                nu: 5_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.777765207256173e+24_f64,
                nu: 5_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.923216514717266e+26_f64,
                nu: 5_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0038643002095155e+29_f64,
                nu: 5_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.4566328222327073e+31_f64,
                nu: 5_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.1151488565944835e+33_f64,
                nu: 5_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.073588345076824e+35_f64,
                nu: 5_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.469479018923033e+37_f64,
                nu: 5_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.5037505570431e+39_f64,
                nu: 5_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 9.470093873035588e+41_f64,
                nu: 5_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler022_41: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data023_42: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 10_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0045800444191760525_f64,
                nu: 10_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 21.89170616372338_f64,
                nu: 10_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 12267.475049806462_f64,
                nu: 10_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3540200.2090195213_f64,
                nu: 10_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 771298871.1707267_f64,
                nu: 10_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 145831809975.9671_f64,
                nu: 10_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 25449470018534.777_f64,
                nu: 10_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4228469210516757_f64,
                nu: 10_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 680494045575051648_f64,
                nu: 10_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 107159715947763695616_f64,
                nu: 10_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.6618215752886714e+22_f64,
                nu: 10_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.5486246072566784e+24_f64,
                nu: 10_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.8764628702155475e+26_f64,
                nu: 10_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.859253814540969e+28_f64,
                nu: 10_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 8.813537071131744e+30_f64,
                nu: 10_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.320741826832528e+33_f64,
                nu: 10_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.973279136086219e+35_f64,
                nu: 10_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.941189374838467e+37_f64,
                nu: 10_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.3754494922439984e+39_f64,
                nu: 10_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.498975524720145e+41_f64,
                nu: 10_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler023_43: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data024_44: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 20_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.0242393579718066e-11_f64,
                nu: 20_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0001250799735644948_f64,
                nu: 20_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.6470152535015836_f64,
                nu: 20_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3188.7503288536154_f64,
                nu: 20_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2449840.54229523_f64,
                nu: 20_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1126985104.4483771_f64,
                nu: 20_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 379617876611.8858_f64,
                nu: 20_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 104459633129479.88_f64,
                nu: 20_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 25039579987216524_f64,
                nu: 20_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5442008402752998400_f64,
                nu: 20_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1100749858433549467648_f64,
                nu: 20_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.1091734863057236e+23_f64,
                nu: 20_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.876361809128689e+25_f64,
                nu: 20_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.894613052793087e+27_f64,
                nu: 20_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.1946319948836447e+30_f64,
                nu: 20_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.0265314377577587e+32_f64,
                nu: 20_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.3784665214179985e+34_f64,
                nu: 20_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.551608941179665e+36_f64,
                nu: 20_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 9.012931079530515e+38_f64,
                nu: 20_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.4483461256427176e+41_f64,
                nu: 20_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler024_45: Value<()> = Rc::new(RefCell::new(1.0000000000000008e-12_f64));
);
thread_local!(
    pub static data025_46: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 50_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.931469646810852e-45_f64,
                nu: 50_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.7568945607268435e-30_f64,
                nu: 50_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.546837273066707e-21_f64,
                nu: 50_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.2551205757604056e-14_f64,
                nu: 50_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.534425186613026e-09_f64,
                nu: 50_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0001459010691646894_f64,
                nu: 50_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.3965549457254882_f64,
                nu: 50_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5726.865663128989_f64,
                nu: 50_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 12672593.113027781_f64,
                nu: 50_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 17650802430.016712_f64,
                nu: 50_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 17220231607789.926_f64,
                nu: 50_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 12704607933652176_f64,
                nu: 50_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7498949194219371520_f64,
                nu: 50_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.694403489890492e+21_f64,
                nu: 50_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.5691634774370186e+24_f64,
                nu: 50_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.892774945816359e+26_f64,
                nu: 50_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.995884905474934e+29_f64,
                nu: 50_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.19460503617815e+31_f64,
                nu: 50_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.784542972869712e+34_f64,
                nu: 50_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.821958085594082e+36_f64,
                nu: 50_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler025_47: Value<()> = Rc::new(RefCell::new(5.000000000000003e-12_f64));
);
thread_local!(
    pub static data026_48: Value<Box<[testcase_cyl_bessel_i_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_cyl_bessel_i_double_ {
                f0: 0_f64,
                nu: 100_f64,
                x: 0_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 7.093551488531312e-119_f64,
                nu: 100_f64,
                x: 5_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.0823442017492015e-88_f64,
                nu: 100_f64,
                x: 10_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 5.98878885364689e-71_f64,
                nu: 100_f64,
                x: 15_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.870319321642877e-58_f64,
                nu: 100_f64,
                x: 20_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.442689691312237e-48_f64,
                nu: 100_f64,
                x: 25_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3.947642005333427e-40_f64,
                nu: 100_f64,
                x: 30_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4.283659618081878e-33_f64,
                nu: 100_f64,
                x: 35_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 6.624938022259613e-27_f64,
                nu: 100_f64,
                x: 40_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.37025872627889e-21_f64,
                nu: 100_f64,
                x: 45_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.7278879470966917e-16_f64,
                nu: 100_f64,
                x: 50_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1.2763258878228082e-11_f64,
                nu: 100_f64,
                x: 55_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2.883277090649197e-07_f64,
                nu: 100_f64,
                x: 60_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 0.0035805902717061223_f64,
                nu: 100_f64,
                x: 65_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 27.017219102595387_f64,
                nu: 100_f64,
                x: 70_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 134001.44891209516_f64,
                nu: 100_f64,
                x: 75_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 465194832.85060996_f64,
                nu: 100_f64,
                x: 80_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 1189280653119.4814_f64,
                nu: 100_f64,
                x: 85_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 2334119331258728_f64,
                nu: 100_f64,
                x: 90_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 3639922307850243584_f64,
                nu: 100_f64,
                x: 95_f64,
                f: 0_f64,
            },
            testcase_cyl_bessel_i_double_ {
                f0: 4641534941616200482816_f64,
                nu: 100_f64,
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler026_49: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
pub fn test_50(data: Ptr<testcase_cyl_bessel_i_double_>, mut toler: ()) {
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
                __builtin_fprintf_51 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"check_value.cc" )  ) .into() , ( 725_() ) .into() , ( Ptr::<() >::from_string_literal(b"void test(const testcase_cyl_bessel_i<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" )  ) .into() , ( Ptr::<() >::from_string_literal(b"max_abs_frac < toler" )  ) .into() , ] ) )
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
        test_50(
            data007_10.with(|v| v.as_pointer()),
            toler007_11.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data008_12.with(|v| v.as_pointer()),
            toler008_13.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data009_14.with(|v| v.as_pointer()),
            toler009_15.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data010_16.with(|v| v.as_pointer()),
            toler010_17.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data011_18.with(|v| v.as_pointer()),
            toler011_19.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data012_20.with(|v| v.as_pointer()),
            toler012_21.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data013_22.with(|v| v.as_pointer()),
            toler013_23.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data014_24.with(|v| v.as_pointer()),
            toler014_25.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data015_26.with(|v| v.as_pointer()),
            toler015_27.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data016_28.with(|v| v.as_pointer()),
            toler016_29.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data017_30.with(|v| v.as_pointer()),
            toler017_31.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data018_32.with(|v| v.as_pointer()),
            toler018_33.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data019_34.with(|v| v.as_pointer()),
            toler019_35.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data020_36.with(|v| v.as_pointer()),
            toler020_37.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data021_38.with(|v| v.as_pointer()),
            toler021_39.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data022_40.with(|v| v.as_pointer()),
            toler022_41.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data023_42.with(|v| v.as_pointer()),
            toler023_43.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data024_44.with(|v| v.as_pointer()),
            toler024_45.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data025_46.with(|v| v.as_pointer()),
            toler025_47.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_50(
            data026_48.with(|v| v.as_pointer()),
            toler026_49.with(|rc| *rc.borrow()),
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
        field!(this, pid_).write((({ getpid_52() }) as ::libc::pid_t));
        field!(this, sem_set_)
            .write(({ semget_53(((0_() as ()) as ::libc::key_t), 1_(), (256_() | 128_())) }));
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
            semctl_54(
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
pub fn set_memory_limits_55(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = (size * 1048576_());
    ({ getrlimit_56((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_57((__rlimit_resource_RLIMIT_DATA as ()), (r.as_pointer())) });
    ({ getrlimit_56((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_57((__rlimit_resource___RLIMIT_RSS as ()), (r.as_pointer())) });
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    ({ getrlimit_56((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_57((__rlimit_resource_RLIMIT_AS as ()), (r.as_pointer())) });
}
pub fn set_file_limit_58(mut size: ()) {
    let r: Value<rlimit> = Rc::new(RefCell::new(<rlimit>::default()));
    let mut limit: () = ((size) as ());
    ({ getrlimit_56((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
    (*r.borrow_mut()).rlim_cur = limit;
    ({ setrlimit_57((__rlimit_resource_RLIMIT_FSIZE as ()), (r.as_pointer())) });
}
pub fn verify_demangle_59(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = ({
        __cxa_demangle_60(
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
pub fn run_tests_wrapped_locale_61(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
                    __builtin_fprintf_51 ( libcc2rs::stderr_unsafe()  , Ptr::<() >::from_string_literal(b"%s:%d: %s: Assertion \'%s\' failed.\n" )  , Box::new ( [ ( Ptr::<() >::from_string_literal(b"testsuite_hooks.cc" )  ) .into() , ( 182_() ) .into() , ( Ptr::<() >::from_string_literal(b"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" )  ) .into() , ( Ptr::<() >::from_string_literal(b"preLC_ALL == postLC_ALL" )  ) .into() , ] ) )
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
pub fn run_tests_wrapped_env_62(
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
pub fn test_tm_64(
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
    (*tmp_65.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_65.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return tmp_65.with(|rc| *rc.borrow());
}
thread_local!(
    static tmp_65: Value<::libc::tm> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
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
        if ((*self).with(|__s| __s.pid_) == ({ getpid_52() })) {
            ({
                semctl_54(
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
            semop_63(
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
            semop_63(
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
    let _ = data007_10.with(|_| ());
    let _ = toler007_11.with(|_| ());
    let _ = data008_12.with(|_| ());
    let _ = toler008_13.with(|_| ());
    let _ = data009_14.with(|_| ());
    let _ = toler009_15.with(|_| ());
    let _ = data010_16.with(|_| ());
    let _ = toler010_17.with(|_| ());
    let _ = data011_18.with(|_| ());
    let _ = toler011_19.with(|_| ());
    let _ = data012_20.with(|_| ());
    let _ = toler012_21.with(|_| ());
    let _ = data013_22.with(|_| ());
    let _ = toler013_23.with(|_| ());
    let _ = data014_24.with(|_| ());
    let _ = toler014_25.with(|_| ());
    let _ = data015_26.with(|_| ());
    let _ = toler015_27.with(|_| ());
    let _ = data016_28.with(|_| ());
    let _ = toler016_29.with(|_| ());
    let _ = data017_30.with(|_| ());
    let _ = toler017_31.with(|_| ());
    let _ = data018_32.with(|_| ());
    let _ = toler018_33.with(|_| ());
    let _ = data019_34.with(|_| ());
    let _ = toler019_35.with(|_| ());
    let _ = data020_36.with(|_| ());
    let _ = toler020_37.with(|_| ());
    let _ = data021_38.with(|_| ());
    let _ = toler021_39.with(|_| ());
    let _ = data022_40.with(|_| ());
    let _ = toler022_41.with(|_| ());
    let _ = data023_42.with(|_| ());
    let _ = toler023_43.with(|_| ());
    let _ = data024_44.with(|_| ());
    let _ = toler024_45.with(|_| ());
    let _ = data025_46.with(|_| ());
    let _ = toler025_47.with(|_| ());
    let _ = data026_48.with(|_| ());
    let _ = toler026_49.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__4.with(|_| ());
    let _ = throw_on__5.with(|_| ());
    let _ = _M_count_6.with(|_| ());
    let _ = next_id__7.with(|_| ());
}
