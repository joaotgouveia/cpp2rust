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
pub struct testcase_legendre_double_ {
    pub f0: (),
    pub l: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 0_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: -1_f64,
                l: 1_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.9_f64,
                l: 1_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.8_f64,
                l: 1_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.7_f64,
                l: 1_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.6_f64,
                l: 1_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.5_f64,
                l: 1_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.3999999999999999_f64,
                l: 1_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.29999999999999993_f64,
                l: 1_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.19999999999999996_f64,
                l: 1_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.09999999999999998_f64,
                l: 1_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0_f64,
                l: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10000000000000009_f64,
                l: 1_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.20000000000000018_f64,
                l: 1_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.30000000000000004_f64,
                l: 1_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.40000000000000013_f64,
                l: 1_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.5_f64,
                l: 1_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.6000000000000001_f64,
                l: 1_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.7000000000000002_f64,
                l: 1_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.8_f64,
                l: 1_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.9000000000000001_f64,
                l: 1_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 1_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_14: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.7150000000000001_f64,
                l: 2_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.23499999999999988_f64,
                l: 2_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.039999999999999925_f64,
                l: 2_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2600000000000001_f64,
                l: 2_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.36500000000000005_f64,
                l: 2_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.44000000000000006_f64,
                l: 2_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.5_f64,
                l: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.485_f64,
                l: 2_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.4399999999999999_f64,
                l: 2_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.365_f64,
                l: 2_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.25999999999999984_f64,
                l: 2_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.125_f64,
                l: 2_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.04000000000000015_f64,
                l: 2_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.23500000000000032_f64,
                l: 2_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.4600000000000002_f64,
                l: 2_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.7150000000000003_f64,
                l: 2_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 2_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_16: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: -1_f64,
                l: 5_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.04114125000000009_f64,
                l: 5_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.39951999999999993_f64,
                l: 5_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.3651987499999999_f64,
                l: 5_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.15263999999999994_f64,
                l: 5_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.08984375_f64,
                l: 5_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2706400000000001_f64,
                l: 5_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.34538624999999995_f64,
                l: 5_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.30751999999999996_f64,
                l: 5_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.17882874999999995_f64,
                l: 5_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0_f64,
                l: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.17882875000000015_f64,
                l: 5_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.3075200000000001_f64,
                l: 5_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.34538625_f64,
                l: 5_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.2706399999999999_f64,
                l: 5_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08984375_f64,
                l: 5_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.15264000000000016_f64,
                l: 5_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.36519875000000024_f64,
                l: 5_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.39951999999999993_f64,
                l: 5_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.04114124999999915_f64,
                l: 5_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 5_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_18: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2631456178558596_f64,
                l: 10_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08580579553164033_f64,
                l: 10_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2436627456_f64,
                l: 10_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.09683906440000026_f64,
                l: 10_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.12907202559999983_f64,
                l: 10_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.12212499738710943_f64,
                l: 10_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.24609375_f64,
                l: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.12212499738710922_f64,
                l: 10_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.12907202560000042_f64,
                l: 10_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.2514763495160156_f64,
                l: 10_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.09683906439999963_f64,
                l: 10_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.18822860717773438_f64,
                l: 10_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.24366274559999984_f64,
                l: 10_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08580579553164128_f64,
                l: 10_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.30052979560000004_f64,
                l: 10_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.2631456178558601_f64,
                l: 10_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 10_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_20: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.14930823530984835_f64,
                l: 20_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.20457394463834172_f64,
                l: 20_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.1591675291009811_f64,
                l: 20_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.10159261558628112_f64,
                l: 20_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.18028715947998047_f64,
                l: 20_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0980421943445948_f64,
                l: 20_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.08207713094452766_f64,
                l: 20_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.17619705200195312_f64,
                l: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.08207713094452802_f64,
                l: 20_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.09804219434459409_f64,
                l: 20_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.18028715947998042_f64,
                l: 20_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.10159261558628192_f64,
                l: 20_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.04835838106737356_f64,
                l: 20_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.15916752910098075_f64,
                l: 20_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.20457394463834136_f64,
                l: 20_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.22420460541741347_f64,
                l: 20_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.14930823530984758_f64,
                l: 20_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 20_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_22: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.1700376599438367_f64,
                l: 50_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.014572731645892852_f64,
                l: 50_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.058860798844002096_f64,
                l: 50_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.041569033381824674_f64,
                l: 50_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.1091105157471479_f64,
                l: 50_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.0834322722041975_f64,
                l: 50_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0382058126613136_f64,
                l: 50_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.11227517265921705_f64,
                l: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.038205812661314155_f64,
                l: 50_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.08343227220419656_f64,
                l: 50_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10911051574714797_f64,
                l: 50_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.04156903338182601_f64,
                l: 50_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.03105909923960981_f64,
                l: 50_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.05886079884400143_f64,
                l: 50_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.014572731645890737_f64,
                l: 50_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.13879737345093113_f64,
                l: 50_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.1700376599438368_f64,
                l: 50_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 50_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_24: std::cell::LazyCell<[testcase_legendre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                x: -1_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10226582055871893_f64,
                l: 100_(),
                x: -0.9_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                x: -0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.07713250719977864_f64,
                l: 100_(),
                x: -0.7_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.02374702390513314_f64,
                l: 100_(),
                x: -0.6_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                x: -0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.07225820212568503_f64,
                l: 100_(),
                x: -0.3999999999999999_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.057127392202801046_f64,
                l: 100_(),
                x: -0.29999999999999993_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.014681835355659706_f64,
                l: 100_(),
                x: -0.19999999999999996_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0638950984347502_f64,
                l: 100_(),
                x: -0.09999999999999998_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.07958923738717873_f64,
                l: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.06389509843474976_f64,
                l: 100_(),
                x: 0.10000000000000009_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.014681835355657875_f64,
                l: 100_(),
                x: 0.20000000000000018_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.057127392202801566_f64,
                l: 100_(),
                x: 0.30000000000000004_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.07225820212568408_f64,
                l: 100_(),
                x: 0.40000000000000013_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0605180259618612_f64,
                l: 100_(),
                x: 0.5_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.023747023905134217_f64,
                l: 100_(),
                x: 0.6000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: -0.0771325071997805_f64,
                l: 100_(),
                x: 0.7000000000000002_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.05086116791358423_f64,
                l: 100_(),
                x: 0.8_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 0.10226582055872063_f64,
                l: 100_(),
                x: 0.9000000000000001_f64,
                f: 0_f64,
            },
            testcase_legendre_double_ {
                f0: 1_f64,
                l: 100_(),
                x: 1_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub unsafe fn test_26(data: *const [testcase_legendre_double_; 21], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 21_();
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
                __builtin_fprintf_27 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 328_() ) .into() , ( c"void test(const testcase_legendre<Ret> (&)[Num], Ret) [Ret = double, Num = 21U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_15)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_17)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_19)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_21)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_23)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_25)),
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
        this.pid_ = ((unsafe { getpid_28() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_29(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_30(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) }) == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_31(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_32(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_33(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_32(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_33(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_32((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_33(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_34(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_32(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_33(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_35(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_36(
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
pub unsafe fn run_tests_wrapped_locale_37(mut name: *const (), l: *const __gnu_test_func_callback) {
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
                    __builtin_fprintf_27 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_38(
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
        if ((self.pid_) == (unsafe { getpid_28() })) {
            (unsafe { semctl_30(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_39(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_39(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_40(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_41));
}
static mut tmp_41: std::cell::LazyCell<::libc::tm> =
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
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
