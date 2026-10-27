extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg)]
pub struct __gnu_test_func_callback {
    _M_size: (),
    _M_tests: [Option<unsafe fn()>; 15],
}
impl __gnu_test_func_callback {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            _M_size: 0_(),
            _M_tests: [None; 15],
        };
        this
    }
    pub unsafe fn size(&self) -> () {
        return self._M_size;
    }
    pub unsafe fn tests(&self) -> *const Option<unsafe fn()> {
        return self._M_tests.as_ptr();
    }
    pub unsafe fn push_back(&mut self, mut test: Option<unsafe fn()>) {
        self._M_tests[(self._M_size) as usize] = test;
        self._M_size.prefix_inc();
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        unsafe { __gnu_test_func_callback::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const __gnu_test_object_counter) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_test_object_counter,
    ) -> *mut __gnu_test_object_counter {
        return &mut (*(self as *mut __gnu_test_object_counter));
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_object_counter::copy_from(self as *const __gnu_test_object_counter) }
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        unsafe { __gnu_test_object_counter::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__1))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)))
        {
            (unsafe { __throw_runtime_error_3(c"copy_constructor::mark_call".as_ptr()) });
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__2)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__4))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)))
        {
            (unsafe { __throw_runtime_error_3(c"assignment_operator::mark_call".as_ptr()) });
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__4)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__5)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_6)) = 0_();
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct __gnu_test_copy_tracker {
    id_: (),
    throw_on_copy_: (),
}
impl __gnu_test_copy_tracker {
    pub unsafe fn new(mut id: Option<()>, mut throw_on_copy: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__7)).postfix_dec()
        });
        let mut throw_on_copy: () = throw_on_copy.unwrap_or_else(|| unsafe { false });
        let mut this = Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const __gnu_test_copy_tracker) -> Self {
        let mut this = Self {
            id_: (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }),
            throw_on_copy_: (*rhs).throw_on_copy_,
        };
        if this.throw_on_copy_ {
            (unsafe {
                __gnu_test_copy_constructor::throw_on(
                    (unsafe { __gnu_test_copy_constructor::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_copy_constructor::mark_call() });
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        rhs: *const __gnu_test_copy_tracker,
    ) -> *mut __gnu_test_copy_tracker {
        self.id_ = (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) });
        if (*rhs).throw_on_copy_ {
            (unsafe {
                __gnu_test_assignment_operator::throw_on(
                    (unsafe { __gnu_test_assignment_operator::count() }).wrapping_add(1_()),
                )
            });
        }
        (unsafe { __gnu_test_assignment_operator::mark_call() });
        return &mut (*(self as *mut __gnu_test_copy_tracker));
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { __gnu_test_destructor::mark_call() });
    }
    pub unsafe fn id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn reset() {
        (unsafe { __gnu_test_copy_constructor::reset() });
        (unsafe { __gnu_test_assignment_operator::reset() });
        (unsafe { __gnu_test_destructor::reset() });
    }
}
impl std::cmp::Ord for __gnu_test_copy_tracker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_8(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_8(
                other as *const __gnu_test_copy_tracker,
                self as *const __gnu_test_copy_tracker,
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
        unsafe {
            operator_eq_9(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            )
        }
    }
}
impl std::cmp::Eq for __gnu_test_copy_tracker {}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        unsafe { __gnu_test_copy_tracker::copy_from(self as *const __gnu_test_copy_tracker) }
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        unsafe { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub unsafe fn operator_eq_9(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_8(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        < (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_semaphore {
    sem_set_: (),
    pid_: ::libc::pid_t,
}
impl __gnu_test_semaphore {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct testcase_ellint_3_double_ {
    pub f0: (),
    pub k: (),
    pub nu: (),
    pub phi: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_14: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler003_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_16: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler004_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_18: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler005_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_20: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler006_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_22: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler007_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_24: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler008_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_26: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler009_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_28: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler010_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_30: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler011_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_32: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler012_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_34: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler013_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_36: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler014_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_38: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler015_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_40: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler016_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_42: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler017_43: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data018_44: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler018_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data019_46: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler019_47: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data020_48: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler020_49: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data021_50: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler021_51: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data022_52: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler022_53: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data023_54: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler023_55: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data024_56: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler024_57: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data025_58: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler025_59: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data026_60: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler026_61: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data027_62: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler027_63: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data028_64: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler028_65: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data029_66: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler029_67: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data030_68: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler030_69: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data031_70: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler031_71: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data032_72: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler032_73: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data033_74: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler033_75: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data034_76: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler034_77: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data035_78: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler035_79: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data036_80: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler036_81: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data037_82: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler037_83: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data038_84: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler038_85: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data039_86: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler039_87: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data040_88: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler040_89: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data041_90: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler041_91: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data042_92: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler042_93: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data043_94: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler043_95: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data044_96: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler044_97: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data045_98: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler045_99: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data046_100: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler046_101: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data047_102: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler047_103: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data048_104: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler048_105: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data049_106: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler049_107: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data050_108: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler050_109: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data051_110: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler051_111: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data052_112: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler052_113: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data053_114: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler053_115: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data054_116: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler054_117: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data055_118: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler055_119: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data056_120: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler056_121: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data057_122: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler057_123: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data058_124: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler058_125: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data059_126: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler059_127: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data060_128: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler060_129: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data061_130: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler061_131: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data062_132: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler062_133: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data063_134: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler063_135: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data064_136: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler064_137: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data065_138: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler065_139: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data066_140: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler066_141: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data067_142: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler067_143: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data068_144: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler068_145: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data069_146: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler069_147: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data070_148: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler070_149: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data071_150: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler071_151: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data072_152: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler072_153: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data073_154: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler073_155: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data074_156: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler074_157: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data075_158: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler075_159: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data076_160: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler076_161: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data077_162: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler077_163: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data078_164: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler078_165: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data079_166: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler079_167: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data080_168: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler080_169: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data081_170: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler081_171: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data082_172: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler082_173: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data083_174: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler083_175: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data084_176: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler084_177: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data085_178: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler085_179: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data086_180: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler086_181: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data087_182: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler087_183: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data088_184: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler088_185: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data089_186: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler089_187: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data090_188: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler090_189: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data091_190: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler091_191: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data092_192: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler092_193: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data093_194: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler093_195: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data094_196: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler094_197: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data095_198: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler095_199: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data096_200: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler096_201: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data097_202: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler097_203: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data098_204: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler098_205: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data099_206: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler099_207: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data100_208: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler100_209: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data101_210: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler101_211: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data102_212: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler102_213: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data103_214: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler103_215: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data104_216: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler104_217: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data105_218: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler105_219: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data106_220: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler106_221: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data107_222: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler107_223: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data108_224: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler108_225: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data109_226: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler109_227: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data110_228: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler110_229: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data111_230: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler111_231: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data112_232: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler112_233: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data113_234: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler113_235: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data114_236: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler114_237: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data115_238: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler115_239: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data116_240: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler116_241: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data117_242: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler117_243: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data118_244: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler118_245: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data119_246: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler119_247: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data120_248: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler120_249: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data121_250: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler121_251: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data122_252: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler122_253: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data123_254: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler123_255: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data124_256: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler124_257: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data125_258: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler125_259: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data126_260: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler126_261: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data127_262: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler127_263: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data128_264: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler128_265: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data129_266: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler129_267: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data130_268: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler130_269: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data131_270: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler131_271: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data132_272: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler132_273: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data133_274: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler133_275: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data134_276: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler134_277: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data135_278: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler135_279: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data136_280: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler136_281: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data137_282: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler137_283: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data138_284: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler138_285: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data139_286: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler139_287: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data140_288: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler140_289: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data141_290: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler141_291: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data142_292: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler142_293: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data143_294: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler143_295: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data144_296: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler144_297: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data145_298: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler145_299: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data146_300: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler146_301: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data147_302: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler147_303: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data148_304: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler148_305: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data149_306: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler149_307: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data150_308: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler150_309: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data151_310: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler151_311: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data152_312: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler152_313: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data153_314: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler153_315: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data154_316: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler154_317: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data155_318: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler155_319: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data156_320: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler156_321: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data157_322: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler157_323: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data158_324: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler158_325: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data159_326: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler159_327: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data160_328: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler160_329: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data161_330: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler161_331: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data162_332: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler162_333: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data163_334: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler163_335: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data164_336: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler164_337: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data165_338: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler165_339: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data166_340: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler166_341: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data167_342: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler167_343: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data168_344: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler168_345: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data169_346: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler169_347: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data170_348: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler170_349: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data171_350: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler171_351: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data172_352: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler172_353: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data173_354: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler173_355: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data174_356: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler174_357: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data175_358: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler175_359: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data176_360: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler176_361: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data177_362: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler177_363: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data178_364: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler178_365: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data179_366: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler179_367: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data180_368: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler180_369: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data181_370: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler181_371: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data182_372: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler182_373: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data183_374: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler183_375: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data184_376: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler184_377: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data185_378: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler185_379: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data186_380: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler186_381: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data187_382: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler187_383: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data188_384: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler188_385: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data189_386: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler189_387: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data190_388: std::cell::LazyCell<[testcase_ellint_3_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler190_389: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_390(data: *const [testcase_ellint_3_double_; 10], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 10_();
    let mut i: () = 0_();
    'loop_: while ((i) < (num_datum)) {
        let f: () = unimplemented!();
        let f0: () = (*data)[(i) as usize].f0;
        let diff: () = ((f) - (f0));
        if ((unimplemented!()) > (max_abs_diff)) {
            max_abs_diff = unimplemented!();
        }
        if ((unimplemented!()) > ((10_()) * (eps))) && ((unimplemented!()) > ((10_()) * (eps))) {
            let frac: () = ((diff) / (f0));
            if ((unimplemented!()) > (max_abs_frac)) {
                max_abs_frac = unimplemented!();
            }
        }
        i.prefix_inc();
    }
    let mut __do_while = true;
    'loop_: while __do_while || (false) {
        __do_while = false;
        if !((max_abs_frac) < (toler)) {
            (unsafe {
                __builtin_fprintf_391 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 6145_() ) .into() , ( c"void test(const testcase_ellint_3<Ret> (&)[Num], Ret) [Ret = double, Num = 10U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
            });
            unimplemented!();
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_15)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_17)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_19)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_21)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_23)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_25)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_27)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_29)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_31)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_33)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_35)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_37)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_39)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_41)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_42)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_43)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_44)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_45)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_46)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_47)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_48)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_49)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_50)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_51)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_52)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_53)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_54)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_55)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_56)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_57)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_58)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_59)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_60)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_61)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_62)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_63)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data028_64)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler028_65)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data029_66)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler029_67)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data030_68)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler030_69)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data031_70)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler031_71)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data032_72)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler032_73)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data033_74)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler033_75)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data034_76)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler034_77)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data035_78)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler035_79)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data036_80)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler036_81)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data037_82)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler037_83)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data038_84)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler038_85)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data039_86)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler039_87)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data040_88)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler040_89)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data041_90)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler041_91)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data042_92)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler042_93)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data043_94)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler043_95)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data044_96)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler044_97)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data045_98)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler045_99)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data046_100)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler046_101)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data047_102)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler047_103)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data048_104)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler048_105)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data049_106)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler049_107)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data050_108)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler050_109)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data051_110)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler051_111)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data052_112)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler052_113)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data053_114)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler053_115)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data054_116)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler054_117)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data055_118)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler055_119)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data056_120)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler056_121)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data057_122)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler057_123)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data058_124)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler058_125)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data059_126)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler059_127)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data060_128)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler060_129)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data061_130)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler061_131)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data062_132)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler062_133)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data063_134)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler063_135)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data064_136)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler064_137)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data065_138)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler065_139)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data066_140)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler066_141)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data067_142)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler067_143)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data068_144)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler068_145)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data069_146)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler069_147)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data070_148)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler070_149)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data071_150)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler071_151)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data072_152)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler072_153)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data073_154)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler073_155)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data074_156)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler074_157)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data075_158)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler075_159)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data076_160)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler076_161)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data077_162)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler077_163)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data078_164)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler078_165)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data079_166)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler079_167)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data080_168)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler080_169)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data081_170)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler081_171)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data082_172)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler082_173)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data083_174)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler083_175)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data084_176)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler084_177)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data085_178)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler085_179)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data086_180)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler086_181)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data087_182)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler087_183)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data088_184)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler088_185)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data089_186)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler089_187)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data090_188)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler090_189)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data091_190)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler091_191)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data092_192)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler092_193)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data093_194)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler093_195)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data094_196)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler094_197)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data095_198)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler095_199)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data096_200)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler096_201)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data097_202)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler097_203)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data098_204)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler098_205)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data099_206)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler099_207)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data100_208)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler100_209)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data101_210)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler101_211)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data102_212)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler102_213)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data103_214)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler103_215)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data104_216)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler104_217)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data105_218)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler105_219)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data106_220)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler106_221)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data107_222)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler107_223)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data108_224)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler108_225)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data109_226)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler109_227)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data110_228)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler110_229)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data111_230)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler111_231)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data112_232)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler112_233)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data113_234)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler113_235)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data114_236)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler114_237)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data115_238)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler115_239)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data116_240)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler116_241)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data117_242)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler117_243)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data118_244)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler118_245)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data119_246)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler119_247)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data120_248)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler120_249)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data121_250)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler121_251)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data122_252)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler122_253)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data123_254)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler123_255)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data124_256)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler124_257)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data125_258)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler125_259)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data126_260)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler126_261)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data127_262)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler127_263)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data128_264)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler128_265)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data129_266)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler129_267)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data130_268)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler130_269)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data131_270)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler131_271)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data132_272)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler132_273)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data133_274)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler133_275)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data134_276)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler134_277)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data135_278)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler135_279)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data136_280)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler136_281)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data137_282)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler137_283)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data138_284)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler138_285)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data139_286)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler139_287)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data140_288)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler140_289)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data141_290)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler141_291)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data142_292)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler142_293)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data143_294)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler143_295)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data144_296)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler144_297)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data145_298)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler145_299)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data146_300)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler146_301)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data147_302)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler147_303)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data148_304)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler148_305)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data149_306)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler149_307)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data150_308)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler150_309)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data151_310)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler151_311)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data152_312)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler152_313)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data153_314)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler153_315)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data154_316)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler154_317)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data155_318)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler155_319)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data156_320)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler156_321)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data157_322)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler157_323)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data158_324)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler158_325)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data159_326)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler159_327)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data160_328)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler160_329)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data161_330)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler161_331)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data162_332)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler162_333)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data163_334)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler163_335)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data164_336)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler164_337)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data165_338)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler165_339)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data166_340)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler166_341)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data167_342)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler167_343)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data168_344)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler168_345)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data169_346)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler169_347)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data170_348)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler170_349)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data171_350)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler171_351)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data172_352)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler172_353)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data173_354)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler173_355)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data174_356)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler174_357)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data175_358)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler175_359)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data176_360)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler176_361)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data177_362)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler177_363)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data178_364)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler178_365)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data179_366)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler179_367)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data180_368)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler180_369)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data181_370)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler181_371)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data182_372)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler182_373)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data183_374)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler183_375)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data184_376)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler184_377)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data185_378)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler185_379)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data186_380)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler186_381)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data187_382)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler187_383)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data188_384)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler188_385)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data189_386)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler189_387)),
        )
    });
    (unsafe {
        test_390(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data190_388)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler190_389)),
        )
    });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: 0 as ::libc::pid_t,
        };
        this.pid_ = ((unsafe { getpid_392() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_393(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_394(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) })
            == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_395(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_396(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_397(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_396(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_397(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_396((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_397(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_398(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_396(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_397(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_399(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_400(
            mangled,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            (&mut status as *mut ()),
        )
    });
    if !(demangled).is_null() {
        s = (demangled).cast_const();
    } else {
        switch!(match status {
            __v if __v == 0_() => {
                s = c"error code = 0: success".as_ptr();
                break;
            }
            __v if __v == -1_() => {
                s = c"error code = -1: memory allocation failure".as_ptr();
                break;
            }
            __v if __v == -2_() => {
                s = c"error code = -2: invalid mangled name".as_ptr();
                break;
            }
            __v if __v == -3_() => {
                s = c"error code = -3: invalid arguments".as_ptr();
                break;
            }
            _ => {
                s = c"error code unknown - who knows what happened".as_ptr();
            }
        });
    }
    let mut w: () = unimplemented!();
    if unimplemented!() {
        (unsafe { __throw_runtime_error_3(s) });
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_401(
    mut name: *const (),
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut res: *const () = (unimplemented!()).cast_const();
    if !(res).is_null() {
        let mut preLC_ALL: () = unimplemented!();
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                (unsafe {
                    __builtin_fprintf_391 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
                });
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub unsafe fn run_tests_wrapped_env_402(
    mut name: *const (),
    mut env: *const (),
    l: *const __gnu_test_func_callback,
) {
    let mut loc_name: () = unimplemented!();
    let mut orig: () = unimplemented!();
    let mut oldENV: *const () = (unimplemented!()).cast_const();
    if !(unimplemented!() != 0) {
        let mut tests: *const Option<unsafe fn()> =
            (unsafe { __gnu_test_func_callback::tests(&(*l)) });
        let mut i: () = 0_();
        'loop_: while ((i) < (unsafe { __gnu_test_func_callback::size(&(*l)) })) {
            (unsafe { (*tests.offset((i) as isize)).unwrap()() });
            i.prefix_inc();
        }
        unimplemented!();
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { __throw_runtime_error_3(unimplemented!()) });
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__7: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: (),
    pub buf: *mut semid_ds,
    pub array: *mut (),
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl __gnu_test_semaphore {}
impl __gnu_test_semaphore {
    pub unsafe fn destructor(&mut self) {
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((self.pid_) == (unsafe { getpid_392() })) {
            (unsafe { semctl_394(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_403(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_403(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_404(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_405));
}
static mut tmp_405: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data001_10);
    std::cell::LazyCell::force(&*&raw const toler001_11);
    std::cell::LazyCell::force(&*&raw const data002_12);
    std::cell::LazyCell::force(&*&raw const toler002_13);
    std::cell::LazyCell::force(&*&raw const data003_14);
    std::cell::LazyCell::force(&*&raw const toler003_15);
    std::cell::LazyCell::force(&*&raw const data004_16);
    std::cell::LazyCell::force(&*&raw const toler004_17);
    std::cell::LazyCell::force(&*&raw const data005_18);
    std::cell::LazyCell::force(&*&raw const toler005_19);
    std::cell::LazyCell::force(&*&raw const data006_20);
    std::cell::LazyCell::force(&*&raw const toler006_21);
    std::cell::LazyCell::force(&*&raw const data007_22);
    std::cell::LazyCell::force(&*&raw const toler007_23);
    std::cell::LazyCell::force(&*&raw const data008_24);
    std::cell::LazyCell::force(&*&raw const toler008_25);
    std::cell::LazyCell::force(&*&raw const data009_26);
    std::cell::LazyCell::force(&*&raw const toler009_27);
    std::cell::LazyCell::force(&*&raw const data010_28);
    std::cell::LazyCell::force(&*&raw const toler010_29);
    std::cell::LazyCell::force(&*&raw const data011_30);
    std::cell::LazyCell::force(&*&raw const toler011_31);
    std::cell::LazyCell::force(&*&raw const data012_32);
    std::cell::LazyCell::force(&*&raw const toler012_33);
    std::cell::LazyCell::force(&*&raw const data013_34);
    std::cell::LazyCell::force(&*&raw const toler013_35);
    std::cell::LazyCell::force(&*&raw const data014_36);
    std::cell::LazyCell::force(&*&raw const toler014_37);
    std::cell::LazyCell::force(&*&raw const data015_38);
    std::cell::LazyCell::force(&*&raw const toler015_39);
    std::cell::LazyCell::force(&*&raw const data016_40);
    std::cell::LazyCell::force(&*&raw const toler016_41);
    std::cell::LazyCell::force(&*&raw const data017_42);
    std::cell::LazyCell::force(&*&raw const toler017_43);
    std::cell::LazyCell::force(&*&raw const data018_44);
    std::cell::LazyCell::force(&*&raw const toler018_45);
    std::cell::LazyCell::force(&*&raw const data019_46);
    std::cell::LazyCell::force(&*&raw const toler019_47);
    std::cell::LazyCell::force(&*&raw const data020_48);
    std::cell::LazyCell::force(&*&raw const toler020_49);
    std::cell::LazyCell::force(&*&raw const data021_50);
    std::cell::LazyCell::force(&*&raw const toler021_51);
    std::cell::LazyCell::force(&*&raw const data022_52);
    std::cell::LazyCell::force(&*&raw const toler022_53);
    std::cell::LazyCell::force(&*&raw const data023_54);
    std::cell::LazyCell::force(&*&raw const toler023_55);
    std::cell::LazyCell::force(&*&raw const data024_56);
    std::cell::LazyCell::force(&*&raw const toler024_57);
    std::cell::LazyCell::force(&*&raw const data025_58);
    std::cell::LazyCell::force(&*&raw const toler025_59);
    std::cell::LazyCell::force(&*&raw const data026_60);
    std::cell::LazyCell::force(&*&raw const toler026_61);
    std::cell::LazyCell::force(&*&raw const data027_62);
    std::cell::LazyCell::force(&*&raw const toler027_63);
    std::cell::LazyCell::force(&*&raw const data028_64);
    std::cell::LazyCell::force(&*&raw const toler028_65);
    std::cell::LazyCell::force(&*&raw const data029_66);
    std::cell::LazyCell::force(&*&raw const toler029_67);
    std::cell::LazyCell::force(&*&raw const data030_68);
    std::cell::LazyCell::force(&*&raw const toler030_69);
    std::cell::LazyCell::force(&*&raw const data031_70);
    std::cell::LazyCell::force(&*&raw const toler031_71);
    std::cell::LazyCell::force(&*&raw const data032_72);
    std::cell::LazyCell::force(&*&raw const toler032_73);
    std::cell::LazyCell::force(&*&raw const data033_74);
    std::cell::LazyCell::force(&*&raw const toler033_75);
    std::cell::LazyCell::force(&*&raw const data034_76);
    std::cell::LazyCell::force(&*&raw const toler034_77);
    std::cell::LazyCell::force(&*&raw const data035_78);
    std::cell::LazyCell::force(&*&raw const toler035_79);
    std::cell::LazyCell::force(&*&raw const data036_80);
    std::cell::LazyCell::force(&*&raw const toler036_81);
    std::cell::LazyCell::force(&*&raw const data037_82);
    std::cell::LazyCell::force(&*&raw const toler037_83);
    std::cell::LazyCell::force(&*&raw const data038_84);
    std::cell::LazyCell::force(&*&raw const toler038_85);
    std::cell::LazyCell::force(&*&raw const data039_86);
    std::cell::LazyCell::force(&*&raw const toler039_87);
    std::cell::LazyCell::force(&*&raw const data040_88);
    std::cell::LazyCell::force(&*&raw const toler040_89);
    std::cell::LazyCell::force(&*&raw const data041_90);
    std::cell::LazyCell::force(&*&raw const toler041_91);
    std::cell::LazyCell::force(&*&raw const data042_92);
    std::cell::LazyCell::force(&*&raw const toler042_93);
    std::cell::LazyCell::force(&*&raw const data043_94);
    std::cell::LazyCell::force(&*&raw const toler043_95);
    std::cell::LazyCell::force(&*&raw const data044_96);
    std::cell::LazyCell::force(&*&raw const toler044_97);
    std::cell::LazyCell::force(&*&raw const data045_98);
    std::cell::LazyCell::force(&*&raw const toler045_99);
    std::cell::LazyCell::force(&*&raw const data046_100);
    std::cell::LazyCell::force(&*&raw const toler046_101);
    std::cell::LazyCell::force(&*&raw const data047_102);
    std::cell::LazyCell::force(&*&raw const toler047_103);
    std::cell::LazyCell::force(&*&raw const data048_104);
    std::cell::LazyCell::force(&*&raw const toler048_105);
    std::cell::LazyCell::force(&*&raw const data049_106);
    std::cell::LazyCell::force(&*&raw const toler049_107);
    std::cell::LazyCell::force(&*&raw const data050_108);
    std::cell::LazyCell::force(&*&raw const toler050_109);
    std::cell::LazyCell::force(&*&raw const data051_110);
    std::cell::LazyCell::force(&*&raw const toler051_111);
    std::cell::LazyCell::force(&*&raw const data052_112);
    std::cell::LazyCell::force(&*&raw const toler052_113);
    std::cell::LazyCell::force(&*&raw const data053_114);
    std::cell::LazyCell::force(&*&raw const toler053_115);
    std::cell::LazyCell::force(&*&raw const data054_116);
    std::cell::LazyCell::force(&*&raw const toler054_117);
    std::cell::LazyCell::force(&*&raw const data055_118);
    std::cell::LazyCell::force(&*&raw const toler055_119);
    std::cell::LazyCell::force(&*&raw const data056_120);
    std::cell::LazyCell::force(&*&raw const toler056_121);
    std::cell::LazyCell::force(&*&raw const data057_122);
    std::cell::LazyCell::force(&*&raw const toler057_123);
    std::cell::LazyCell::force(&*&raw const data058_124);
    std::cell::LazyCell::force(&*&raw const toler058_125);
    std::cell::LazyCell::force(&*&raw const data059_126);
    std::cell::LazyCell::force(&*&raw const toler059_127);
    std::cell::LazyCell::force(&*&raw const data060_128);
    std::cell::LazyCell::force(&*&raw const toler060_129);
    std::cell::LazyCell::force(&*&raw const data061_130);
    std::cell::LazyCell::force(&*&raw const toler061_131);
    std::cell::LazyCell::force(&*&raw const data062_132);
    std::cell::LazyCell::force(&*&raw const toler062_133);
    std::cell::LazyCell::force(&*&raw const data063_134);
    std::cell::LazyCell::force(&*&raw const toler063_135);
    std::cell::LazyCell::force(&*&raw const data064_136);
    std::cell::LazyCell::force(&*&raw const toler064_137);
    std::cell::LazyCell::force(&*&raw const data065_138);
    std::cell::LazyCell::force(&*&raw const toler065_139);
    std::cell::LazyCell::force(&*&raw const data066_140);
    std::cell::LazyCell::force(&*&raw const toler066_141);
    std::cell::LazyCell::force(&*&raw const data067_142);
    std::cell::LazyCell::force(&*&raw const toler067_143);
    std::cell::LazyCell::force(&*&raw const data068_144);
    std::cell::LazyCell::force(&*&raw const toler068_145);
    std::cell::LazyCell::force(&*&raw const data069_146);
    std::cell::LazyCell::force(&*&raw const toler069_147);
    std::cell::LazyCell::force(&*&raw const data070_148);
    std::cell::LazyCell::force(&*&raw const toler070_149);
    std::cell::LazyCell::force(&*&raw const data071_150);
    std::cell::LazyCell::force(&*&raw const toler071_151);
    std::cell::LazyCell::force(&*&raw const data072_152);
    std::cell::LazyCell::force(&*&raw const toler072_153);
    std::cell::LazyCell::force(&*&raw const data073_154);
    std::cell::LazyCell::force(&*&raw const toler073_155);
    std::cell::LazyCell::force(&*&raw const data074_156);
    std::cell::LazyCell::force(&*&raw const toler074_157);
    std::cell::LazyCell::force(&*&raw const data075_158);
    std::cell::LazyCell::force(&*&raw const toler075_159);
    std::cell::LazyCell::force(&*&raw const data076_160);
    std::cell::LazyCell::force(&*&raw const toler076_161);
    std::cell::LazyCell::force(&*&raw const data077_162);
    std::cell::LazyCell::force(&*&raw const toler077_163);
    std::cell::LazyCell::force(&*&raw const data078_164);
    std::cell::LazyCell::force(&*&raw const toler078_165);
    std::cell::LazyCell::force(&*&raw const data079_166);
    std::cell::LazyCell::force(&*&raw const toler079_167);
    std::cell::LazyCell::force(&*&raw const data080_168);
    std::cell::LazyCell::force(&*&raw const toler080_169);
    std::cell::LazyCell::force(&*&raw const data081_170);
    std::cell::LazyCell::force(&*&raw const toler081_171);
    std::cell::LazyCell::force(&*&raw const data082_172);
    std::cell::LazyCell::force(&*&raw const toler082_173);
    std::cell::LazyCell::force(&*&raw const data083_174);
    std::cell::LazyCell::force(&*&raw const toler083_175);
    std::cell::LazyCell::force(&*&raw const data084_176);
    std::cell::LazyCell::force(&*&raw const toler084_177);
    std::cell::LazyCell::force(&*&raw const data085_178);
    std::cell::LazyCell::force(&*&raw const toler085_179);
    std::cell::LazyCell::force(&*&raw const data086_180);
    std::cell::LazyCell::force(&*&raw const toler086_181);
    std::cell::LazyCell::force(&*&raw const data087_182);
    std::cell::LazyCell::force(&*&raw const toler087_183);
    std::cell::LazyCell::force(&*&raw const data088_184);
    std::cell::LazyCell::force(&*&raw const toler088_185);
    std::cell::LazyCell::force(&*&raw const data089_186);
    std::cell::LazyCell::force(&*&raw const toler089_187);
    std::cell::LazyCell::force(&*&raw const data090_188);
    std::cell::LazyCell::force(&*&raw const toler090_189);
    std::cell::LazyCell::force(&*&raw const data091_190);
    std::cell::LazyCell::force(&*&raw const toler091_191);
    std::cell::LazyCell::force(&*&raw const data092_192);
    std::cell::LazyCell::force(&*&raw const toler092_193);
    std::cell::LazyCell::force(&*&raw const data093_194);
    std::cell::LazyCell::force(&*&raw const toler093_195);
    std::cell::LazyCell::force(&*&raw const data094_196);
    std::cell::LazyCell::force(&*&raw const toler094_197);
    std::cell::LazyCell::force(&*&raw const data095_198);
    std::cell::LazyCell::force(&*&raw const toler095_199);
    std::cell::LazyCell::force(&*&raw const data096_200);
    std::cell::LazyCell::force(&*&raw const toler096_201);
    std::cell::LazyCell::force(&*&raw const data097_202);
    std::cell::LazyCell::force(&*&raw const toler097_203);
    std::cell::LazyCell::force(&*&raw const data098_204);
    std::cell::LazyCell::force(&*&raw const toler098_205);
    std::cell::LazyCell::force(&*&raw const data099_206);
    std::cell::LazyCell::force(&*&raw const toler099_207);
    std::cell::LazyCell::force(&*&raw const data100_208);
    std::cell::LazyCell::force(&*&raw const toler100_209);
    std::cell::LazyCell::force(&*&raw const data101_210);
    std::cell::LazyCell::force(&*&raw const toler101_211);
    std::cell::LazyCell::force(&*&raw const data102_212);
    std::cell::LazyCell::force(&*&raw const toler102_213);
    std::cell::LazyCell::force(&*&raw const data103_214);
    std::cell::LazyCell::force(&*&raw const toler103_215);
    std::cell::LazyCell::force(&*&raw const data104_216);
    std::cell::LazyCell::force(&*&raw const toler104_217);
    std::cell::LazyCell::force(&*&raw const data105_218);
    std::cell::LazyCell::force(&*&raw const toler105_219);
    std::cell::LazyCell::force(&*&raw const data106_220);
    std::cell::LazyCell::force(&*&raw const toler106_221);
    std::cell::LazyCell::force(&*&raw const data107_222);
    std::cell::LazyCell::force(&*&raw const toler107_223);
    std::cell::LazyCell::force(&*&raw const data108_224);
    std::cell::LazyCell::force(&*&raw const toler108_225);
    std::cell::LazyCell::force(&*&raw const data109_226);
    std::cell::LazyCell::force(&*&raw const toler109_227);
    std::cell::LazyCell::force(&*&raw const data110_228);
    std::cell::LazyCell::force(&*&raw const toler110_229);
    std::cell::LazyCell::force(&*&raw const data111_230);
    std::cell::LazyCell::force(&*&raw const toler111_231);
    std::cell::LazyCell::force(&*&raw const data112_232);
    std::cell::LazyCell::force(&*&raw const toler112_233);
    std::cell::LazyCell::force(&*&raw const data113_234);
    std::cell::LazyCell::force(&*&raw const toler113_235);
    std::cell::LazyCell::force(&*&raw const data114_236);
    std::cell::LazyCell::force(&*&raw const toler114_237);
    std::cell::LazyCell::force(&*&raw const data115_238);
    std::cell::LazyCell::force(&*&raw const toler115_239);
    std::cell::LazyCell::force(&*&raw const data116_240);
    std::cell::LazyCell::force(&*&raw const toler116_241);
    std::cell::LazyCell::force(&*&raw const data117_242);
    std::cell::LazyCell::force(&*&raw const toler117_243);
    std::cell::LazyCell::force(&*&raw const data118_244);
    std::cell::LazyCell::force(&*&raw const toler118_245);
    std::cell::LazyCell::force(&*&raw const data119_246);
    std::cell::LazyCell::force(&*&raw const toler119_247);
    std::cell::LazyCell::force(&*&raw const data120_248);
    std::cell::LazyCell::force(&*&raw const toler120_249);
    std::cell::LazyCell::force(&*&raw const data121_250);
    std::cell::LazyCell::force(&*&raw const toler121_251);
    std::cell::LazyCell::force(&*&raw const data122_252);
    std::cell::LazyCell::force(&*&raw const toler122_253);
    std::cell::LazyCell::force(&*&raw const data123_254);
    std::cell::LazyCell::force(&*&raw const toler123_255);
    std::cell::LazyCell::force(&*&raw const data124_256);
    std::cell::LazyCell::force(&*&raw const toler124_257);
    std::cell::LazyCell::force(&*&raw const data125_258);
    std::cell::LazyCell::force(&*&raw const toler125_259);
    std::cell::LazyCell::force(&*&raw const data126_260);
    std::cell::LazyCell::force(&*&raw const toler126_261);
    std::cell::LazyCell::force(&*&raw const data127_262);
    std::cell::LazyCell::force(&*&raw const toler127_263);
    std::cell::LazyCell::force(&*&raw const data128_264);
    std::cell::LazyCell::force(&*&raw const toler128_265);
    std::cell::LazyCell::force(&*&raw const data129_266);
    std::cell::LazyCell::force(&*&raw const toler129_267);
    std::cell::LazyCell::force(&*&raw const data130_268);
    std::cell::LazyCell::force(&*&raw const toler130_269);
    std::cell::LazyCell::force(&*&raw const data131_270);
    std::cell::LazyCell::force(&*&raw const toler131_271);
    std::cell::LazyCell::force(&*&raw const data132_272);
    std::cell::LazyCell::force(&*&raw const toler132_273);
    std::cell::LazyCell::force(&*&raw const data133_274);
    std::cell::LazyCell::force(&*&raw const toler133_275);
    std::cell::LazyCell::force(&*&raw const data134_276);
    std::cell::LazyCell::force(&*&raw const toler134_277);
    std::cell::LazyCell::force(&*&raw const data135_278);
    std::cell::LazyCell::force(&*&raw const toler135_279);
    std::cell::LazyCell::force(&*&raw const data136_280);
    std::cell::LazyCell::force(&*&raw const toler136_281);
    std::cell::LazyCell::force(&*&raw const data137_282);
    std::cell::LazyCell::force(&*&raw const toler137_283);
    std::cell::LazyCell::force(&*&raw const data138_284);
    std::cell::LazyCell::force(&*&raw const toler138_285);
    std::cell::LazyCell::force(&*&raw const data139_286);
    std::cell::LazyCell::force(&*&raw const toler139_287);
    std::cell::LazyCell::force(&*&raw const data140_288);
    std::cell::LazyCell::force(&*&raw const toler140_289);
    std::cell::LazyCell::force(&*&raw const data141_290);
    std::cell::LazyCell::force(&*&raw const toler141_291);
    std::cell::LazyCell::force(&*&raw const data142_292);
    std::cell::LazyCell::force(&*&raw const toler142_293);
    std::cell::LazyCell::force(&*&raw const data143_294);
    std::cell::LazyCell::force(&*&raw const toler143_295);
    std::cell::LazyCell::force(&*&raw const data144_296);
    std::cell::LazyCell::force(&*&raw const toler144_297);
    std::cell::LazyCell::force(&*&raw const data145_298);
    std::cell::LazyCell::force(&*&raw const toler145_299);
    std::cell::LazyCell::force(&*&raw const data146_300);
    std::cell::LazyCell::force(&*&raw const toler146_301);
    std::cell::LazyCell::force(&*&raw const data147_302);
    std::cell::LazyCell::force(&*&raw const toler147_303);
    std::cell::LazyCell::force(&*&raw const data148_304);
    std::cell::LazyCell::force(&*&raw const toler148_305);
    std::cell::LazyCell::force(&*&raw const data149_306);
    std::cell::LazyCell::force(&*&raw const toler149_307);
    std::cell::LazyCell::force(&*&raw const data150_308);
    std::cell::LazyCell::force(&*&raw const toler150_309);
    std::cell::LazyCell::force(&*&raw const data151_310);
    std::cell::LazyCell::force(&*&raw const toler151_311);
    std::cell::LazyCell::force(&*&raw const data152_312);
    std::cell::LazyCell::force(&*&raw const toler152_313);
    std::cell::LazyCell::force(&*&raw const data153_314);
    std::cell::LazyCell::force(&*&raw const toler153_315);
    std::cell::LazyCell::force(&*&raw const data154_316);
    std::cell::LazyCell::force(&*&raw const toler154_317);
    std::cell::LazyCell::force(&*&raw const data155_318);
    std::cell::LazyCell::force(&*&raw const toler155_319);
    std::cell::LazyCell::force(&*&raw const data156_320);
    std::cell::LazyCell::force(&*&raw const toler156_321);
    std::cell::LazyCell::force(&*&raw const data157_322);
    std::cell::LazyCell::force(&*&raw const toler157_323);
    std::cell::LazyCell::force(&*&raw const data158_324);
    std::cell::LazyCell::force(&*&raw const toler158_325);
    std::cell::LazyCell::force(&*&raw const data159_326);
    std::cell::LazyCell::force(&*&raw const toler159_327);
    std::cell::LazyCell::force(&*&raw const data160_328);
    std::cell::LazyCell::force(&*&raw const toler160_329);
    std::cell::LazyCell::force(&*&raw const data161_330);
    std::cell::LazyCell::force(&*&raw const toler161_331);
    std::cell::LazyCell::force(&*&raw const data162_332);
    std::cell::LazyCell::force(&*&raw const toler162_333);
    std::cell::LazyCell::force(&*&raw const data163_334);
    std::cell::LazyCell::force(&*&raw const toler163_335);
    std::cell::LazyCell::force(&*&raw const data164_336);
    std::cell::LazyCell::force(&*&raw const toler164_337);
    std::cell::LazyCell::force(&*&raw const data165_338);
    std::cell::LazyCell::force(&*&raw const toler165_339);
    std::cell::LazyCell::force(&*&raw const data166_340);
    std::cell::LazyCell::force(&*&raw const toler166_341);
    std::cell::LazyCell::force(&*&raw const data167_342);
    std::cell::LazyCell::force(&*&raw const toler167_343);
    std::cell::LazyCell::force(&*&raw const data168_344);
    std::cell::LazyCell::force(&*&raw const toler168_345);
    std::cell::LazyCell::force(&*&raw const data169_346);
    std::cell::LazyCell::force(&*&raw const toler169_347);
    std::cell::LazyCell::force(&*&raw const data170_348);
    std::cell::LazyCell::force(&*&raw const toler170_349);
    std::cell::LazyCell::force(&*&raw const data171_350);
    std::cell::LazyCell::force(&*&raw const toler171_351);
    std::cell::LazyCell::force(&*&raw const data172_352);
    std::cell::LazyCell::force(&*&raw const toler172_353);
    std::cell::LazyCell::force(&*&raw const data173_354);
    std::cell::LazyCell::force(&*&raw const toler173_355);
    std::cell::LazyCell::force(&*&raw const data174_356);
    std::cell::LazyCell::force(&*&raw const toler174_357);
    std::cell::LazyCell::force(&*&raw const data175_358);
    std::cell::LazyCell::force(&*&raw const toler175_359);
    std::cell::LazyCell::force(&*&raw const data176_360);
    std::cell::LazyCell::force(&*&raw const toler176_361);
    std::cell::LazyCell::force(&*&raw const data177_362);
    std::cell::LazyCell::force(&*&raw const toler177_363);
    std::cell::LazyCell::force(&*&raw const data178_364);
    std::cell::LazyCell::force(&*&raw const toler178_365);
    std::cell::LazyCell::force(&*&raw const data179_366);
    std::cell::LazyCell::force(&*&raw const toler179_367);
    std::cell::LazyCell::force(&*&raw const data180_368);
    std::cell::LazyCell::force(&*&raw const toler180_369);
    std::cell::LazyCell::force(&*&raw const data181_370);
    std::cell::LazyCell::force(&*&raw const toler181_371);
    std::cell::LazyCell::force(&*&raw const data182_372);
    std::cell::LazyCell::force(&*&raw const toler182_373);
    std::cell::LazyCell::force(&*&raw const data183_374);
    std::cell::LazyCell::force(&*&raw const toler183_375);
    std::cell::LazyCell::force(&*&raw const data184_376);
    std::cell::LazyCell::force(&*&raw const toler184_377);
    std::cell::LazyCell::force(&*&raw const data185_378);
    std::cell::LazyCell::force(&*&raw const toler185_379);
    std::cell::LazyCell::force(&*&raw const data186_380);
    std::cell::LazyCell::force(&*&raw const toler186_381);
    std::cell::LazyCell::force(&*&raw const data187_382);
    std::cell::LazyCell::force(&*&raw const toler187_383);
    std::cell::LazyCell::force(&*&raw const data188_384);
    std::cell::LazyCell::force(&*&raw const toler188_385);
    std::cell::LazyCell::force(&*&raw const data189_386);
    std::cell::LazyCell::force(&*&raw const toler189_387);
    std::cell::LazyCell::force(&*&raw const data190_388);
    std::cell::LazyCell::force(&*&raw const toler190_389);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
