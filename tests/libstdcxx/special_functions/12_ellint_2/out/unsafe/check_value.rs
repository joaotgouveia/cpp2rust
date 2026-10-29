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
            unimplemented!();
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
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)).postfix_inc();
        if ((*std::cell::LazyCell::force_mut(&mut *&raw mut count__3))
            == (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)))
        {
            unimplemented!();
        }
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count__3)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = 0_();
    }
    pub unsafe fn throw_on(mut count: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut throw_on__4)) = count;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub unsafe fn count() -> () {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5));
    }
    pub unsafe fn mark_call() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)).postfix_inc();
    }
    pub unsafe fn reset() {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut _M_count_5)) = 0_();
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut next_id__6)).postfix_dec()
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
            if operator_lt_7(
                self as *const __gnu_test_copy_tracker,
                other as *const __gnu_test_copy_tracker,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(
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
            operator_eq_8(
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
pub unsafe fn operator_eq_8(
    lhs: *const __gnu_test_copy_tracker,
    rhs: *const __gnu_test_copy_tracker,
) -> () {
    return ((unsafe { __gnu_test_copy_tracker::id(&(*lhs)) })
        == (unsafe { __gnu_test_copy_tracker::id(&(*rhs)) }));
}
pub unsafe fn operator_lt_7(
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
    pid_: (),
}
impl __gnu_test_semaphore {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct testcase_ellint_2_double_ {
    pub f0: (),
    pub k: (),
    pub phi: (),
    pub f: (),
}
pub static mut data001_9: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.9_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17381690606167963_f64,
                k: -0.9_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.34337919186972055_f64,
                k: -0.9_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5046426865985634_f64,
                k: -0.9_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6540000384236859_f64,
                k: -0.9_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.7885492841990464_f64,
                k: -0.9_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.906456986263154_f64,
                k: -0.9_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0075154899135927_f64,
                k: -0.9_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.094013558319407_f64,
                k: -0.9_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.1716970527816142_f64,
                k: -0.9_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_11: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17396762274534805_f64,
                k: -0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3445868522696932_f64,
                k: -0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5087292365450243_f64,
                k: -0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6637201653917623_f64,
                k: -0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8076034441016741_f64,
                k: -0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.9394548037249507_f64,
                k: -0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.059747331039504_f64,
                k: -0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.170698186245236_f64,
                k: -0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2763499431699064_f64,
                k: -0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_13: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.7_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.1741004124270254_f64,
                k: -0.7_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3456460508576476_f64,
                k: -0.7_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5122849569331466_f64,
                k: -0.7_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6720765409879953_f64,
                k: -0.7_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8237093263155649_f64,
                k: -0.7_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.9667231330945281_f64,
                k: -0.7_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.10170906449495_f64,
                k: -0.7_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2304180097292916_f64,
                k: -0.7_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3556611355719554_f64,
                k: -0.7_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_15: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.6_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17421534919599127_f64,
                k: -0.6_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.34655927787174096_f64,
                k: -0.6_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5153303453843214_f64,
                k: -0.6_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6791655059745302_f64,
                k: -0.6_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8372021818034988_f64,
                k: -0.6_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.9892215935493776_f64,
                k: -0.6_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.1357478470419362_f64,
                k: -0.6_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.278061737284406_f64,
                k: -0.6_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.418083394448724_f64,
                k: -0.6_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_17: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17431249677315908_f64,
                k: -0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.34732862537770803_f64,
                k: -0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5178819348599379_f64,
                k: -0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6850602295416455_f64,
                k: -0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8483166280334721_f64,
                k: -0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.007555555144472_f64,
                k: -0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.1631768599287302_f64,
                k: -0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3160584048772548_f64,
                k: -0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.4674622093394272_f64,
                k: -0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_19: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.3999999999999999_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17439190872481267_f64,
                k: -0.3999999999999999_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3479558176709921_f64,
                k: -0.3999999999999999_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5199529068380446_f64,
                k: -0.3999999999999999_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6898163846443153_f64,
                k: -0.3999999999999999_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8572208885993603_f64,
                k: -0.3999999999999999_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0221301327876993_f64,
                k: -0.3999999999999999_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.1848138019818375_f64,
                k: -0.3999999999999999_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3458259266501533_f64,
                k: -0.3999999999999999_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.5059416123600404_f64,
                k: -0.3999999999999999_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_21: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.29999999999999993_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17445362864048913_f64,
                k: -0.29999999999999993_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3484422353571347_f64,
                k: -0.29999999999999993_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5215535387741176_f64,
                k: -0.29999999999999993_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6934758441836989_f64,
                k: -0.29999999999999993_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8640360992823766_f64,
                k: -0.29999999999999993_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0332234514065408_f64,
                k: -0.29999999999999993_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2011943182068923_f64,
                k: -0.29999999999999993_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3682566113689625_f64,
                k: -0.29999999999999993_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.534833464923249_f64,
                k: -0.29999999999999993_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_23: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.19999999999999996_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.1744976902765281_f64,
                k: -0.19999999999999996_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3487889340076209_f64,
                k: -0.19999999999999996_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5226915285605741_f64,
                k: -0.19999999999999996_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6960691336015757_f64,
                k: -0.19999999999999996_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8688478237486338_f64,
                k: -0.19999999999999996_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0410255369689567_f64,
                k: -0.19999999999999996_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2126730391631364_f64,
                k: -0.19999999999999996_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.383925954032515_f64,
                k: -0.19999999999999996_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.5549685462425293_f64,
                k: -0.19999999999999996_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_25: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: -0.09999999999999998_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17452411766649942_f64,
                k: -0.09999999999999998_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3489966580544241_f64,
                k: -0.09999999999999998_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5233722240050879_f64,
                k: -0.09999999999999998_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6976170521728486_f64,
                k: -0.09999999999999998_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8717130927300749_f64,
                k: -0.09999999999999998_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0456602197056326_f64,
                k: -0.09999999999999998_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2194762899272027_f64,
                k: -0.09999999999999998_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3931950229892747_f64,
                k: -0.09999999999999998_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.5668619420216683_f64,
                k: -0.09999999999999998_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_26: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_27: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17453292519943295_f64,
                k: 0_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3490658503988659_f64,
                k: 0_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5235987755982988_f64,
                k: 0_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6981317007977318_f64,
                k: 0_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8726646259971648_f64,
                k: 0_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0471975511965976_f64,
                k: 0_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2217304763960306_f64,
                k: 0_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3962634015954636_f64,
                k: 0_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.5707963267948966_f64,
                k: 0_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_29: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.10000000000000009_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17452411766649942_f64,
                k: 0.10000000000000009_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3489966580544241_f64,
                k: 0.10000000000000009_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5233722240050879_f64,
                k: 0.10000000000000009_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6976170521728486_f64,
                k: 0.10000000000000009_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8717130927300749_f64,
                k: 0.10000000000000009_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0456602197056326_f64,
                k: 0.10000000000000009_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2194762899272027_f64,
                k: 0.10000000000000009_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3931950229892747_f64,
                k: 0.10000000000000009_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.5668619420216683_f64,
                k: 0.10000000000000009_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_30: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_31: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.20000000000000018_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.1744976902765281_f64,
                k: 0.20000000000000018_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3487889340076209_f64,
                k: 0.20000000000000018_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5226915285605741_f64,
                k: 0.20000000000000018_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6960691336015757_f64,
                k: 0.20000000000000018_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8688478237486338_f64,
                k: 0.20000000000000018_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0410255369689567_f64,
                k: 0.20000000000000018_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2126730391631364_f64,
                k: 0.20000000000000018_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.383925954032515_f64,
                k: 0.20000000000000018_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.5549685462425291_f64,
                k: 0.20000000000000018_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_32: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_33: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.30000000000000004_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17445362864048913_f64,
                k: 0.30000000000000004_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3484422353571347_f64,
                k: 0.30000000000000004_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5215535387741176_f64,
                k: 0.30000000000000004_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6934758441836989_f64,
                k: 0.30000000000000004_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8640360992823766_f64,
                k: 0.30000000000000004_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0332234514065408_f64,
                k: 0.30000000000000004_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2011943182068923_f64,
                k: 0.30000000000000004_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3682566113689625_f64,
                k: 0.30000000000000004_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.534833464923249_f64,
                k: 0.30000000000000004_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_34: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_35: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.40000000000000013_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17439190872481267_f64,
                k: 0.40000000000000013_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3479558176709921_f64,
                k: 0.40000000000000013_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5199529068380446_f64,
                k: 0.40000000000000013_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6898163846443153_f64,
                k: 0.40000000000000013_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8572208885993603_f64,
                k: 0.40000000000000013_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0221301327876993_f64,
                k: 0.40000000000000013_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.1848138019818375_f64,
                k: 0.40000000000000013_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.345825926650153_f64,
                k: 0.40000000000000013_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.5059416123600402_f64,
                k: 0.40000000000000013_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_36: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_37: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.5_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17431249677315908_f64,
                k: 0.5_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.34732862537770803_f64,
                k: 0.5_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5178819348599379_f64,
                k: 0.5_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6850602295416455_f64,
                k: 0.5_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8483166280334721_f64,
                k: 0.5_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.007555555144472_f64,
                k: 0.5_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.1631768599287302_f64,
                k: 0.5_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3160584048772548_f64,
                k: 0.5_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.4674622093394272_f64,
                k: 0.5_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_38: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_39: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.6000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17421534919599127_f64,
                k: 0.6000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.34655927787174096_f64,
                k: 0.6000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5153303453843214_f64,
                k: 0.6000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6791655059745302_f64,
                k: 0.6000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8372021818034988_f64,
                k: 0.6000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.9892215935493776_f64,
                k: 0.6000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.135747847041936_f64,
                k: 0.6000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.278061737284406_f64,
                k: 0.6000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.418083394448724_f64,
                k: 0.6000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_40: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_41: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.7000000000000002_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.1741004124270254_f64,
                k: 0.7000000000000002_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3456460508576476_f64,
                k: 0.7000000000000002_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5122849569331466_f64,
                k: 0.7000000000000002_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6720765409879952_f64,
                k: 0.7000000000000002_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8237093263155649_f64,
                k: 0.7000000000000002_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.9667231330945281_f64,
                k: 0.7000000000000002_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.10170906449495_f64,
                k: 0.7000000000000002_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2304180097292916_f64,
                k: 0.7000000000000002_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.3556611355719552_f64,
                k: 0.7000000000000002_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_42: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data018_43: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.8_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17396762274534805_f64,
                k: 0.8_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.3445868522696932_f64,
                k: 0.8_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5087292365450243_f64,
                k: 0.8_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6637201653917623_f64,
                k: 0.8_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.8076034441016741_f64,
                k: 0.8_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.9394548037249507_f64,
                k: 0.8_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.059747331039504_f64,
                k: 0.8_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.170698186245236_f64,
                k: 0.8_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.2763499431699064_f64,
                k: 0.8_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_44: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data019_45: std::cell::LazyCell<[testcase_ellint_2_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_ellint_2_double_ {
                f0: 0_f64,
                k: 0.9000000000000001_f64,
                phi: 0_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.17381690606167963_f64,
                k: 0.9000000000000001_f64,
                phi: 0.17453292519943295_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.34337919186972055_f64,
                k: 0.9000000000000001_f64,
                phi: 0.3490658503988659_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.5046426865985634_f64,
                k: 0.9000000000000001_f64,
                phi: 0.5235987755982988_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.6540000384236859_f64,
                k: 0.9000000000000001_f64,
                phi: 0.6981317007977318_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.7885492841990464_f64,
                k: 0.9000000000000001_f64,
                phi: 0.8726646259971648_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 0.9064569862631539_f64,
                k: 0.9000000000000001_f64,
                phi: 1.0471975511965976_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0075154899135927_f64,
                k: 0.9000000000000001_f64,
                phi: 1.2217304763960306_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.0940135583194068_f64,
                k: 0.9000000000000001_f64,
                phi: 1.3962634015954636_f64,
                f: 0_f64,
            },
            testcase_ellint_2_double_ {
                f0: 1.171697052781614_f64,
                k: 0.9000000000000001_f64,
                phi: 1.5707963267948966_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_46: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_47(data: *const [testcase_ellint_2_double_; 10], mut toler: ()) {
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
            unimplemented!();
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
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_9)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_10)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_11)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_12)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_13)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_14)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_15)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_16)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_17)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_18)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_19)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_20)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_21)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_22)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_23)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_24)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_25)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_26)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_27)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_28)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_29)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_30)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_31)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_32)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_33)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_34)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_35)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_36)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_37)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_38)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_39)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_40)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_41)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_42)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_43)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_44)),
        )
    });
    (unsafe {
        test_47(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_45)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_46)),
        )
    });
    return 0_();
}
impl __gnu_test_semaphore {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            sem_set_: unimplemented!(),
            pid_: unimplemented!(),
        };
        this.pid_ = unimplemented!();
        this.sem_set_ = unimplemented!();
        if ((this.sem_set_) == (-1_())) {
            unimplemented!();
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
        this
    }
}
pub unsafe fn set_memory_limits_48(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) * (1048576_()));
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn set_file_limit_49(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_50(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = unimplemented!();
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
    if !(unimplemented!()) {
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn run_tests_wrapped_locale_51(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_52(
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
        unimplemented!();
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut count__3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut throw_on__4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut _M_count_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut next_id__6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union __gnu_test_semun {
    pub val: (),
    pub buf: *mut (),
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
        if ((self.pid_) == (unimplemented!())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [(); 1] = [unimplemented!()];
        if ((unimplemented!()) == (-1_())) {
            unimplemented!();
        }
    }
}
pub unsafe fn test_tm_53(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_54)).clone();
}
static mut tmp_54: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const data001_9);
    std::cell::LazyCell::force(&*&raw const toler001_10);
    std::cell::LazyCell::force(&*&raw const data002_11);
    std::cell::LazyCell::force(&*&raw const toler002_12);
    std::cell::LazyCell::force(&*&raw const data003_13);
    std::cell::LazyCell::force(&*&raw const toler003_14);
    std::cell::LazyCell::force(&*&raw const data004_15);
    std::cell::LazyCell::force(&*&raw const toler004_16);
    std::cell::LazyCell::force(&*&raw const data005_17);
    std::cell::LazyCell::force(&*&raw const toler005_18);
    std::cell::LazyCell::force(&*&raw const data006_19);
    std::cell::LazyCell::force(&*&raw const toler006_20);
    std::cell::LazyCell::force(&*&raw const data007_21);
    std::cell::LazyCell::force(&*&raw const toler007_22);
    std::cell::LazyCell::force(&*&raw const data008_23);
    std::cell::LazyCell::force(&*&raw const toler008_24);
    std::cell::LazyCell::force(&*&raw const data009_25);
    std::cell::LazyCell::force(&*&raw const toler009_26);
    std::cell::LazyCell::force(&*&raw const data010_27);
    std::cell::LazyCell::force(&*&raw const toler010_28);
    std::cell::LazyCell::force(&*&raw const data011_29);
    std::cell::LazyCell::force(&*&raw const toler011_30);
    std::cell::LazyCell::force(&*&raw const data012_31);
    std::cell::LazyCell::force(&*&raw const toler012_32);
    std::cell::LazyCell::force(&*&raw const data013_33);
    std::cell::LazyCell::force(&*&raw const toler013_34);
    std::cell::LazyCell::force(&*&raw const data014_35);
    std::cell::LazyCell::force(&*&raw const toler014_36);
    std::cell::LazyCell::force(&*&raw const data015_37);
    std::cell::LazyCell::force(&*&raw const toler015_38);
    std::cell::LazyCell::force(&*&raw const data016_39);
    std::cell::LazyCell::force(&*&raw const toler016_40);
    std::cell::LazyCell::force(&*&raw const data017_41);
    std::cell::LazyCell::force(&*&raw const toler017_42);
    std::cell::LazyCell::force(&*&raw const data018_43);
    std::cell::LazyCell::force(&*&raw const toler018_44);
    std::cell::LazyCell::force(&*&raw const data019_45);
    std::cell::LazyCell::force(&*&raw const toler019_46);
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
