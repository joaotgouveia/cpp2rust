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
pub struct testcase_sph_neumann_double_ {
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
    pub static data001_9: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -3.875649686842579_f64,
                n: 0_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.7551651237807455_f64,
                n: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.9755851584984279_f64,
                n: 0_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5403023058681398_f64,
                n: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.25225788991621495_f64,
                n: 0_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.04715813444513527_f64,
                n: 0_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.1018548889425669_f64,
                n: 0_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2080734182735712_f64,
                n: 0_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.27918827676566177_f64,
                n: 0_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.3204574462187735_f64,
                n: 0_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.3361099558663504_f64,
                n: 0_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.3299974988668151_f64,
                n: 0_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.30588605417862963_f64,
                n: 0_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2675590535116561_f64,
                n: 0_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.21881582862388288_f64,
                n: 0_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.16341090521590299_f64,
                n: 0_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.10496176233265714_f64,
                n: 0_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.04684351098461772_f64,
                n: 0_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007916242713258222_f64,
                n: 0_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.05673243709264526_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -16.492214584388407_f64,
                n: 1_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.469181324769897_f64,
                n: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.209631891362349_f64,
                n: 1_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3817732906760363_f64,
                n: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.9609940074174409_f64,
                n: 1_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.6964354140327931_f64,
                n: 1_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5040748902464972_f64,
                n: 1_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.35061200427605527_f64,
                n: 1_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.2217266311654487_f64,
                n: 1_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.11120587915407318_f64,
                n: 1_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.016564013158538646_f64,
                n: 1_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.06295916360231597_f64,
                n: 1_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.12740959652576553_f64,
                n: 1_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.17666922320036457_f64,
                n: 1_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.21076723929766045_f64,
                n: 1_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2300533501309578_f64,
                n: 1_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.23528261660264485_f64,
                n: 1_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.22763858414438104_f64,
                n: 1_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.2087108518446568_f64,
                n: 1_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.18043836751409864_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(5.000000000000004e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -194.03092532581832_f64,
                n: 2_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -25.059922824838637_f64,
                n: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.862942406950969_f64,
                n: 2_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.605017566159969_f64,
                n: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.054127727885643_f64,
                n: 2_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3457126936204509_f64,
                n: 2_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.9659832722222763_f64,
                n: 2_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.733991424687654_f64,
                n: 2_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5748237849862601_f64,
                n: 2_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.4539045012036613_f64,
                n: 2_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.35417978840293796_f64,
                n: 2_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.26703833526449916_f64,
                n: 2_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.18827719584715374_f64,
                n: 2_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.11612829076848646_f64,
                n: 2_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0502020371857545_f64,
                n: 2_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.009129107382315343_f64,
                n: 2_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.06112008468097453_f64,
                n: 2_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.10491554511163632_f64,
                n: 2_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.13973362282567303_f64,
                n: 2_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.16499545760110443_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -3884190.062663773_f64,
                n: 5_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -61327.56316698064_f64,
                n: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5478.952932319084_f64,
                n: 5_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -999.4403433922364_f64,
                n: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -270.4972050294236_f64,
                n: 5_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -94.23611008523247_f64,
                n: 5_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -39.18282778658433_f64,
                n: 5_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -18.591445311190984_f64,
                n: 5_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9.782142020318227_f64,
                n: 5_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5.599100154806323_f64,
                n: 5_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.4400655233636823_f64,
                n: 5_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.2470233284653904_f64,
                n: 5_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.549143994577916_f64,
                n: 5_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.1205896325654248_f64,
                n: 5_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.8459225560519484_f64,
                n: 5_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.6628012664504588_f64,
                n: 5_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.5358937443603853_f64,
                n: 5_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.4443032422909055_f64,
                n: 5_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.3752015723289989_f64,
                n: 5_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.3204650467497392_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -2750653598174213.5_f64,
                n: 10_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1349739281107.0554_f64,
                n: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -15733380424.95376_f64,
                n: 10_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -672215008.2562084_f64,
                n: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -58607405.98867945_f64,
                n: 10_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -8032728.814823481_f64,
                n: 10_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1505955.5720640516_f64,
                n: 10_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -355414.72008543846_f64,
                n: 10_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -100086.80374425423_f64,
                n: 10_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -32423.79408533442_f64,
                n: 10_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -11772.863161809979_f64,
                n: 10_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4699.859188811392_f64,
                n: 10_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2033.0183273853759_f64,
                n: 10_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -942.1907502842549_f64,
                n: 10_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -463.65206971202474_f64,
                n: 10_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -240.5355298798893_f64,
                n: 10_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -130.78478404631085_f64,
                n: 10_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -74.17066550173753_f64,
                n: 10_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -43.69824989818498_f64,
                n: 10_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -26.65611440571871_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -1.407759140254225e+36_f64,
                n: 20_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.728876183823471e+29_f64,
                n: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3544611382105945e+26_f64,
                n: 20_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.239592218578983e+23_f64,
                n: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.009641671595306e+21_f64,
                n: 20_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -65999646851668172800_f64,
                n: 20_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2619336475307073536_f64,
                n: 20_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -160543649281522240_f64,
                n: 20_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -13719071872797762_f64,
                n: 20_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1524247248298953.8_f64,
                n: 20_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -209484650509384.06_f64,
                n: 20_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -34327545666696.49_f64,
                n: 20_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6522260876203.317_f64,
                n: 20_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1406018871897.2307_f64,
                n: 20_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -338025193731.7888_f64,
                n: 20_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -89381690326.01868_f64,
                n: 20_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -25701805899.474934_f64,
                n: 20_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7961859734.240776_f64,
                n: 20_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2636237230.085001_f64,
                n: 20_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -926795140.3057547_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -1.382374280800406e+109_f64,
                n: 50_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.144791292212169e+93_f64,
                n: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.434849490890053e+84_f64,
                n: 50_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.739192284629757e+78_f64,
                n: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.136503757329993e+73_f64,
                n: 50_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.8821098528635756e+69_f64,
                n: 50_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.1148255024189452e+66_f64,
                n: 50_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.235021944367097e+63_f64,
                n: 50_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.0565226939717125e+60_f64,
                n: 50_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.4262702131152733e+58_f64,
                n: 50_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.1118745474840939e+56_f64,
                n: 50_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3243260716629503e+54_f64,
                n: 50_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.251947209412933e+52_f64,
                n: 50_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5.186150720110329e+50_f64,
                n: 50_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.5513212909462263e+49_f64,
                n: 50_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -5.827647140790065e+47_f64,
                n: 50_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.6745414086543416e+46_f64,
                n: 50_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.4657308996352946e+45_f64,
                n: 50_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9.410267436669065e+43_f64,
                n: 50_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.964109188270132e+42_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.5000000000000015e-12_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -4.285610946051641e+247_f64,
                n: 100_(),
                x: 0.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.691172001175378e+217_f64,
                n: 100_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.7753107402139484e+199_f64,
                n: 100_(),
                x: 0.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.683079463258677e+186_f64,
                n: 100_(),
                x: 1_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.0906342369729277e+177_f64,
                n: 100_(),
                x: 1.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.0993184254131119e+169_f64,
                n: 100_(),
                x: 1.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.9071480498141315e+162_f64,
                n: 100_(),
                x: 1.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.6559558301924957e+156_f64,
                n: 100_(),
                x: 2_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.8154136926485787e+151_f64,
                n: 100_(),
                x: 2.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.352763166211138e+146_f64,
                n: 100_(),
                x: 2.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.880953701410059e+142_f64,
                n: 100_(),
                x: 2.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.410222995302561e+138_f64,
                n: 100_(),
                x: 3_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.365190415404474e+135_f64,
                n: 100_(),
                x: 3.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.698074910106327e+131_f64,
                n: 100_(),
                x: 3.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.279055349924666e+128_f64,
                n: 100_(),
                x: 3.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.0796647795891517e+126_f64,
                n: 100_(),
                x: 4_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.3785795774441247e+123_f64,
                n: 100_(),
                x: 4.25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.439159663194743e+120_f64,
                n: 100_(),
                x: 4.5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -3.1802258278275794e+118_f64,
                n: 100_(),
                x: 4.75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.799713982625566e+116_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
thread_local!(
    pub static data009_25: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -0.05673243709264526_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.08390715290764525_f64,
                n: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.05064586085725475_f64,
                n: 0_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.020404103090669597_f64,
                n: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03964811247453894_f64,
                n: 0_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.005141714996252802_f64,
                n: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.025819777288328762_f64,
                n: 0_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.016673451541306544_f64,
                n: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.011673821973727327_f64,
                n: 0_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.019299320569842265_f64,
                n: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00040230465930828606_f64,
                n: 0_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.015873549673585938_f64,
                n: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008653136172894954_f64,
                n: 0_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00904741718694714_f64,
                n: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.012290016929663325_f64,
                n: 0_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0013798405479880944_f64,
                n: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.011580901686988727_f64,
                n: 0_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.004978595734768557_f64,
                n: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007686037484155996_f64,
                n: 0_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00862318872287684_f64,
                n: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_26: Value<()> = Rc::new(RefCell::new(5.000000000000003e-11_f64));
);
thread_local!(
    pub static data010_27: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: 0.18043836751409864_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0627928263797015_f64,
                n: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03997613195332415_f64,
                n: 1_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.046667467690914864_f64,
                n: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0037081455049293634_f64,
                n: 1_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.032762996969886965_f64,
                n: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012971498479556563_f64,
                n: 1_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.018210992723451058_f64,
                n: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.01916838547795213_f64,
                n: 1_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.00486151066268173_f64,
                n: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.018170052158169303_f64,
                n: 1_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.005344736179596711_f64,
                n: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.012587316051033977_f64,
                n: 1_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.01118482998206909_f64,
                n: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.005006554913063562_f64,
                n: 1_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012440856180892041_f64,
                n: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.002207723783947951_f64,
                n: 1_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.009877978531842104_f64,
                n: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007273134233897652_f64,
                n: 1_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.00497742452386882_f64,
                n: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_28: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
thread_local!(
    pub static data011_29: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: 0.16499545760110443_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.06506930499373478_f64,
                n: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.058641087247919575_f64,
                n: 2_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01340398293703237_f64,
                n: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.04009308993513046_f64,
                n: 2_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008418014693241499_f64,
                n: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.024707934561509628_f64,
                n: 2_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.018039275995565374_f64,
                n: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010395929608530518_f64,
                n: 2_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01959101120960317_f64,
                n: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.001393398413390248_f64,
                n: 2_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.015606312864606101_f64,
                n: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.009234089221404215_f64,
                n: 2_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008568067330572752_f64,
                n: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012490279126185866_f64,
                n: 2_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0009133084412046427_f64,
                n: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.01150298202402586_f64,
                n: 2_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.005307861685829961_f64,
                n: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.00745635956098028_f64,
                n: 2_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008772511458592905_f64,
                n: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_30: Value<()> = Rc::new(RefCell::new(5.000000000000003e-11_f64));
);
thread_local!(
    pub static data012_31: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -0.3204650467497392_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.09383354167869182_f64,
                n: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.02047569828185906_f64,
                n: 5_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.04817234775737278_f64,
                n: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.018309489232548347_f64,
                n: 5_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.026639390496569996_f64,
                n: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.02200603898557621_f64,
                n: 5_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.011268975348057965_f64,
                n: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.02177038837227486_f64,
                n: 5_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.000697113196458537_f64,
                n: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0174395894502209_f64,
                n: 5_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.008869917091934309_f64,
                n: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010421334444951861_f64,
                n: 5_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.012746769858008553_f64,
                n: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0026282888028967737_f64,
                n: 5_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01247765858132419_f64,
                n: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.004077181681818264_f64,
                n: 5_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.008977775957057982_f64,
                n: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.008318455789667615_f64,
                n: 5_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0037206784862748965_f64,
                n: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_32: Value<()> = Rc::new(RefCell::new(5.000000000000003e-11_f64));
);
thread_local!(
    pub static data013_33: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -26.65611440571871_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.17245367208805784_f64,
                n: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.07846168984964258_f64,
                n: 10_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03684341049628996_f64,
                n: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.021158339301097475_f64,
                n: 10_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.03121959106475494_f64,
                n: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012840593422414807_f64,
                n: 10_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.021803068636888072_f64,
                n: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.014071636804469044_f64,
                n: 10_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.013524687511158758_f64,
                n: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.015684932653180595_f64,
                n: 10_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.005635689556726212_f64,
                n: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.015364490270315362_f64,
                n: 10_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0014525575672261295_f64,
                n: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012648951699549433_f64,
                n: 10_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.006857160806112037_f64,
                n: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.008015115294140146_f64,
                n: 10_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.009813974221901915_f64,
                n: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.002500285407231495_f64,
                n: 10_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010025777373636155_f64,
                n: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_34: Value<()> = Rc::new(RefCell::new(5.000000000000003e-11_f64));
);
thread_local!(
    pub static data014_35: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -926795140.3057547_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1211.2106053526036_f64,
                n: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.5559965765652175_f64,
                n: 20_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.0934011322509144_f64,
                n: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.04403198567527646_f64,
                n: 20_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.03607803360661391_f64,
                n: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.029828405631319645_f64,
                n: 20_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.004841481098676076_f64,
                n: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.020504694681516944_f64,
                n: 20_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.013759531302541216_f64,
                n: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.012783038861734196_f64,
                n: 20_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.013117009421906418_f64,
                n: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010338106075674407_f64,
                n: 20_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010538610814111244_f64,
                n: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010200029094273744_f64,
                n: 20_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007312345094561712_f64,
                n: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010581510354950906_f64,
                n: 20_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0036866374015298723_f64,
                n: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.01049838431833827_f64,
                n: 20_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 5.631729378833498e-05_f64,
                n: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_36: Value<()> = Rc::new(RefCell::new(1.0000000000000007e-09_f64));
);
thread_local!(
    pub static data015_37: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -6.964109188270132e+42_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.528227272351331e+27_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9000490264588767232_f64,
                n: 50_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -9542541667002.576_f64,
                n: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -363518140.7102718_f64,
                n: 50_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -152551.57233157742_f64,
                n: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -386.2659918620869_f64,
                n: 50_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.32905079472911_f64,
                n: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.19968460851503717_f64,
                n: 50_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.04190000150460776_f64,
                n: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010696040672421926_f64,
                n: 50_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.007819876855526828_f64,
                n: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.010088474938191183_f64,
                n: 50_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0062423671279823656_f64,
                n: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.0011284242794942282_f64,
                n: 50_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00939342660374863_f64,
                n: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.013108079602843421_f64,
                n: 50_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.007539660722572193_f64,
                n: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.00426057035528373_f64,
                n: 50_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: 0.010747822973682467_f64,
                n: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler015_38: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
thread_local!(
    pub static data016_39: Value<Box<[testcase_sph_neumann_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_sph_neumann_double_ {
                f0: -1.799713982625566e+116_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -8.573226309328894e+85_f64,
                n: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.9270658593710037e+68_f64,
                n: 100_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.220889358295033e+55_f64,
                n: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -2.086875261300736e+46_f64,
                n: 100_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -4.249612402361085e+38_f64,
                n: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.704289834891003e+32_f64,
                n: 100_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -6.302156526072277e+26_f64,
                n: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1.3199917400493998e+22_f64,
                n: 100_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1125692891326591104_f64,
                n: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -309801083340345.56_f64,
                n: 100_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -232585620046.64404_f64,
                n: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -421135935.93755186_f64,
                n: 100_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -1680637.4531202628_f64,
                n: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -13868.302591128748_f64,
                n: 100_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -227.24385709173205_f64,
                n: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -7.280703878713856_f64,
                n: 100_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.4664815444825085_f64,
                n: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.06727077272065451_f64,
                n: 100_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_sph_neumann_double_ {
                f0: -0.02298385049156227_f64,
                n: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler016_40: Value<()> = Rc::new(RefCell::new(2.5000000000000014e-11_f64));
);
pub fn test_41(data: Ptr<testcase_sph_neumann_double_>, mut toler: ()) {
    let mut test: () = true;
    let mut eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 20_();
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
        test_41(
            data001_9.with(|v| v.as_pointer()),
            toler001_10.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data002_11.with(|v| v.as_pointer()),
            toler002_12.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data003_13.with(|v| v.as_pointer()),
            toler003_14.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data004_15.with(|v| v.as_pointer()),
            toler004_16.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data005_17.with(|v| v.as_pointer()),
            toler005_18.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data006_19.with(|v| v.as_pointer()),
            toler006_20.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data007_21.with(|v| v.as_pointer()),
            toler007_22.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data008_23.with(|v| v.as_pointer()),
            toler008_24.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data009_25.with(|v| v.as_pointer()),
            toler009_26.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data010_27.with(|v| v.as_pointer()),
            toler010_28.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data011_29.with(|v| v.as_pointer()),
            toler011_30.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data012_31.with(|v| v.as_pointer()),
            toler012_32.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data013_33.with(|v| v.as_pointer()),
            toler013_34.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data014_35.with(|v| v.as_pointer()),
            toler014_36.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data015_37.with(|v| v.as_pointer()),
            toler015_38.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_41(
            data016_39.with(|v| v.as_pointer()),
            toler016_40.with(|rc| *rc.borrow()),
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
pub fn set_memory_limits_42(size: Option<()>) {
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
pub fn set_file_limit_43(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_44(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_45(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_46(
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
pub fn test_tm_47(
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
    (*tmp_48.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_48.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_48.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_48: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
