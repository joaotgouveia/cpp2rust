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
#[byte_size(40)]
pub struct testcase_ellint_3_double_ {
    #[offset(0)]
    pub f0: (),
    #[offset(8)]
    pub k: (),
    #[offset(16)]
    pub nu: (),
    #[offset(24)]
    pub phi: (),
    #[offset(32)]
    pub f: (),
}
thread_local!(
    pub static data001_9: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17525427376115027_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35492464591297446_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5438822141615712_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7479740042353251_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9746389845196645_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2334463254523438_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5355247765594913_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8882928567775126_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2805491384227703_f64,
                k: -0.9_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler001_10: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data002_11: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17543204932716244_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35636022898551184_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5488027889838258_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7598883477452927_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9985330300356812_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2759958823999022_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.60511873646394_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9941406879519472_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.429501118783488_f64,
                k: -0.9_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler002_12: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data003_13: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1756104732196841_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3578165994435611_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5538815090521528_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7724687412325144_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0244466254771925_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.32348240776408_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6849848968804237_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1185749045502273_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.607683574334841_f64,
                k: -0.9_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler003_14: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data004_15: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1757895496674622_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35929429810867447_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5591275715424081_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7857831472202539_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0526941001131365_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.37696822345386_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7779437432911238_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.267650934181363_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.8256506968858512_f64,
                k: -0.9_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler004_16: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data005_17: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17596928293938452_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3607938864247282_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5645509666711561_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7999099699786943_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0836647913872215_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4378726836091849_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8880446720682853_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4505848932025227_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.1000689868578615_f64,
                k: -0.9_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler005_18: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data006_19: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17614967734498183_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36231594750319435_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5701625698434957_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8149402591829342_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1178482279283477_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5081455873012106_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0213599730864_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6822467012926827_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.4591069002104677_f64,
                k: -0.9_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler006_20: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data007_21: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17633073723493825_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3638610872349281_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5759742474471624_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8309805194850115_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1558706545698916_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.590557637941567_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.187518601021508_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.988576777131685_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.9549939883570224_f64,
                k: -0.9_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler007_22: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data008_23: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17651246700160939_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3654299354735898_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5819989787767487_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8481563358735286_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1985495623872375_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6892158134027688_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4029722191094236_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.420108494134005_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.6985482312992435_f64,
                k: -0.9_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler008_24: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data009_25: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17669487107954862_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3670231472962842_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5882509971136549_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8666171142220903_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2469779109884802_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8105469760531578_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6989505165893752_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.093521326775742_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.982074081364571_f64,
                k: -0.9_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler009_26: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data010_27: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1768779539460417_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36864140434751286_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5947459536681705_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8865423722605666_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3026595810616726_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9653635459278078_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.1451407527189463_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.374523068031611_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 8.994256203185868_f64,
                k: -0.9_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler010_28: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data011_29: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.175101542413389_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35365068839779396_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5392680440908455_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7358792602807037_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9477094297007117_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1789022995388236_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4323027881876012_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7069629739121677_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9953027776647294_f64,
                k: -0.8_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler011_30: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data012_31: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17527903952342144_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3550770531354855_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5441145598764355_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7474562566680438_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9704695368423856_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2183080025184605_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4943711151994405_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7972401309544201_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1172616484005085_f64,
                k: -0.8_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler012_32: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data013_33: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1754571837508642_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35652404627248163_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5491163851292091_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7596768428213118_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9951352689354377_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2622192109995993_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.565410667634774_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9029531718534984_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.26247894341868_f64,
                k: -0.8_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler013_34: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data014_35: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17563597931587369_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3579922041200513_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5542825369111132_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7726064737697736_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0220015271210958_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.311596531230267_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6478518468813512_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.029045841420348_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4392042002725693_f64,
                k: -0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler014_36: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data015_37: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17581543047866136_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35948208343061633_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5596228089370202_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7863206388923412_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0514333069550323_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3677213138838757_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7451736773665165_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.183010042458683_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6604037035529724_f64,
                k: -0.8_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler015_38: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data016_39: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17599554153999472_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3609942624335154_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5651478617478067_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8009069762237101_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.083889162767934_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.432350665446628_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8625761085390575_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3768757305654766_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.9478781158239746_f64,
                k: -0.8_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler016_40: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data017_41: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17617631684170665_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3625293419366623_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5708693262294516_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8164679674015097_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1199552158519064_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5079766673336394_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0082747447038165_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6315146066775523_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.341812189228805_f64,
                k: -0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler017_42: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data018_43: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1763577607672122_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36408794649916976_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5767999229062414_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8331244141814281_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1603958891464856_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5982855143796213_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.196248440837182_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.987328178611187_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.9268876980046397_f64,
                k: -0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler018_44: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data019_45: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17653987774203392_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36567072568046877_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5829535999655862_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8510199830917611_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2062322059736537_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.709032142091743_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4529058049405066_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.5368893360106948_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.924642205819606_f64,
                k: -0.8_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler019_46: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data020_47: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17672267223433513_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36727835537196063_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5893456936371665_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8703272347113885_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.258867611132335_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.849873190066002_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.836838129930042_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.567484419165406_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 7.2263259298637115_f64,
                k: -0.8_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler020_48: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data021_49: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17496737466916723_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3525468753567793_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5353674027599713_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7260379765168445_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9269829634831345_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1400447527693316_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.365766811719407_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6024686895959162_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8456939983747234_f64,
                k: -0.7_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler021_50: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data022_51: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1751446273730092_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3539652799747045_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5401517958943398_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7373443085447773_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9488895079669705_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1772807959736322_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4231796401075831_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.684185679988747_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9541347343119562_f64,
                k: -0.7_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler022_52: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data023_53: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17532252613350796_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3554041759680752_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5450891303336193_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7492763577771842_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9726170633793634_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2187303976209327_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4887796709222487_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7796581281839212_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0829290325820202_f64,
                k: -0.7_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler023_54: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data024_55: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1755010751632857_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3568640957657196_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5501882731651335_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7618975949439027_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9984462343088562_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2652862989039833_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.564766680869136_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8932499694938163_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2392290510988535_f64,
                k: -0.7_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler024_56: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data025_57: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17568027871494424_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3583455920818026_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5554588545119061_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.775281204025681_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.026724128760032_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3181380338980246_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6542840785132085_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.031559513113182_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4342502915307875_f64,
                k: -0.7_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler025_58: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data026_59: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17586014108156545_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3598492389434165_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5609113560674_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7895121263519705_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.057886573293873_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3789149005151722_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7620212286086225_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2051554347435585_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.686801996823699_f64,
                k: -0.7_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler026_60: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data027_61: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17604066659721918_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36137563278353424_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5665572127274761_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.804689665529783_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0924902943683852_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4499247992499797_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8953714382113815_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.432322994924867_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.031457349674674_f64,
                k: -0.7_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler027_62: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data028_63: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17622185963747933_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3629253936043526_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5724089297015001_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8209308471318263_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1312609022179871_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5345768067715795_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.066884744593442_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.748344453755124_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.540840877178856_f64,
                k: -0.7_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler028_64: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data029_65: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17640372461994805_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3644991662165109_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5784802180037257_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8383748096839259_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1751669030061143_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.63818518991736_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3002065924302197_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.2337600665337862_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.404240572907696_f64,
                k: -0.7_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler029_66: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data030_67: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.176586266004788_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36609762156017206_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5847861518784241_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8571886287829185_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2255385617397643_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7696521899992939_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6476314987883502_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.137343490289808_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 6.3796094177887746_f64,
                k: -0.7_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler030_68: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data031_69: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1748515436298836_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3516050986554432_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5321065257844614_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7180530466448566_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9108275903019598_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1112333229323361_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.319146119036527_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.533202210508478_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7507538029157523_f64,
                k: -0.6_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler031_70: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data032_71: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17502858548476194_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3530167315053739_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5368393247632681_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7291422858958677_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9320803671835469_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1468984688863377_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3733904977062528_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6094225663372157_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8508766487100685_f64,
                k: -0.6_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler032_72: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data033_73: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17520627248155893_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3544487393543775_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5417231055768252_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7408430028073467_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9550900152700612_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1865688084431796_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4352978868932598_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6983400371331816_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9695980282802215_f64,
                k: -0.6_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler033_74: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data034_75: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17538460882640122_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3559016513373556_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5467666115225454_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.753217094183053_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9801263780899292_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2310891277158875_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5069157924585623_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.803958359833794_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.11341544050606_f64,
                k: -0.6_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler034_76: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data035_77: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17556359876533037_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3573760167424468_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5519793377132022_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7663359162000289_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0075231136019616_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.281584207381345_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5911666941449827_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9323227566025762_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.292503642098513_f64,
                k: -0.6_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler035_78: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data036_79: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17574324658480217_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3588724060316931_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5573716182634526_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7802822731307746_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.037698977648629_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3395933991042925_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6924049626591782_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.093101185651892_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.5239007084492706_f64,
                k: -0.6_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler036_80: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data037_81: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17592355661219386_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36039141192661606_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5629547263690385_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7951529513016599_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0711886441942242_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4072952835139891_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8174863977376825_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.302992157854223_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.838872309951497_f64,
                k: -0.6_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler037_82: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data038_83: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17610453321631936_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36193365056369764_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5687409896226853_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8110619867147718_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1086886419010082_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4879048567239257_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9780310073615923_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.594154558677271_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.3029735898397155_f64,
                k: -0.6_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler038_84: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data039_85: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17628618080795252_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3634997627252101_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5747439234215191_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8281449349915816_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.151128179599828_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5864286332503075_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1958944866494527_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.0398358172574604_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.086703640926183_f64,
                k: -0.6_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler039_86: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data040_87: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17646850384035848_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36509041515134105_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5809783859626063_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8465645339616372_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1997828426963724_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7112436789225605_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.519316855367231_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.8656670488606686_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.8709993116265595_f64,
                k: -0.6_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler040_88: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data041_89: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17475385514035785_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3508186847010158_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5294286270519057_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7116472756263033_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8982452359422777_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0895506700518853_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2853005857432933_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4845545520549488_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.685750354812596_f64,
                k: -0.5_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler041_90: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data042_91: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17493071928248824_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.352224676880348_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5341192865200811_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7225639811717759_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9189958323277101_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.124054916305536_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3372938086286021_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5570024469132429_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.780303494654548_f64,
                k: -0.5_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler042_92: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data043_93: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17510822779582402_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3536509472553149_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.538959332373287_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7340809084007079_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.941454428185354_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1624120186296487_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3965823372867114_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6414308440430099_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8922947612264018_f64,
                k: -0.5_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler043_94: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data044_95: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1752863848810204_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3550980222233272_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5439574073186619_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7462587143875267_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9658827118609202_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.205431958435733_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.465107799483287_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7416018368052644_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0277924458111314_f64,
                k: -0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler044_96: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data045_97: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17546519477859268_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3565664482253168_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5491228967741132_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7591673161169005_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9926041563132821_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.254192585691867_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5456393705347609_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8631904972952076_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1962905366178065_f64,
                k: -0.5_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler045_98: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data046_99: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1756446617694151_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35805679276065394_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5544660149620003_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7728878357825901_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0220246013918972_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3101681612463965_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6422994881851025_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0152636030998816_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4136715042011945_f64,
                k: -0.5_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler046_100: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data047_101: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1758247901752274_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35956964546660036_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5599979037298419_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.787515079112099_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.054662050503522_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3754438357425935_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7615727400820127_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.213463806756524_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.709049186175356_f64,
                k: -0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler047_102: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data048_103: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17600558435914915_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3611056192672626_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5657307464113711_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.803160730842372_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.091191068813146_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.453094640638064_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9144386536785372_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.487878895823497_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.1433945297859225_f64,
                k: -0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler048_104: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data049_105: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17618704872620228_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3626653515974583_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5716778995452916_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8199575298431502_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1325112162158122_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5479055930718042_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1215243941010486_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.906940576765013_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.8750701888108066_f64,
                k: -0.5_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler049_106: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data050_107: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1763691877238418_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.364249505707407_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5778540459023143_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8380648052171653_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1798568683069752_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6678766243739607_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4282976450693483_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.6810787666126656_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.535513209602645_f64,
                k: -0.5_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler050_108: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data051_109: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17467414669441528_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3501822277248344_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5272901591750875_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7066237440734124_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8885921049760216_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0733136290471381_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2605612170157066_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4497513956433439_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6399998658645112_f64,
                k: -0.3999999999999999_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler051_110: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data052_111: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17485086590796767_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3515836641250699_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5319473167571173_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7174061552801093_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9089615777348703_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1069605483834348_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3109353428823_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5195460789903448_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7306968836847187_f64,
                k: -0.3999999999999999_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler052_112: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data053_113: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17502822886437389_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35300530062530805_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.536752595482109_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7287800642867693_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9310021901058356_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.144348727118761_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3683427764108813_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6008221459300933_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8380358826317627_f64,
                k: -0.3999999999999999_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler053_114: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data054_115: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.175206239759829_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35444766141612105_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5417145584153601_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7408051700108401_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9549695050929656_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1862627879844718_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4346501803799458_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6971744798077697_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.967792413252014_f64,
                k: -0.3999999999999999_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler054_116: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data055_117: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17538490283034375_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3559112906431995_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5468425041326453_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7535502774266829_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9811793502678063_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2337464222030734_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.512518341928922_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8140224451130311_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1289968719280026_f64,
                k: -0.3999999999999999_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler055_118: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data056_119: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17556422235224273_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3573967534176392_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5521465519503719_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7670952094204744_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.01002787615775_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2882265661384342_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6059059780051874_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9600182740224081_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.336746137317651_f64,
                k: -0.3999999999999999_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler056_120: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data057_121: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1757442026426703_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35890463689046265_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5576377397519449_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7815332422776127_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0420205885765887_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.351720523038177_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7210360970313896_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.150078051016924_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.618694020985019_f64,
                k: -0.3999999999999999_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler057_122: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data058_123: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17592484806010436_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3604355513963144_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5633281366994488_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7969742456215755_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0778155987523672_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4272018169896268_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.868437790745338_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.412867740920747_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.032707874387324_f64,
                k: -0.3999999999999999_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler058_124: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data059_125: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17610616300487833_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3619901316717198_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5692309736184242_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8135487845662435_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1182902719261825_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5192950589409022_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.067876171022398_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.8135222249879783_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.72895480021999_f64,
                k: -0.3999999999999999_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler059_126: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data060_127: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17628815191971123_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3635690381537877_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5753607944700031_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8314135585017257_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.164648159872136_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6357275034001995_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.36287875665724_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.5521010369134958_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.305553510287251_f64,
                k: -0.3999999999999999_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler060_128: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data061_129: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.174612286530001_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3496914610279842_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5256582287372631_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7028422651240854_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8814413919511117_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.061489706726052_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.242841682417422_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.425179587701593_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6080486199305126_f64,
                k: -0.29999999999999993_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler061_130: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data062_131: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17478889331392972_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3510893901832918_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5302899089611584_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7135241705237141_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9015308603240589_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0945187977283313_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.292069926838568_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4931243665896394_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6960848815118226_f64,
                k: -0.29999999999999993_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler062_132: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data063_133: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17496614335337535_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3525074593713937_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5350687500283688_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7247910662224819_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9232645153589161_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1312092060698349_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.348147315459232_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5722049569662748_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8002173372290498_f64,
                k: -0.29999999999999993_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler063_134: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data064_135: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17514404084107435_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3539461910864565_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5400032546337269_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7367019379406765_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9468934549172218_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.172327460838914_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4128880552936287_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.665901004744966_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9260216862473254_f64,
                k: -0.29999999999999993_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler064_136: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data065_137: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17532259000954434_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3554061277098369_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5451026555293892_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7493247631096506_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9727279358309311_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2188928987074241_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.488877167408594_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.779455849821919_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.082212177317553_f64,
                k: -0.29999999999999993_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler065_138: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data066_139: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1755017951315818_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.356887832516812_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.550377000101428_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7627383978989599_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.001157051883042_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.272298741405511_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5799590511080066_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9212367220124293_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.283350588193397_f64,
                k: -0.29999999999999993_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler066_140: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data067_141: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17568166052076745_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3583918907473118_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5558372474436756_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7770349809088822_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0326772113675962_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.334513998371737_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6921742922838403_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1056608968472186_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.556097552858906_f64,
                k: -0.29999999999999993_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler067_142: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data068_143: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17586219053197988_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3599189107455767_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5614953801996173_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7923230318966769_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0679345542878826_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4084400085913955_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8357382859296454_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.360419799617152_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.9562123549913872_f64,
                k: -0.29999999999999993_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler068_144: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data069_145: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1760433895619167_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3614695251741079_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5673645339377464_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8087314997900109_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.107790306986062_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4985874311132998_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0298167266724954_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.748392905498543_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.628305048456717_f64,
                k: -0.29999999999999993_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler069_146: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data070_147: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17622526204962433_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3630443923077714_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.573459147447192_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8264151292884516_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1534256210757743_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6124900353411677_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.316590551484509_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.4625619526539824_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.147951494401679_f64,
                k: -0.29999999999999993_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler070_148: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data071_149: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17456817290292806_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.34934315932086796_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5245088052944399_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7002049100984489_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8765100664996798_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0534305870298994_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2308975521670789_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4087733584990738_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5868678474541662_f64,
                k: -0.19999999999999996_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler071_150: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data072_151: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17474469953608965_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35073860234984255_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5291225871295152_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7108170155889807_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8964075852116938_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0860417038089853_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2793599255528623_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4754938544089076_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6731552050562593_f64,
                k: -0.19999999999999996_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler072_152: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data073_153: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17492186907740698_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35215414286134267_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5338828561518244_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7220096028268826_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9179308761442853_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1222602841587976_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3345489407496247_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5531225705475502_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7751816279738935_f64,
                k: -0.19999999999999996_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler073_154: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data074_155: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1750996857171516_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3535903021483563_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5387980727453708_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7338411641805973_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9413279932952403_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.162840702180144_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3982440216739438_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.645063498365364_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.89839241699671_f64,
                k: -0.19999999999999996_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler074_156: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data075_157: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17527815368535152_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.355047621342978_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5438774235321134_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7463791047180426_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9669053971417464_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2087859420184757_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4729799844168852_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.756444506459666_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.05129569266768_f64,
                k: -0.19999999999999996_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler075_158: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data076_159: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17545727725228877_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35652666242062175_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5491309054910241_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7597016120921155_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9950473740159033_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2614666007124373_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5625255355205496_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8954460255613343_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2481046259421302_f64,
                k: -0.19999999999999996_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler076_160: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data077_161: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17563706072900442_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3580280092680724_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5545694225051505_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.773900038284382_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0262441366366397_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3228192988439669_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6728005754680795_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.076158710746851_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.5148333891629315_f64,
                k: -0.19999999999999996_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler077_162: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data078_163: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17581750846781172_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3595522688202851_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.560204896594665_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7890819698853149_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0611336754143517_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3956969951058884_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.813813161220961_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.325636552887956_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.9058704854500963_f64,
                k: -0.19999999999999996_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler078_164: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data079_165: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17599862486281712_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36110007227128776_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5660503965856722_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8053752387451769_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1005662342414086_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4845340298105778_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.004333224496939_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.705285667674476_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.562216638642263_f64,
                k: -0.19999999999999996_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler079_166: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data080_167: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1761804143504495_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3626720763650293_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5721202875823774_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8229332387670448_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1457077279880385_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.596734689932568_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2856537353421724_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.4034714304613902_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.044826935620036_f64,
                k: -0.19999999999999996_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler080_168: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data081_169: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17454173353063662_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.34913506721468096_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5238255001653895_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.6986470085417702_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8736179258696486_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0487386319621685_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.223991375207876_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3993423113684051_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5747455615173558_f64,
                k: -0.09999999999999998_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler081_170: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data082_171: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17471821213559732_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3505290261001114_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5284286599025573_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7092179973116671_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8934033053586866_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0811075784236857_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2720133232666426_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4653630031861395_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6600374067558428_f64,
                k: -0.09999999999999998_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler082_172: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data083_173: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17489533344059083_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3519430570781504_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5331779074151253_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7203668161508122_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.914803722682443_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1170528708071514_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3266916802718358_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5421622241831547_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.760865611508342_f64,
                k: -0.09999999999999998_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler083_174: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data084_175: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1750731016344119_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35337768072524217_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5380816780162917_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7321516675595502_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9380654600020122_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1573218723395986_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3897859679542097_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6331009404328622_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8826015946315438_f64,
                k: -0.09999999999999998_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler084_176: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data085_177: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17525152094559704_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3548334374282598_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5431491309950545_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7446396203476686_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9634927683757044_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2029081382746343_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4638022887050806_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7432413830105224_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.033636740307676_f64,
                k: -0.09999999999999998_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler085_178: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data086_179: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17543059564292182_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35631088838721664_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5483902334643644_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7579084694608883_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9914671368672068_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2551692247937198_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5524660788146873_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.880657857083067_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.227986891296685_f64,
                k: -0.09999999999999998_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler086_180: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data087_181: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17561033003590576_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3578106166817193_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.553815856596292_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7720491048457564_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0224751740393108_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3160230906351114_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6616282844233206_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.059255566485039_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4913004919173822_f64,
                k: -0.09999999999999998_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler087_182: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data088_183: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17579072847532518_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35933322840606297_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5594378864946032_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7871685650403171_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0571501305617423_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3882948301743525_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8011785680114223_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3057268183616464_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.877191018800974_f64,
                k: -0.09999999999999998_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler088_184: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data089_185: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17597179535373417_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.360879353878315_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5652693524452644_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.803394025906124_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.096335864637446_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4763748483246868_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9896610222794102_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6806423920122024_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.5246199613295612_f64,
                k: -0.09999999999999998_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler089_186: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data090_187: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1761535351059935_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3624496489292237_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5713245759011053_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.82087808820385_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.141189434214445_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5875929286844597_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.267862298659666_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.3697528941897903_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.98628904173055_f64,
                k: -0.09999999999999998_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler090_188: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data091_189: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17453292519943295_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3490658503988659_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5235987755982988_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.6981317007977318_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8726646259971648_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0471975511965976_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2217304763960306_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3962634015954636_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5707963267948966_f64,
                k: 0_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler091_190: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data092_191: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17470938780535167_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3504593158165558_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5281984138384987_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7086891080799296_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8924131130724964_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.079487144466667_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2696086247356864_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4620562617494721_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6557647109660167_f64,
                k: 0_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler092_192: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data093_193: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17488649304197776_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35187284488675424_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5329440075014613_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7198234702182282_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9137731103025875_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1153429007215137_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3241202847784086_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5385854914338242_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7562036827601815_f64,
                k: 0_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler093_194: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data094_195: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17506424509761404_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3533069579477463_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5378439835952237_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7315928940868784_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9369903179708498_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1555098909390267_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3870184960144325_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6291980835772994_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.877460709222638_f64,
                k: 0_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler094_196: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data095_197: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17524264820030025_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.354762195138715_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.542907492354401_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7440643375710991_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9623682616255331_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2009785880262487_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4608000106167567_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7389349574753439_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0278893379868057_f64,
                k: 0_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler095_198: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data096_199: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17542170661831016_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3562391174019542_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5481444909986313_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7573154660771808_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9902875118823331_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.253102285776058_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5491761777615785_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8758359693666533_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.221441469079183_f64,
                k: 0_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler096_200: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data097_201: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1756014246606565_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35773830754879005_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5535658398644597_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7714370171515151_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.021233494054121_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3137928444460387_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6579755004159076_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0537461418295506_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4836470664490253_f64,
                k: 0_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler097_202: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data098_203: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17578180667760368_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35926037139411_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5591834131585508_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7865358485693255_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0558379029273324_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3858662544850615_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.797049117035904_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2992404490153917_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.867868604772738_f64,
                k: 0_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler098_204: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data099_205: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1759628570611887_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3608059389648423_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5650102270696786_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8027389198411693_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0949425007763358_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.473698569225342_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9848676587180696_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6726187823193546_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.5124073655203634_f64,
                k: 0_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler099_206: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data100_207: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17614458024574997_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3623756657882198_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5710605885919664_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8201985701575591_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1397014388908147_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.584595241515496_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2620531413370775_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.3587842061975066_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.967294132898051_f64,
                k: 0_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler100_208: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data101_209: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17454173353063662_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.34913506721468096_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5238255001653895_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.6986470085417702_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8736179258696486_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0487386319621685_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.223991375207876_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3993423113684051_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.574745561517356_f64,
                k: 0.10000000000000009_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler101_210: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data102_211: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17471821213559732_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3505290261001114_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5284286599025573_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7092179973116671_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8934033053586866_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0811075784236857_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2720133232666426_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4653630031861395_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6600374067558428_f64,
                k: 0.10000000000000009_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler102_212: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data103_213: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17489533344059083_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3519430570781504_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5331779074151253_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7203668161508122_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.914803722682443_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1170528708071514_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3266916802718358_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5421622241831547_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.760865611508342_f64,
                k: 0.10000000000000009_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler103_214: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data104_215: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1750731016344119_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35337768072524217_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5380816780162917_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7321516675595502_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9380654600020122_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1573218723395986_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3897859679542097_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6331009404328622_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8826015946315438_f64,
                k: 0.10000000000000009_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler104_216: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data105_217: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17525152094559704_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3548334374282598_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5431491309950545_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7446396203476686_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9634927683757044_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2029081382746343_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4638022887050806_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7432413830105224_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.033636740307676_f64,
                k: 0.10000000000000009_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler105_218: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data106_219: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17543059564292182_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35631088838721664_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5483902334643646_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7579084694608883_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9914671368672068_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2551692247937198_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5524660788146873_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.880657857083067_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.227986891296685_f64,
                k: 0.10000000000000009_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler106_220: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data107_221: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17561033003590576_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3578106166817193_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.553815856596292_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7720491048457564_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0224751740393108_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3160230906351114_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6616282844233206_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.059255566485039_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4913004919173822_f64,
                k: 0.10000000000000009_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler107_222: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data108_223: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17579072847532518_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35933322840606297_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5594378864946032_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7871685650403171_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0571501305617423_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3882948301743525_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8011785680114223_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3057268183616464_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.877191018800974_f64,
                k: 0.10000000000000009_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler108_224: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data109_225: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17597179535373417_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.360879353878315_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5652693524452644_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.803394025906124_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.096335864637446_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4763748483246868_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9896610222794102_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6806423920122024_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.5246199613295612_f64,
                k: 0.10000000000000009_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler109_226: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data110_227: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1761535351059935_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3624496489292237_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5713245759011053_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.82087808820385_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.141189434214445_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5875929286844597_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.267862298659666_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.3697528941897903_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.98628904173055_f64,
                k: 0.10000000000000009_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler110_228: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data111_229: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17456817290292806_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.34934315932086796_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5245088052944399_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7002049100984489_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8765100664996798_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0534305870298994_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2308975521670789_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4087733584990738_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5868678474541662_f64,
                k: 0.20000000000000018_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler111_230: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data112_231: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17474469953608965_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35073860234984255_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5291225871295152_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7108170155889807_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8964075852116938_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0860417038089853_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2793599255528623_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4754938544089076_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6731552050562593_f64,
                k: 0.20000000000000018_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler112_232: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data113_233: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17492186907740698_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35215414286134267_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5338828561518244_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7220096028268826_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9179308761442853_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1222602841587976_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3345489407496247_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5531225705475502_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7751816279738935_f64,
                k: 0.20000000000000018_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler113_234: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data114_235: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1750996857171516_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3535903021483563_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5387980727453708_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7338411641805973_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9413279932952403_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.162840702180144_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3982440216739438_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.645063498365364_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.89839241699671_f64,
                k: 0.20000000000000018_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler114_236: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data115_237: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17527815368535152_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.355047621342978_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5438774235321134_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7463791047180426_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9669053971417464_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2087859420184757_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4729799844168852_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.756444506459666_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0512956926676806_f64,
                k: 0.20000000000000018_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler115_238: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data116_239: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17545727725228877_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35652666242062175_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5491309054910241_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7597016120921155_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9950473740159033_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2614666007124373_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5625255355205498_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8954460255613346_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2481046259421302_f64,
                k: 0.20000000000000018_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler116_240: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data117_241: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17563706072900442_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3580280092680724_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5545694225051505_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.773900038284382_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0262441366366397_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3228192988439669_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6728005754680795_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.076158710746851_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.5148333891629315_f64,
                k: 0.20000000000000018_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler117_242: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data118_243: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17581750846781172_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3595522688202851_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.560204896594665_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.789081969885315_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0611336754143517_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3956969951058884_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.813813161220961_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.325636552887956_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.9058704854500963_f64,
                k: 0.20000000000000018_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler118_244: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data119_245: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17599862486281712_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36110007227128776_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5660503965856722_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8053752387451769_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1005662342414086_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4845340298105778_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.004333224496939_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.705285667674476_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.5622166386422633_f64,
                k: 0.20000000000000018_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler119_246: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data120_247: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1761804143504495_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3626720763650293_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5721202875823774_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8229332387670448_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1457077279880388_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.596734689932568_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2856537353421724_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.4034714304613902_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.044826935620037_f64,
                k: 0.20000000000000018_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler120_248: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data121_249: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.174612286530001_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3496914610279842_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5256582287372631_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7028422651240854_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8814413919511117_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.061489706726052_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.242841682417422_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.425179587701593_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6080486199305128_f64,
                k: 0.30000000000000004_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler121_250: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data122_251: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17478889331392972_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3510893901832918_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5302899089611584_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7135241705237141_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9015308603240589_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0945187977283313_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2920699268385683_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4931243665896394_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6960848815118226_f64,
                k: 0.30000000000000004_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler122_252: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data123_253: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17496614335337535_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3525074593713937_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5350687500283688_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7247910662224819_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9232645153589161_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1312092060698349_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.348147315459232_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.572204956966275_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.80021733722905_f64,
                k: 0.30000000000000004_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler123_254: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data124_255: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17514404084107435_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3539461910864565_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5400032546337269_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7367019379406765_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9468934549172218_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.172327460838914_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4128880552936287_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.665901004744966_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9260216862473254_f64,
                k: 0.30000000000000004_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler124_256: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data125_257: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17532259000954434_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3554061277098369_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5451026555293892_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7493247631096506_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9727279358309311_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2188928987074241_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.488877167408594_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.779455849821919_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.082212177317553_f64,
                k: 0.30000000000000004_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler125_258: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data126_259: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1755017951315818_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.356887832516812_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.550377000101428_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7627383978989599_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.001157051883042_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.272298741405511_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5799590511080066_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9212367220124293_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.283350588193397_f64,
                k: 0.30000000000000004_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler126_260: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data127_261: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17568166052076745_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3583918907473118_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5558372474436756_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7770349809088822_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0326772113675962_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.334513998371737_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6921742922838403_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1056608968472186_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.556097552858906_f64,
                k: 0.30000000000000004_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler127_262: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data128_263: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17586219053197988_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3599189107455767_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5614953801996173_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7923230318966769_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0679345542878826_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4084400085913955_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8357382859296454_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.360419799617152_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.9562123549913872_f64,
                k: 0.30000000000000004_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler128_264: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data129_265: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1760433895619167_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3614695251741079_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5673645339377464_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8087314997900109_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.107790306986062_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4985874311132998_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0298167266724954_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.748392905498543_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.628305048456717_f64,
                k: 0.30000000000000004_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler129_266: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data130_267: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17622526204962433_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3630443923077714_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.573459147447192_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8264151292884516_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1534256210757743_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6124900353411677_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.316590551484509_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.4625619526539824_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.147951494401679_f64,
                k: 0.30000000000000004_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler130_268: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data131_269: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17467414669441528_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3501822277248344_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5272901591750875_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7066237440734124_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8885921049760216_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0733136290471381_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2605612170157066_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4497513956433439_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6399998658645112_f64,
                k: 0.40000000000000013_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler131_270: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data132_271: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17485086590796767_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3515836641250699_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5319473167571173_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7174061552801093_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9089615777348703_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1069605483834348_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3109353428823_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.519546078990345_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.730696883684719_f64,
                k: 0.40000000000000013_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler132_272: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data133_273: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17502822886437389_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35300530062530805_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.536752595482109_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7287800642867693_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9310021901058357_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1443487271187611_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3683427764108813_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6008221459300933_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8380358826317627_f64,
                k: 0.40000000000000013_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler133_274: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data134_275: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.175206239759829_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35444766141612105_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5417145584153601_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7408051700108401_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9549695050929657_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1862627879844718_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4346501803799458_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.69717447980777_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.967792413252014_f64,
                k: 0.40000000000000013_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler134_276: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data135_277: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17538490283034375_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3559112906431995_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5468425041326453_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7535502774266829_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9811793502678063_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2337464222030736_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.512518341928922_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8140224451130313_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1289968719280026_f64,
                k: 0.40000000000000013_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler135_278: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data136_279: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17556422235224273_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3573967534176392_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5521465519503719_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7670952094204744_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.01002787615775_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2882265661384342_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6059059780051876_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9600182740224081_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3367461373176512_f64,
                k: 0.40000000000000013_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler136_280: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data137_281: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1757442026426703_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35890463689046265_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5576377397519449_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7815332422776127_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0420205885765887_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.351720523038177_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7210360970313896_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1500780510169246_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.618694020985019_f64,
                k: 0.40000000000000013_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler137_282: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data138_283: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17592484806010436_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3604355513963144_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5633281366994488_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7969742456215755_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0778155987523672_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4272018169896268_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8684377907453382_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4128677409207473_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.0327078743873246_f64,
                k: 0.40000000000000013_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler138_284: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data139_285: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17610616300487833_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3619901316717198_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5692309736184243_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8135487845662435_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1182902719261825_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5192950589409022_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.067876171022398_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.8135222249879788_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.72895480021999_f64,
                k: 0.40000000000000013_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler139_286: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data140_287: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17628815191971123_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3635690381537877_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5753607944700031_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8314135585017257_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.164648159872136_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6357275034001995_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3628787566572402_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.552101036913496_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.305553510287251_f64,
                k: 0.40000000000000013_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler140_288: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data141_289: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17475385514035785_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3508186847010158_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5294286270519057_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7116472756263033_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8982452359422777_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0895506700518853_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2853005857432933_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4845545520549488_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.685750354812596_f64,
                k: 0.5_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler141_290: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data142_291: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17493071928248824_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.352224676880348_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5341192865200811_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7225639811717759_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9189958323277101_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.124054916305536_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3372938086286021_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5570024469132429_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.780303494654548_f64,
                k: 0.5_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler142_292: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data143_293: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17510822779582402_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3536509472553149_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.538959332373287_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7340809084007079_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.941454428185354_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1624120186296487_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3965823372867114_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6414308440430099_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8922947612264018_f64,
                k: 0.5_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler143_294: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data144_295: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1752863848810204_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3550980222233272_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5439574073186619_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7462587143875267_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9658827118609202_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.205431958435733_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.465107799483287_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7416018368052644_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0277924458111314_f64,
                k: 0.5_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler144_296: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data145_297: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17546519477859268_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3565664482253168_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5491228967741132_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7591673161169005_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9926041563132821_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.254192585691867_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5456393705347609_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8631904972952076_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1962905366178065_f64,
                k: 0.5_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler145_298: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data146_299: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1756446617694151_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35805679276065394_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5544660149620003_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7728878357825901_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0220246013918972_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3101681612463965_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6422994881851025_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0152636030998816_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4136715042011945_f64,
                k: 0.5_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler146_300: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data147_301: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1758247901752274_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35956964546660036_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5599979037298419_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.787515079112099_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.054662050503522_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3754438357425935_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7615727400820127_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.213463806756524_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.709049186175356_f64,
                k: 0.5_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler147_302: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data148_303: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17600558435914915_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3611056192672626_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5657307464113711_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.803160730842372_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.091191068813146_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.453094640638064_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9144386536785372_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.487878895823497_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.1433945297859225_f64,
                k: 0.5_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler148_304: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data149_305: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17618704872620228_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3626653515974583_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5716778995452916_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8199575298431502_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1325112162158122_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5479055930718042_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1215243941010486_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.906940576765013_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.8750701888108066_f64,
                k: 0.5_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler149_306: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data150_307: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1763691877238418_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.364249505707407_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5778540459023143_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8380648052171653_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1798568683069752_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6678766243739607_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4282976450693483_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.6810787666126656_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.535513209602645_f64,
                k: 0.5_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler150_308: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data151_309: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1748515436298836_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3516050986554432_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5321065257844614_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7180530466448566_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9108275903019598_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1112333229323361_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.319146119036527_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.533202210508478_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7507538029157526_f64,
                k: 0.6000000000000001_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler151_310: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data152_311: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17502858548476194_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3530167315053739_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5368393247632681_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7291422858958677_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9320803671835469_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1468984688863377_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3733904977062528_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6094225663372157_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8508766487100685_f64,
                k: 0.6000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler152_312: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data153_313: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17520627248155893_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3544487393543775_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5417231055768252_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7408430028073467_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9550900152700612_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1865688084431796_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.43529788689326_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6983400371331818_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9695980282802217_f64,
                k: 0.6000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler153_314: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data154_315: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17538460882640122_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3559016513373556_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5467666115225454_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.753217094183053_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9801263780899292_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2310891277158875_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5069157924585623_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.803958359833794_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.11341544050606_f64,
                k: 0.6000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler154_316: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data155_317: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17556359876533037_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3573760167424468_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5519793377132022_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.766335916200029_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0075231136019616_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.281584207381345_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5911666941449827_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9323227566025762_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.292503642098513_f64,
                k: 0.6000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler155_318: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data156_319: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17574324658480217_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3588724060316931_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5573716182634526_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7802822731307746_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.037698977648629_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3395933991042928_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6924049626591784_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.093101185651892_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.523900708449271_f64,
                k: 0.6000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler156_320: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data157_321: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17592355661219386_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36039141192661606_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5629547263690385_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7951529513016599_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0711886441942242_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4072952835139891_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8174863977376825_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.302992157854223_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.838872309951497_f64,
                k: 0.6000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler157_322: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data158_323: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17610453321631936_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36193365056369764_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5687409896226853_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8110619867147718_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1086886419010082_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4879048567239257_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9780310073615925_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.594154558677271_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.302973589839716_f64,
                k: 0.6000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler158_324: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data159_325: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17628618080795252_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3634997627252101_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5747439234215191_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8281449349915817_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.151128179599828_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5864286332503075_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1958944866494527_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.0398358172574604_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.086703640926183_f64,
                k: 0.6000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler159_326: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data160_327: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17646850384035848_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36509041515134105_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5809783859626063_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8465645339616372_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1997828426963724_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7112436789225605_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.519316855367231_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.865667048860669_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.87099931162656_f64,
                k: 0.6000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler160_328: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data161_329: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17496737466916723_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3525468753567793_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5353674027599713_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7260379765168445_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9269829634831345_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1400447527693318_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3657668117194073_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6024686895959164_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8456939983747236_f64,
                k: 0.7000000000000002_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler161_330: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data162_331: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1751446273730092_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3539652799747045_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5401517958943398_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7373443085447773_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9488895079669705_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1772807959736322_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4231796401075834_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6841856799887471_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9541347343119564_f64,
                k: 0.7000000000000002_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler162_332: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data163_333: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17532252613350796_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3554041759680752_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5450891303336193_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7492763577771842_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9726170633793634_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2187303976209327_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4887796709222487_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7796581281839214_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0829290325820207_f64,
                k: 0.7000000000000002_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler163_334: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data164_335: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1755010751632857_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35686409576571965_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5501882731651335_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7618975949439027_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9984462343088563_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2652862989039833_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.564766680869136_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8932499694938165_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2392290510988535_f64,
                k: 0.7000000000000002_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler164_336: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data165_337: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17568027871494424_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3583455920818026_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5554588545119061_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7752812040256811_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0267241287600322_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3181380338980246_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6542840785132087_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0315595131131823_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.434250291530788_f64,
                k: 0.7000000000000002_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler165_338: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data166_339: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17586014108156545_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3598492389434165_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5609113560674_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7895121263519705_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0578865732938731_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3789149005151722_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7620212286086228_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.205155434743559_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6868019968236996_f64,
                k: 0.7000000000000002_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler166_340: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data167_341: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17604066659721918_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36137563278353424_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5665572127274761_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.804689665529783_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0924902943683852_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.44992479924998_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8953714382113818_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.432322994924867_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.0314573496746746_f64,
                k: 0.7000000000000002_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler167_342: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data168_343: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17622185963747933_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3629253936043526_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5724089297015001_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8209308471318263_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1312609022179871_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5345768067715795_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0668847445934424_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.7483444537551245_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.540840877178857_f64,
                k: 0.7000000000000002_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler168_344: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data169_345: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17640372461994805_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3644991662165109_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5784802180037258_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8383748096839259_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1751669030061143_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6381851899173603_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3002065924302197_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.233760066533787_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.404240572907697_f64,
                k: 0.7000000000000002_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler169_346: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data170_347: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.176586266004788_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36609762156017206_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5847861518784241_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8571886287829185_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2255385617397643_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.769652189999294_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6476314987883507_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.137343490289808_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 6.379609417788776_f64,
                k: 0.7000000000000002_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler170_348: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data171_349: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.175101542413389_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35365068839779396_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5392680440908455_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7358792602807037_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9477094297007117_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1789022995388236_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4323027881876012_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7069629739121677_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9953027776647294_f64,
                k: 0.8_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler171_350: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data172_351: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17527903952342144_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3550770531354855_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5441145598764355_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7474562566680438_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9704695368423856_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2183080025184605_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4943711151994405_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7972401309544201_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1172616484005085_f64,
                k: 0.8_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler172_352: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data173_353: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1754571837508642_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35652404627248163_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5491163851292091_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7596768428213118_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9951352689354377_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2622192109995993_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.565410667634774_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9029531718534984_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.26247894341868_f64,
                k: 0.8_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler173_354: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data174_355: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17563597931587369_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3579922041200513_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5542825369111132_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7726064737697736_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0220015271210958_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.311596531230267_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.6478518468813512_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.029045841420348_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4392042002725693_f64,
                k: 0.8_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler174_356: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data175_357: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17581543047866136_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35948208343061633_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5596228089370202_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7863206388923412_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0514333069550323_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3677213138838757_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.7451736773665165_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.183010042458683_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6604037035529724_f64,
                k: 0.8_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler175_358: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data176_359: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17599554153999472_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3609942624335154_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5651478617478067_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8009069762237101_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.083889162767934_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.432350665446628_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8625761085390575_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.3768757305654766_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.9478781158239746_f64,
                k: 0.8_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler176_360: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data177_361: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17617631684170665_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3625293419366623_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5708693262294516_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8164679674015097_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1199552158519064_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5079766673336394_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0082747447038165_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6315146066775523_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.341812189228805_f64,
                k: 0.8_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler177_362: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data178_363: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1763577607672122_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36408794649916976_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5767999229062414_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8331244141814281_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1603958891464856_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5982855143796213_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.196248440837182_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.987328178611187_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.9268876980046397_f64,
                k: 0.8_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler178_364: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data179_365: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17653987774203392_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36567072568046877_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5829535999655862_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8510199830917611_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2062322059736537_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.709032142091743_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4529058049405066_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.5368893360106948_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.924642205819606_f64,
                k: 0.8_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler179_366: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data180_367: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17672267223433513_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36727835537196063_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5893456936371665_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8703272347113885_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.258867611132335_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.849873190066002_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.836838129930042_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.567484419165406_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 7.2263259298637115_f64,
                k: 0.8_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler180_368: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data181_369: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17525427376115027_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35492464591297446_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5438822141615712_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7479740042353251_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9746389845196645_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.233446325452344_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5355247765594915_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8882928567775128_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2805491384227707_f64,
                k: 0.9000000000000001_f64,
                nu: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler181_370: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data182_371: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17543204932716244_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35636022898551184_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.548802788983826_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7598883477452927_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.9985330300356812_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2759958823999022_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.60511873646394_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.9941406879519474_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.429501118783489_f64,
                k: 0.9000000000000001_f64,
                nu: 0.1_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler182_372: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data183_373: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1756104732196841_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3578165994435611_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5538815090521528_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7724687412325144_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0244466254771925_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.32348240776408_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.684984896880424_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.118574904550228_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6076835743348417_f64,
                k: 0.9000000000000001_f64,
                nu: 0.2_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler183_374: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data184_375: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1757895496674622_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.35929429810867447_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5591275715424082_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7857831472202539_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0526941001131365_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.37696822345386_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.777943743291124_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.2676509341813635_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.8256506968858517_f64,
                k: 0.9000000000000001_f64,
                nu: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler184_376: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data185_377: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17596928293938452_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3607938864247282_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5645509666711561_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.7999099699786943_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.0836647913872215_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.4378726836091849_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8880446720682853_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.450584893202523_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.1000689868578624_f64,
                k: 0.9000000000000001_f64,
                nu: 0.4_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler185_378: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data186_379: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17614967734498183_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36231594750319435_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5701625698434957_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8149402591829342_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1178482279283477_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.5081455873012106_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.0213599730864_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.682246701292683_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.4591069002104686_f64,
                k: 0.9000000000000001_f64,
                nu: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler186_380: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data187_381: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17633073723493825_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3638610872349281_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5759742474471624_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8309805194850115_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1558706545698916_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.590557637941567_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.1875186010215084_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.9885767771316853_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.9549939883570238_f64,
                k: 0.9000000000000001_f64,
                nu: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler187_382: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data188_383: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17651246700160939_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3654299354735898_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5819989787767487_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8481563358735286_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.1985495623872375_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.689215813402769_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.4029722191094236_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.420108494134006_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.698548231299244_f64,
                k: 0.9000000000000001_f64,
                nu: 0.7000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler188_384: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data189_385: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.17669487107954862_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.3670231472962842_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5882509971136549_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8666171142220903_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.2469779109884802_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.8105469760531578_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 2.6989505165893752_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 4.093521326775743_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.982074081364573_f64,
                k: 0.9000000000000001_f64,
                nu: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler189_386: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
thread_local!(
    pub static data190_387: Value<Box<[testcase_ellint_3_double_]>> =
        Rc::new(RefCell::new(Box::new([
            testcase_ellint_3_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.1768779539460417_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.36864140434751286_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.5947459536681705_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 0.8865423722605666_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.3026595810616726_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 1.965363545927808_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 3.1451407527189468_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 5.374523068031613_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_3_double_ {
                f0: 8.994256203185872_f64,
                k: 0.9000000000000001_f64,
                nu: 0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ])));
);
thread_local!(
    pub static toler190_388: Value<()> = Rc::new(RefCell::new(2.500000000000002e-13_f64));
);
pub fn test_389(data: Ptr<testcase_ellint_3_double_>, mut toler: ()) {
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
        test_389(
            data001_9.with(|v| v.as_pointer()),
            toler001_10.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data002_11.with(|v| v.as_pointer()),
            toler002_12.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data003_13.with(|v| v.as_pointer()),
            toler003_14.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data004_15.with(|v| v.as_pointer()),
            toler004_16.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data005_17.with(|v| v.as_pointer()),
            toler005_18.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data006_19.with(|v| v.as_pointer()),
            toler006_20.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data007_21.with(|v| v.as_pointer()),
            toler007_22.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data008_23.with(|v| v.as_pointer()),
            toler008_24.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data009_25.with(|v| v.as_pointer()),
            toler009_26.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data010_27.with(|v| v.as_pointer()),
            toler010_28.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data011_29.with(|v| v.as_pointer()),
            toler011_30.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data012_31.with(|v| v.as_pointer()),
            toler012_32.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data013_33.with(|v| v.as_pointer()),
            toler013_34.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data014_35.with(|v| v.as_pointer()),
            toler014_36.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data015_37.with(|v| v.as_pointer()),
            toler015_38.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data016_39.with(|v| v.as_pointer()),
            toler016_40.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data017_41.with(|v| v.as_pointer()),
            toler017_42.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data018_43.with(|v| v.as_pointer()),
            toler018_44.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data019_45.with(|v| v.as_pointer()),
            toler019_46.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data020_47.with(|v| v.as_pointer()),
            toler020_48.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data021_49.with(|v| v.as_pointer()),
            toler021_50.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data022_51.with(|v| v.as_pointer()),
            toler022_52.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data023_53.with(|v| v.as_pointer()),
            toler023_54.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data024_55.with(|v| v.as_pointer()),
            toler024_56.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data025_57.with(|v| v.as_pointer()),
            toler025_58.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data026_59.with(|v| v.as_pointer()),
            toler026_60.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data027_61.with(|v| v.as_pointer()),
            toler027_62.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data028_63.with(|v| v.as_pointer()),
            toler028_64.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data029_65.with(|v| v.as_pointer()),
            toler029_66.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data030_67.with(|v| v.as_pointer()),
            toler030_68.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data031_69.with(|v| v.as_pointer()),
            toler031_70.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data032_71.with(|v| v.as_pointer()),
            toler032_72.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data033_73.with(|v| v.as_pointer()),
            toler033_74.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data034_75.with(|v| v.as_pointer()),
            toler034_76.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data035_77.with(|v| v.as_pointer()),
            toler035_78.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data036_79.with(|v| v.as_pointer()),
            toler036_80.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data037_81.with(|v| v.as_pointer()),
            toler037_82.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data038_83.with(|v| v.as_pointer()),
            toler038_84.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data039_85.with(|v| v.as_pointer()),
            toler039_86.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data040_87.with(|v| v.as_pointer()),
            toler040_88.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data041_89.with(|v| v.as_pointer()),
            toler041_90.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data042_91.with(|v| v.as_pointer()),
            toler042_92.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data043_93.with(|v| v.as_pointer()),
            toler043_94.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data044_95.with(|v| v.as_pointer()),
            toler044_96.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data045_97.with(|v| v.as_pointer()),
            toler045_98.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data046_99.with(|v| v.as_pointer()),
            toler046_100.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data047_101.with(|v| v.as_pointer()),
            toler047_102.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data048_103.with(|v| v.as_pointer()),
            toler048_104.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data049_105.with(|v| v.as_pointer()),
            toler049_106.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data050_107.with(|v| v.as_pointer()),
            toler050_108.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data051_109.with(|v| v.as_pointer()),
            toler051_110.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data052_111.with(|v| v.as_pointer()),
            toler052_112.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data053_113.with(|v| v.as_pointer()),
            toler053_114.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data054_115.with(|v| v.as_pointer()),
            toler054_116.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data055_117.with(|v| v.as_pointer()),
            toler055_118.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data056_119.with(|v| v.as_pointer()),
            toler056_120.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data057_121.with(|v| v.as_pointer()),
            toler057_122.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data058_123.with(|v| v.as_pointer()),
            toler058_124.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data059_125.with(|v| v.as_pointer()),
            toler059_126.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data060_127.with(|v| v.as_pointer()),
            toler060_128.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data061_129.with(|v| v.as_pointer()),
            toler061_130.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data062_131.with(|v| v.as_pointer()),
            toler062_132.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data063_133.with(|v| v.as_pointer()),
            toler063_134.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data064_135.with(|v| v.as_pointer()),
            toler064_136.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data065_137.with(|v| v.as_pointer()),
            toler065_138.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data066_139.with(|v| v.as_pointer()),
            toler066_140.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data067_141.with(|v| v.as_pointer()),
            toler067_142.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data068_143.with(|v| v.as_pointer()),
            toler068_144.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data069_145.with(|v| v.as_pointer()),
            toler069_146.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data070_147.with(|v| v.as_pointer()),
            toler070_148.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data071_149.with(|v| v.as_pointer()),
            toler071_150.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data072_151.with(|v| v.as_pointer()),
            toler072_152.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data073_153.with(|v| v.as_pointer()),
            toler073_154.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data074_155.with(|v| v.as_pointer()),
            toler074_156.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data075_157.with(|v| v.as_pointer()),
            toler075_158.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data076_159.with(|v| v.as_pointer()),
            toler076_160.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data077_161.with(|v| v.as_pointer()),
            toler077_162.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data078_163.with(|v| v.as_pointer()),
            toler078_164.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data079_165.with(|v| v.as_pointer()),
            toler079_166.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data080_167.with(|v| v.as_pointer()),
            toler080_168.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data081_169.with(|v| v.as_pointer()),
            toler081_170.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data082_171.with(|v| v.as_pointer()),
            toler082_172.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data083_173.with(|v| v.as_pointer()),
            toler083_174.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data084_175.with(|v| v.as_pointer()),
            toler084_176.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data085_177.with(|v| v.as_pointer()),
            toler085_178.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data086_179.with(|v| v.as_pointer()),
            toler086_180.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data087_181.with(|v| v.as_pointer()),
            toler087_182.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data088_183.with(|v| v.as_pointer()),
            toler088_184.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data089_185.with(|v| v.as_pointer()),
            toler089_186.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data090_187.with(|v| v.as_pointer()),
            toler090_188.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data091_189.with(|v| v.as_pointer()),
            toler091_190.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data092_191.with(|v| v.as_pointer()),
            toler092_192.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data093_193.with(|v| v.as_pointer()),
            toler093_194.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data094_195.with(|v| v.as_pointer()),
            toler094_196.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data095_197.with(|v| v.as_pointer()),
            toler095_198.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data096_199.with(|v| v.as_pointer()),
            toler096_200.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data097_201.with(|v| v.as_pointer()),
            toler097_202.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data098_203.with(|v| v.as_pointer()),
            toler098_204.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data099_205.with(|v| v.as_pointer()),
            toler099_206.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data100_207.with(|v| v.as_pointer()),
            toler100_208.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data101_209.with(|v| v.as_pointer()),
            toler101_210.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data102_211.with(|v| v.as_pointer()),
            toler102_212.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data103_213.with(|v| v.as_pointer()),
            toler103_214.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data104_215.with(|v| v.as_pointer()),
            toler104_216.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data105_217.with(|v| v.as_pointer()),
            toler105_218.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data106_219.with(|v| v.as_pointer()),
            toler106_220.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data107_221.with(|v| v.as_pointer()),
            toler107_222.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data108_223.with(|v| v.as_pointer()),
            toler108_224.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data109_225.with(|v| v.as_pointer()),
            toler109_226.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data110_227.with(|v| v.as_pointer()),
            toler110_228.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data111_229.with(|v| v.as_pointer()),
            toler111_230.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data112_231.with(|v| v.as_pointer()),
            toler112_232.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data113_233.with(|v| v.as_pointer()),
            toler113_234.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data114_235.with(|v| v.as_pointer()),
            toler114_236.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data115_237.with(|v| v.as_pointer()),
            toler115_238.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data116_239.with(|v| v.as_pointer()),
            toler116_240.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data117_241.with(|v| v.as_pointer()),
            toler117_242.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data118_243.with(|v| v.as_pointer()),
            toler118_244.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data119_245.with(|v| v.as_pointer()),
            toler119_246.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data120_247.with(|v| v.as_pointer()),
            toler120_248.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data121_249.with(|v| v.as_pointer()),
            toler121_250.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data122_251.with(|v| v.as_pointer()),
            toler122_252.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data123_253.with(|v| v.as_pointer()),
            toler123_254.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data124_255.with(|v| v.as_pointer()),
            toler124_256.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data125_257.with(|v| v.as_pointer()),
            toler125_258.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data126_259.with(|v| v.as_pointer()),
            toler126_260.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data127_261.with(|v| v.as_pointer()),
            toler127_262.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data128_263.with(|v| v.as_pointer()),
            toler128_264.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data129_265.with(|v| v.as_pointer()),
            toler129_266.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data130_267.with(|v| v.as_pointer()),
            toler130_268.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data131_269.with(|v| v.as_pointer()),
            toler131_270.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data132_271.with(|v| v.as_pointer()),
            toler132_272.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data133_273.with(|v| v.as_pointer()),
            toler133_274.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data134_275.with(|v| v.as_pointer()),
            toler134_276.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data135_277.with(|v| v.as_pointer()),
            toler135_278.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data136_279.with(|v| v.as_pointer()),
            toler136_280.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data137_281.with(|v| v.as_pointer()),
            toler137_282.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data138_283.with(|v| v.as_pointer()),
            toler138_284.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data139_285.with(|v| v.as_pointer()),
            toler139_286.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data140_287.with(|v| v.as_pointer()),
            toler140_288.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data141_289.with(|v| v.as_pointer()),
            toler141_290.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data142_291.with(|v| v.as_pointer()),
            toler142_292.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data143_293.with(|v| v.as_pointer()),
            toler143_294.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data144_295.with(|v| v.as_pointer()),
            toler144_296.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data145_297.with(|v| v.as_pointer()),
            toler145_298.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data146_299.with(|v| v.as_pointer()),
            toler146_300.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data147_301.with(|v| v.as_pointer()),
            toler147_302.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data148_303.with(|v| v.as_pointer()),
            toler148_304.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data149_305.with(|v| v.as_pointer()),
            toler149_306.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data150_307.with(|v| v.as_pointer()),
            toler150_308.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data151_309.with(|v| v.as_pointer()),
            toler151_310.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data152_311.with(|v| v.as_pointer()),
            toler152_312.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data153_313.with(|v| v.as_pointer()),
            toler153_314.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data154_315.with(|v| v.as_pointer()),
            toler154_316.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data155_317.with(|v| v.as_pointer()),
            toler155_318.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data156_319.with(|v| v.as_pointer()),
            toler156_320.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data157_321.with(|v| v.as_pointer()),
            toler157_322.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data158_323.with(|v| v.as_pointer()),
            toler158_324.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data159_325.with(|v| v.as_pointer()),
            toler159_326.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data160_327.with(|v| v.as_pointer()),
            toler160_328.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data161_329.with(|v| v.as_pointer()),
            toler161_330.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data162_331.with(|v| v.as_pointer()),
            toler162_332.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data163_333.with(|v| v.as_pointer()),
            toler163_334.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data164_335.with(|v| v.as_pointer()),
            toler164_336.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data165_337.with(|v| v.as_pointer()),
            toler165_338.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data166_339.with(|v| v.as_pointer()),
            toler166_340.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data167_341.with(|v| v.as_pointer()),
            toler167_342.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data168_343.with(|v| v.as_pointer()),
            toler168_344.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data169_345.with(|v| v.as_pointer()),
            toler169_346.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data170_347.with(|v| v.as_pointer()),
            toler170_348.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data171_349.with(|v| v.as_pointer()),
            toler171_350.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data172_351.with(|v| v.as_pointer()),
            toler172_352.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data173_353.with(|v| v.as_pointer()),
            toler173_354.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data174_355.with(|v| v.as_pointer()),
            toler174_356.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data175_357.with(|v| v.as_pointer()),
            toler175_358.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data176_359.with(|v| v.as_pointer()),
            toler176_360.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data177_361.with(|v| v.as_pointer()),
            toler177_362.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data178_363.with(|v| v.as_pointer()),
            toler178_364.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data179_365.with(|v| v.as_pointer()),
            toler179_366.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data180_367.with(|v| v.as_pointer()),
            toler180_368.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data181_369.with(|v| v.as_pointer()),
            toler181_370.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data182_371.with(|v| v.as_pointer()),
            toler182_372.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data183_373.with(|v| v.as_pointer()),
            toler183_374.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data184_375.with(|v| v.as_pointer()),
            toler184_376.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data185_377.with(|v| v.as_pointer()),
            toler185_378.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data186_379.with(|v| v.as_pointer()),
            toler186_380.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data187_381.with(|v| v.as_pointer()),
            toler187_382.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data188_383.with(|v| v.as_pointer()),
            toler188_384.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data189_385.with(|v| v.as_pointer()),
            toler189_386.with(|rc| *rc.borrow()),
        )
    });
    ({
        test_389(
            data190_387.with(|v| v.as_pointer()),
            toler190_388.with(|rc| *rc.borrow()),
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
pub fn set_memory_limits_390(size: Option<()>) {
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
pub fn set_file_limit_391(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_392(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
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
pub fn run_tests_wrapped_locale_393(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
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
pub fn run_tests_wrapped_env_394(
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
pub fn test_tm_395(
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
    (*tmp_396.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_396.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_396.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_396: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
    let _ = data020_47.with(|_| ());
    let _ = toler020_48.with(|_| ());
    let _ = data021_49.with(|_| ());
    let _ = toler021_50.with(|_| ());
    let _ = data022_51.with(|_| ());
    let _ = toler022_52.with(|_| ());
    let _ = data023_53.with(|_| ());
    let _ = toler023_54.with(|_| ());
    let _ = data024_55.with(|_| ());
    let _ = toler024_56.with(|_| ());
    let _ = data025_57.with(|_| ());
    let _ = toler025_58.with(|_| ());
    let _ = data026_59.with(|_| ());
    let _ = toler026_60.with(|_| ());
    let _ = data027_61.with(|_| ());
    let _ = toler027_62.with(|_| ());
    let _ = data028_63.with(|_| ());
    let _ = toler028_64.with(|_| ());
    let _ = data029_65.with(|_| ());
    let _ = toler029_66.with(|_| ());
    let _ = data030_67.with(|_| ());
    let _ = toler030_68.with(|_| ());
    let _ = data031_69.with(|_| ());
    let _ = toler031_70.with(|_| ());
    let _ = data032_71.with(|_| ());
    let _ = toler032_72.with(|_| ());
    let _ = data033_73.with(|_| ());
    let _ = toler033_74.with(|_| ());
    let _ = data034_75.with(|_| ());
    let _ = toler034_76.with(|_| ());
    let _ = data035_77.with(|_| ());
    let _ = toler035_78.with(|_| ());
    let _ = data036_79.with(|_| ());
    let _ = toler036_80.with(|_| ());
    let _ = data037_81.with(|_| ());
    let _ = toler037_82.with(|_| ());
    let _ = data038_83.with(|_| ());
    let _ = toler038_84.with(|_| ());
    let _ = data039_85.with(|_| ());
    let _ = toler039_86.with(|_| ());
    let _ = data040_87.with(|_| ());
    let _ = toler040_88.with(|_| ());
    let _ = data041_89.with(|_| ());
    let _ = toler041_90.with(|_| ());
    let _ = data042_91.with(|_| ());
    let _ = toler042_92.with(|_| ());
    let _ = data043_93.with(|_| ());
    let _ = toler043_94.with(|_| ());
    let _ = data044_95.with(|_| ());
    let _ = toler044_96.with(|_| ());
    let _ = data045_97.with(|_| ());
    let _ = toler045_98.with(|_| ());
    let _ = data046_99.with(|_| ());
    let _ = toler046_100.with(|_| ());
    let _ = data047_101.with(|_| ());
    let _ = toler047_102.with(|_| ());
    let _ = data048_103.with(|_| ());
    let _ = toler048_104.with(|_| ());
    let _ = data049_105.with(|_| ());
    let _ = toler049_106.with(|_| ());
    let _ = data050_107.with(|_| ());
    let _ = toler050_108.with(|_| ());
    let _ = data051_109.with(|_| ());
    let _ = toler051_110.with(|_| ());
    let _ = data052_111.with(|_| ());
    let _ = toler052_112.with(|_| ());
    let _ = data053_113.with(|_| ());
    let _ = toler053_114.with(|_| ());
    let _ = data054_115.with(|_| ());
    let _ = toler054_116.with(|_| ());
    let _ = data055_117.with(|_| ());
    let _ = toler055_118.with(|_| ());
    let _ = data056_119.with(|_| ());
    let _ = toler056_120.with(|_| ());
    let _ = data057_121.with(|_| ());
    let _ = toler057_122.with(|_| ());
    let _ = data058_123.with(|_| ());
    let _ = toler058_124.with(|_| ());
    let _ = data059_125.with(|_| ());
    let _ = toler059_126.with(|_| ());
    let _ = data060_127.with(|_| ());
    let _ = toler060_128.with(|_| ());
    let _ = data061_129.with(|_| ());
    let _ = toler061_130.with(|_| ());
    let _ = data062_131.with(|_| ());
    let _ = toler062_132.with(|_| ());
    let _ = data063_133.with(|_| ());
    let _ = toler063_134.with(|_| ());
    let _ = data064_135.with(|_| ());
    let _ = toler064_136.with(|_| ());
    let _ = data065_137.with(|_| ());
    let _ = toler065_138.with(|_| ());
    let _ = data066_139.with(|_| ());
    let _ = toler066_140.with(|_| ());
    let _ = data067_141.with(|_| ());
    let _ = toler067_142.with(|_| ());
    let _ = data068_143.with(|_| ());
    let _ = toler068_144.with(|_| ());
    let _ = data069_145.with(|_| ());
    let _ = toler069_146.with(|_| ());
    let _ = data070_147.with(|_| ());
    let _ = toler070_148.with(|_| ());
    let _ = data071_149.with(|_| ());
    let _ = toler071_150.with(|_| ());
    let _ = data072_151.with(|_| ());
    let _ = toler072_152.with(|_| ());
    let _ = data073_153.with(|_| ());
    let _ = toler073_154.with(|_| ());
    let _ = data074_155.with(|_| ());
    let _ = toler074_156.with(|_| ());
    let _ = data075_157.with(|_| ());
    let _ = toler075_158.with(|_| ());
    let _ = data076_159.with(|_| ());
    let _ = toler076_160.with(|_| ());
    let _ = data077_161.with(|_| ());
    let _ = toler077_162.with(|_| ());
    let _ = data078_163.with(|_| ());
    let _ = toler078_164.with(|_| ());
    let _ = data079_165.with(|_| ());
    let _ = toler079_166.with(|_| ());
    let _ = data080_167.with(|_| ());
    let _ = toler080_168.with(|_| ());
    let _ = data081_169.with(|_| ());
    let _ = toler081_170.with(|_| ());
    let _ = data082_171.with(|_| ());
    let _ = toler082_172.with(|_| ());
    let _ = data083_173.with(|_| ());
    let _ = toler083_174.with(|_| ());
    let _ = data084_175.with(|_| ());
    let _ = toler084_176.with(|_| ());
    let _ = data085_177.with(|_| ());
    let _ = toler085_178.with(|_| ());
    let _ = data086_179.with(|_| ());
    let _ = toler086_180.with(|_| ());
    let _ = data087_181.with(|_| ());
    let _ = toler087_182.with(|_| ());
    let _ = data088_183.with(|_| ());
    let _ = toler088_184.with(|_| ());
    let _ = data089_185.with(|_| ());
    let _ = toler089_186.with(|_| ());
    let _ = data090_187.with(|_| ());
    let _ = toler090_188.with(|_| ());
    let _ = data091_189.with(|_| ());
    let _ = toler091_190.with(|_| ());
    let _ = data092_191.with(|_| ());
    let _ = toler092_192.with(|_| ());
    let _ = data093_193.with(|_| ());
    let _ = toler093_194.with(|_| ());
    let _ = data094_195.with(|_| ());
    let _ = toler094_196.with(|_| ());
    let _ = data095_197.with(|_| ());
    let _ = toler095_198.with(|_| ());
    let _ = data096_199.with(|_| ());
    let _ = toler096_200.with(|_| ());
    let _ = data097_201.with(|_| ());
    let _ = toler097_202.with(|_| ());
    let _ = data098_203.with(|_| ());
    let _ = toler098_204.with(|_| ());
    let _ = data099_205.with(|_| ());
    let _ = toler099_206.with(|_| ());
    let _ = data100_207.with(|_| ());
    let _ = toler100_208.with(|_| ());
    let _ = data101_209.with(|_| ());
    let _ = toler101_210.with(|_| ());
    let _ = data102_211.with(|_| ());
    let _ = toler102_212.with(|_| ());
    let _ = data103_213.with(|_| ());
    let _ = toler103_214.with(|_| ());
    let _ = data104_215.with(|_| ());
    let _ = toler104_216.with(|_| ());
    let _ = data105_217.with(|_| ());
    let _ = toler105_218.with(|_| ());
    let _ = data106_219.with(|_| ());
    let _ = toler106_220.with(|_| ());
    let _ = data107_221.with(|_| ());
    let _ = toler107_222.with(|_| ());
    let _ = data108_223.with(|_| ());
    let _ = toler108_224.with(|_| ());
    let _ = data109_225.with(|_| ());
    let _ = toler109_226.with(|_| ());
    let _ = data110_227.with(|_| ());
    let _ = toler110_228.with(|_| ());
    let _ = data111_229.with(|_| ());
    let _ = toler111_230.with(|_| ());
    let _ = data112_231.with(|_| ());
    let _ = toler112_232.with(|_| ());
    let _ = data113_233.with(|_| ());
    let _ = toler113_234.with(|_| ());
    let _ = data114_235.with(|_| ());
    let _ = toler114_236.with(|_| ());
    let _ = data115_237.with(|_| ());
    let _ = toler115_238.with(|_| ());
    let _ = data116_239.with(|_| ());
    let _ = toler116_240.with(|_| ());
    let _ = data117_241.with(|_| ());
    let _ = toler117_242.with(|_| ());
    let _ = data118_243.with(|_| ());
    let _ = toler118_244.with(|_| ());
    let _ = data119_245.with(|_| ());
    let _ = toler119_246.with(|_| ());
    let _ = data120_247.with(|_| ());
    let _ = toler120_248.with(|_| ());
    let _ = data121_249.with(|_| ());
    let _ = toler121_250.with(|_| ());
    let _ = data122_251.with(|_| ());
    let _ = toler122_252.with(|_| ());
    let _ = data123_253.with(|_| ());
    let _ = toler123_254.with(|_| ());
    let _ = data124_255.with(|_| ());
    let _ = toler124_256.with(|_| ());
    let _ = data125_257.with(|_| ());
    let _ = toler125_258.with(|_| ());
    let _ = data126_259.with(|_| ());
    let _ = toler126_260.with(|_| ());
    let _ = data127_261.with(|_| ());
    let _ = toler127_262.with(|_| ());
    let _ = data128_263.with(|_| ());
    let _ = toler128_264.with(|_| ());
    let _ = data129_265.with(|_| ());
    let _ = toler129_266.with(|_| ());
    let _ = data130_267.with(|_| ());
    let _ = toler130_268.with(|_| ());
    let _ = data131_269.with(|_| ());
    let _ = toler131_270.with(|_| ());
    let _ = data132_271.with(|_| ());
    let _ = toler132_272.with(|_| ());
    let _ = data133_273.with(|_| ());
    let _ = toler133_274.with(|_| ());
    let _ = data134_275.with(|_| ());
    let _ = toler134_276.with(|_| ());
    let _ = data135_277.with(|_| ());
    let _ = toler135_278.with(|_| ());
    let _ = data136_279.with(|_| ());
    let _ = toler136_280.with(|_| ());
    let _ = data137_281.with(|_| ());
    let _ = toler137_282.with(|_| ());
    let _ = data138_283.with(|_| ());
    let _ = toler138_284.with(|_| ());
    let _ = data139_285.with(|_| ());
    let _ = toler139_286.with(|_| ());
    let _ = data140_287.with(|_| ());
    let _ = toler140_288.with(|_| ());
    let _ = data141_289.with(|_| ());
    let _ = toler141_290.with(|_| ());
    let _ = data142_291.with(|_| ());
    let _ = toler142_292.with(|_| ());
    let _ = data143_293.with(|_| ());
    let _ = toler143_294.with(|_| ());
    let _ = data144_295.with(|_| ());
    let _ = toler144_296.with(|_| ());
    let _ = data145_297.with(|_| ());
    let _ = toler145_298.with(|_| ());
    let _ = data146_299.with(|_| ());
    let _ = toler146_300.with(|_| ());
    let _ = data147_301.with(|_| ());
    let _ = toler147_302.with(|_| ());
    let _ = data148_303.with(|_| ());
    let _ = toler148_304.with(|_| ());
    let _ = data149_305.with(|_| ());
    let _ = toler149_306.with(|_| ());
    let _ = data150_307.with(|_| ());
    let _ = toler150_308.with(|_| ());
    let _ = data151_309.with(|_| ());
    let _ = toler151_310.with(|_| ());
    let _ = data152_311.with(|_| ());
    let _ = toler152_312.with(|_| ());
    let _ = data153_313.with(|_| ());
    let _ = toler153_314.with(|_| ());
    let _ = data154_315.with(|_| ());
    let _ = toler154_316.with(|_| ());
    let _ = data155_317.with(|_| ());
    let _ = toler155_318.with(|_| ());
    let _ = data156_319.with(|_| ());
    let _ = toler156_320.with(|_| ());
    let _ = data157_321.with(|_| ());
    let _ = toler157_322.with(|_| ());
    let _ = data158_323.with(|_| ());
    let _ = toler158_324.with(|_| ());
    let _ = data159_325.with(|_| ());
    let _ = toler159_326.with(|_| ());
    let _ = data160_327.with(|_| ());
    let _ = toler160_328.with(|_| ());
    let _ = data161_329.with(|_| ());
    let _ = toler161_330.with(|_| ());
    let _ = data162_331.with(|_| ());
    let _ = toler162_332.with(|_| ());
    let _ = data163_333.with(|_| ());
    let _ = toler163_334.with(|_| ());
    let _ = data164_335.with(|_| ());
    let _ = toler164_336.with(|_| ());
    let _ = data165_337.with(|_| ());
    let _ = toler165_338.with(|_| ());
    let _ = data166_339.with(|_| ());
    let _ = toler166_340.with(|_| ());
    let _ = data167_341.with(|_| ());
    let _ = toler167_342.with(|_| ());
    let _ = data168_343.with(|_| ());
    let _ = toler168_344.with(|_| ());
    let _ = data169_345.with(|_| ());
    let _ = toler169_346.with(|_| ());
    let _ = data170_347.with(|_| ());
    let _ = toler170_348.with(|_| ());
    let _ = data171_349.with(|_| ());
    let _ = toler171_350.with(|_| ());
    let _ = data172_351.with(|_| ());
    let _ = toler172_352.with(|_| ());
    let _ = data173_353.with(|_| ());
    let _ = toler173_354.with(|_| ());
    let _ = data174_355.with(|_| ());
    let _ = toler174_356.with(|_| ());
    let _ = data175_357.with(|_| ());
    let _ = toler175_358.with(|_| ());
    let _ = data176_359.with(|_| ());
    let _ = toler176_360.with(|_| ());
    let _ = data177_361.with(|_| ());
    let _ = toler177_362.with(|_| ());
    let _ = data178_363.with(|_| ());
    let _ = toler178_364.with(|_| ());
    let _ = data179_365.with(|_| ());
    let _ = toler179_366.with(|_| ());
    let _ = data180_367.with(|_| ());
    let _ = toler180_368.with(|_| ());
    let _ = data181_369.with(|_| ());
    let _ = toler181_370.with(|_| ());
    let _ = data182_371.with(|_| ());
    let _ = toler182_372.with(|_| ());
    let _ = data183_373.with(|_| ());
    let _ = toler183_374.with(|_| ());
    let _ = data184_375.with(|_| ());
    let _ = toler184_376.with(|_| ());
    let _ = data185_377.with(|_| ());
    let _ = toler185_378.with(|_| ());
    let _ = data186_379.with(|_| ());
    let _ = toler186_380.with(|_| ());
    let _ = data187_381.with(|_| ());
    let _ = toler187_382.with(|_| ());
    let _ = data188_383.with(|_| ());
    let _ = toler188_384.with(|_| ());
    let _ = data189_385.with(|_| ());
    let _ = toler189_386.with(|_| ());
    let _ = data190_387.with(|_| ());
    let _ = toler190_388.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
