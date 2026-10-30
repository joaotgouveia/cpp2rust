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
pub struct testcase_laguerre_double_ {
    pub f0: (),
    pub n: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_9: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_11: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -4_f64,
                n: 1_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -14_f64,
                n: 1_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -24_f64,
                n: 1_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -34_f64,
                n: 1_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -44_f64,
                n: 1_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -54_f64,
                n: 1_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -64_f64,
                n: 1_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -74_f64,
                n: 1_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -84_f64,
                n: 1_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -89_f64,
                n: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -94_f64,
                n: 1_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -99_f64,
                n: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_13: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3.5_f64,
                n: 2_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 31_f64,
                n: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 83.5_f64,
                n: 2_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 161_f64,
                n: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 263.5_f64,
                n: 2_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 391_f64,
                n: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 543.5_f64,
                n: 2_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 721_f64,
                n: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 923.5_f64,
                n: 2_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1151_f64,
                n: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1403.5_f64,
                n: 2_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1681_f64,
                n: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1983.5_f64,
                n: 2_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2311_f64,
                n: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2663.5_f64,
                n: 2_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3041_f64,
                n: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3443.5_f64,
                n: 2_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3871_f64,
                n: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 4323.5_f64,
                n: 2_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 4801_f64,
                n: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_15: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -3.1666666666666665_f64,
                n: 5_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 34.33333333333333_f64,
                n: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -355.25_f64,
                n: 5_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -4765.666666666667_f64,
                n: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -23040.666666666664_f64,
                n: 5_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -74399_f64,
                n: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -190559.41666666663_f64,
                n: 5_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -418865.6666666666_f64,
                n: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -825411.5_f64,
                n: 5_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -1498165.6666666665_f64,
                n: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -2550096.916666667_f64,
                n: 5_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -4122299_f64,
                n: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -6387115.666666667_f64,
                n: 5_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -9551265.666666668_f64,
                n: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -13858967.75_f64,
                n: 5_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -19595065.666666664_f64,
                n: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -27088153.166666668_f64,
                n: 5_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -36713699_f64,
                n: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -48897171.91666666_f64,
                n: 5_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -64117165.666666664_f64,
                n: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_17: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1.756276179453263_f64,
                n: 10_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 27.984126984126977_f64,
                n: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -237.51841517857147_f64,
                n: 10_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3227.8077601410932_f64,
                n: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -45786.19979745369_f64,
                n: 10_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 15129.571428571455_f64,
                n: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 7764800.817949459_f64,
                n: 10_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 79724066.60846558_f64,
                n: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 469865425.65122765_f64,
                n: 10_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2037190065.373898_f64,
                n: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 7187828002.982576_f64,
                n: 10_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 21804200401_f64,
                n: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 58854343015.61621_f64,
                n: 10_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 144688291819.51855_f64,
                n: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 329425241736.7004_f64,
                n: 10_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 703324772760.0828_f64,
                n: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1421627560118.6157_f64,
                n: 10_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2741055412243.857_f64,
                n: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 5071986977681.865_f64,
                n: 10_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 9051283795429.572_f64,
                n: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data006_19: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2.0202257444769134_f64,
                n: 20_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -11.961333867812119_f64,
                n: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -50.151037960139455_f64,
                n: 20_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2829.4728613531743_f64,
                n: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -11583.94789911354_f64,
                n: 20_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -18439.42450252094_f64,
                n: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -38838.223606979285_f64,
                n: 20_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 24799805.877530713_f64,
                n: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -673953823.5991328_f64,
                n: 20_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 7551960453.767255_f64,
                n: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 31286508510.614746_f64,
                n: 20_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -1379223608444.9155_f64,
                n: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -6692517968212.972_f64,
                n: 20_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 165423821874449.94_f64,
                n: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 3082390018008546.5_f64,
                n: 20_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 29500368536981676_f64,
                n: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 203535263549741856_f64,
                n: 20_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1129230951443290112_f64,
                n: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 5323926285556310016_f64,
                n: 20_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 22061882785931735040_f64,
                n: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data007_21: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1.4735258819430543_f64,
                n: 50_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 17.534183446338233_f64,
                n: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -195.6243661907738_f64,
                n: 50_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 980.2696188979103_f64,
                n: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 24812.277673870878_f64,
                n: 50_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 293000.5073596236_f64,
                n: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2316195.5013375278_f64,
                n: 50_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -14896937.968694873_f64,
                n: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -502066598.00813466_f64,
                n: 50_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2513677852.691687_f64,
                n: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 45129675503.53891_f64,
                n: 50_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -883876565337.9922_f64,
                n: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 9361319947203.842_f64,
                n: 50_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -80967880733583.23_f64,
                n: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 717391079438942.6_f64,
                n: 50_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -8217471769564841_f64,
                n: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 125952762290099776_f64,
                n: 50_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -2114003130804889088_f64,
                n: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 32438187475835133952_f64,
                n: 50_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -397101034870946922496_f64,
                n: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_23: std::cell::LazyCell<[testcase_laguerre_double_; 21]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_laguerre_double_ {
                f0: 1_f64,
                n: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1.45552716253288_f64,
                n: 100_(),
                x: 5_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 13.27766284430345_f64,
                n: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 91.73703845434245_f64,
                n: 100_(),
                x: 15_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 1854.0367283243388_f64,
                n: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -11281.698886837261_f64,
                n: 100_(),
                x: 25_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 170141.8698704655_f64,
                n: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -2950092.7025822806_f64,
                n: 100_(),
                x: 35_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -7272442.315600696_f64,
                n: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 295697471.9087616_f64,
                n: 100_(),
                x: 45_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 4847420871.269051_f64,
                n: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 59406998102.39229_f64,
                n: 100_(),
                x: 55_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 693492765740.2969_f64,
                n: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 6606192010150.315_f64,
                n: 100_(),
                x: 65_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 17125518672239.77_f64,
                n: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -870493767065150.1_f64,
                n: 100_(),
                x: 75_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -13763178176383768_f64,
                n: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 30667078414479584_f64,
                n: 100_(),
                x: 85_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: 2130722049038017280_f64,
                n: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -7270652300900782080_f64,
                n: 100_(),
                x: 95_f64,
                f: 0_f64,
            },
            testcase_laguerre_double_ {
                f0: -262922606930689163264_f64,
                n: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub unsafe fn test_25(data: *const [testcase_laguerre_double_; 21], mut toler: ()) {
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
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_9)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_10)),
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_11)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_12)),
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_13)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_14)),
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_15)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_16)),
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_17)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_18)),
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_19)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_20)),
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_21)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_22)),
        )
    });
    (unsafe {
        test_25(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_23)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_24)),
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
pub unsafe fn set_memory_limits_26(mut size: Option<()>) {
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
pub unsafe fn set_file_limit_27(mut size: ()) {
    let mut r: () = unimplemented!();
    let mut limit: () = ((size) as ());
    unimplemented!();
    r.rlim_cur = limit;
    unimplemented!();
}
pub unsafe fn verify_demangle_28(mut mangled: *const (), mut wanted: *const ()) {
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
pub unsafe fn run_tests_wrapped_locale_29(mut name: *const (), l: *const __gnu_test_func_callback) {
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
pub unsafe fn run_tests_wrapped_env_30(
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
pub unsafe fn test_tm_31(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_32)).clone();
}
static mut tmp_32: std::cell::LazyCell<()> =
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
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__3);
    std::cell::LazyCell::force(&*&raw const throw_on__4);
    std::cell::LazyCell::force(&*&raw const _M_count_5);
    std::cell::LazyCell::force(&*&raw const next_id__6);
}
