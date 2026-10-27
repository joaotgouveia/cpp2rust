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
pub struct testcase_ellint_1_double_ {
    pub f0: (),
    pub k: (),
    pub phi: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_14: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler003_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_16: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler004_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_18: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler005_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_20: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler006_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_22: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler007_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_24: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler008_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_26: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler009_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_28: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler010_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_30: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler011_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_32: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler012_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_34: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler013_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_36: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler014_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_38: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler015_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_40: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler016_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_42: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler017_43: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data018_44: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler018_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data019_46: std::cell::LazyCell<[testcase_ellint_1_double_; 10]> =
    std::cell::LazyCell::new(|| unsafe {
        [
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
        ]
    });
pub static mut toler019_47: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_48(data: *const [testcase_ellint_1_double_; 10], mut toler: ()) {
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
                __builtin_fprintf_49 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 482_() ) .into() , ( c"void test(const testcase_ellint_1<Ret> (&)[Num], Ret) [Ret = double, Num = 10U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_15)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_17)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_19)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_21)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_23)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_25)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_27)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_29)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_31)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_33)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_35)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_37)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_39)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_41)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_42)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_43)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_44)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_45)),
        )
    });
    (unsafe {
        test_48(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_46)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_47)),
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
        this.pid_ = ((unsafe { getpid_50() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_51(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_52(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_53(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_54(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_55(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_54(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_55(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_54((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_55(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_56(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_54(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_55(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_57(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_58(
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
pub unsafe fn run_tests_wrapped_locale_59(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_49 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_60(
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
        if ((self.pid_) == (unsafe { getpid_50() })) {
            (unsafe { semctl_52(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_61(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_61(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_62(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_63));
}
static mut tmp_63: std::cell::LazyCell<::libc::tm> =
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
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
