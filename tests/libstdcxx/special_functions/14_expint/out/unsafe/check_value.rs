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
pub struct testcase_expint_double_ {
    pub f0: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_expint_double_; 50]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_expint_double_; 50]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_14(data: *const [testcase_expint_double_; 50], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 50_();
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
                __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 191_() ) .into() , ( c"void test(const testcase_expint<Ret> (&)[Num], Ret) [Ret = double, Num = 50U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_14(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_14(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
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
        this.pid_ = ((unsafe { getpid_16() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_17(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_18(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_19(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_20(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_21(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_20(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_21(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_20((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_21(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_22(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_20(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_21(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_23(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_24(
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
pub unsafe fn run_tests_wrapped_locale_25(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_15 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_26(
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
        if ((self.pid_) == (unsafe { getpid_16() })) {
            (unsafe { semctl_18(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_27(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_27(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_28(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_29));
}
static mut tmp_29: std::cell::LazyCell<::libc::tm> =
    std::cell::LazyCell::new(|| unsafe { unsafe { std::mem::zeroed() } });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data001_10);
    std::cell::LazyCell::force(&*&raw const toler001_11);
    std::cell::LazyCell::force(&*&raw const data002_12);
    std::cell::LazyCell::force(&*&raw const toler002_13);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
