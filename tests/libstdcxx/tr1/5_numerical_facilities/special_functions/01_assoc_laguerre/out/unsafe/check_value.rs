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
pub struct testcase_assoc_laguerre_double_ {
    pub f0: (),
    pub n: (),
    pub m: (),
    pub x: (),
    pub f: (),
}
pub static mut data001_10: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler001_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data002_12: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler002_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data003_14: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler003_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data004_16: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler004_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data005_18: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler005_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data006_20: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler006_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data007_22: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler007_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data008_24: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 0_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler008_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data009_26: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -89_f64,
                n: 1_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -99_f64,
                n: 1_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler009_27: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data010_28: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 2_f64,
                n: 1_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8_f64,
                n: 1_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18_f64,
                n: 1_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -28_f64,
                n: 1_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -38_f64,
                n: 1_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48_f64,
                n: 1_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -58_f64,
                n: 1_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -68_f64,
                n: 1_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -78_f64,
                n: 1_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -88_f64,
                n: 1_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -98_f64,
                n: 1_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler010_29: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data011_30: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 3_f64,
                n: 1_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7_f64,
                n: 1_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -17_f64,
                n: 1_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -27_f64,
                n: 1_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -37_f64,
                n: 1_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -47_f64,
                n: 1_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -57_f64,
                n: 1_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -67_f64,
                n: 1_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -77_f64,
                n: 1_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -87_f64,
                n: 1_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -97_f64,
                n: 1_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler011_31: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data012_32: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 6_f64,
                n: 1_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4_f64,
                n: 1_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14_f64,
                n: 1_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -24_f64,
                n: 1_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -34_f64,
                n: 1_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -44_f64,
                n: 1_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -54_f64,
                n: 1_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -64_f64,
                n: 1_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -74_f64,
                n: 1_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -84_f64,
                n: 1_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -94_f64,
                n: 1_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler012_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data013_34: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -89_f64,
                n: 1_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler013_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data014_36: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 1_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -59_f64,
                n: 1_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -69_f64,
                n: 1_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79_f64,
                n: 1_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler014_37: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data015_38: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 51_f64,
                n: 1_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 41_f64,
                n: 1_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31_f64,
                n: 1_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 1_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9_f64,
                n: 1_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19_f64,
                n: 1_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -29_f64,
                n: 1_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -39_f64,
                n: 1_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -49_f64,
                n: 1_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler015_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data016_40: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 101_f64,
                n: 1_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 91_f64,
                n: 1_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 81_f64,
                n: 1_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 71_f64,
                n: 1_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 61_f64,
                n: 1_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 51_f64,
                n: 1_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 41_f64,
                n: 1_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31_f64,
                n: 1_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 1_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 1_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 1_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler016_41: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data017_42: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 2_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31_f64,
                n: 2_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 161_f64,
                n: 2_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 391_f64,
                n: 2_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 721_f64,
                n: 2_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1151_f64,
                n: 2_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1681_f64,
                n: 2_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2311_f64,
                n: 2_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3041_f64,
                n: 2_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3871_f64,
                n: 2_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4801_f64,
                n: 2_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler017_43: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data018_44: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 3_f64,
                n: 2_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 23_f64,
                n: 2_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 143_f64,
                n: 2_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 363_f64,
                n: 2_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 683_f64,
                n: 2_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1103_f64,
                n: 2_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1623_f64,
                n: 2_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2243_f64,
                n: 2_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2963_f64,
                n: 2_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3783_f64,
                n: 2_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4703_f64,
                n: 2_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler018_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data019_46: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 6_f64,
                n: 2_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16_f64,
                n: 2_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 126_f64,
                n: 2_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 336_f64,
                n: 2_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 646_f64,
                n: 2_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1056_f64,
                n: 2_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1566_f64,
                n: 2_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2176_f64,
                n: 2_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2886_f64,
                n: 2_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3696_f64,
                n: 2_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4606_f64,
                n: 2_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler019_47: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data020_48: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 2_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 0.9999999999999954_f64,
                n: 2_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 81_f64,
                n: 2_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 261_f64,
                n: 2_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 541_f64,
                n: 2_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 921_f64,
                n: 2_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1401_f64,
                n: 2_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1981_f64,
                n: 2_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2661_f64,
                n: 2_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3441_f64,
                n: 2_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4321_f64,
                n: 2_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler020_49: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data021_50: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 66_f64,
                n: 2_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4.00000000000001_f64,
                n: 2_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 25.999999999999964_f64,
                n: 2_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 156_f64,
                n: 2_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 386_f64,
                n: 2_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 716_f64,
                n: 2_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1146_f64,
                n: 2_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1676_f64,
                n: 2_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2306_f64,
                n: 2_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3036_f64,
                n: 2_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3866_f64,
                n: 2_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler021_51: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data022_52: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 231_f64,
                n: 2_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 61.000000000000206_f64,
                n: 2_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9.000000000000005_f64,
                n: 2_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21.000000000000124_f64,
                n: 2_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 151.00000000000057_f64,
                n: 2_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 381_f64,
                n: 2_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 711_f64,
                n: 2_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1141_f64,
                n: 2_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1670.9999999999998_f64,
                n: 2_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2301_f64,
                n: 2_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3031_f64,
                n: 2_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler022_53: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data023_54: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1326_f64,
                n: 2_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 855.9999999999969_f64,
                n: 2_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 485.99999999999835_f64,
                n: 2_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 215.99999999999937_f64,
                n: 2_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 45.99999999999983_f64,
                n: 2_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -23.999999999999538_f64,
                n: 2_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6.000000000000106_f64,
                n: 2_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 135.99999999999963_f64,
                n: 2_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 365.9999999999989_f64,
                n: 2_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 695.9999999999978_f64,
                n: 2_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1125.9999999999964_f64,
                n: 2_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler023_55: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data024_56: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 5151_f64,
                n: 2_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4181.0000000000655_f64,
                n: 2_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3311.000000000052_f64,
                n: 2_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2541.00000000004_f64,
                n: 2_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1871.000000000029_f64,
                n: 2_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1301.0000000000207_f64,
                n: 2_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 831.0000000000136_f64,
                n: 2_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 461.0000000000068_f64,
                n: 2_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 191.0000000000025_f64,
                n: 2_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21.000000000000046_f64,
                n: 2_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48.999999999999915_f64,
                n: 2_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler024_57: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data025_58: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 5_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 34.33333333333333_f64,
                n: 5_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4765.666666666667_f64,
                n: 5_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -74399_f64,
                n: 5_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -418865.6666666666_f64,
                n: 5_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1498165.6666666665_f64,
                n: 5_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4122299_f64,
                n: 5_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9551265.666666668_f64,
                n: 5_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19595065.666666664_f64,
                n: 5_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -36713699_f64,
                n: 5_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -64117165.666666664_f64,
                n: 5_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler025_59: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data026_60: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 6_f64,
                n: 5_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 22.66666666666666_f64,
                n: 5_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2960.666666666666_f64,
                n: 5_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -58944_f64,
                n: 5_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -357927.33333333326_f64,
                n: 5_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1329910.6666666665_f64,
                n: 5_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3744894_f64,
                n: 5_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8812877.333333332_f64,
                n: 5_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18283860.666666664_f64,
                n: 5_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -34547844_f64,
                n: 5_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -60734827.333333336_f64,
                n: 5_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler026_61: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data027_62: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 5_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4.333333333333334_f64,
                n: 5_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1679_f64,
                n: 5_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -46029_f64,
                n: 5_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -304045.6666666667_f64,
                n: 5_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1176729.0000000002_f64,
                n: 5_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3395079_f64,
                n: 5_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8120095.666666666_f64,
                n: 5_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -17042778.999999996_f64,
                n: 5_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -32484129_f64,
                n: 5_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -57495145.666666664_f64,
                n: 5_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler027_63: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data028_64: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 252_f64,
                n: 5_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14.666666666666654_f64,
                n: 5_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 51.99999999999991_f64,
                n: 5_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19548_f64,
                n: 5_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -178814.6666666666_f64,
                n: 5_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -797747.9999999998_f64,
                n: 5_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2496348_f64,
                n: 5_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6294614.666666666_f64,
                n: 5_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13712547.999999996_f64,
                n: 5_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -26870147.999999993_f64,
                n: 5_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48587414.66666667_f64,
                n: 5_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler028_65: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data029_66: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 3003_f64,
                n: 5_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 19.666666666666668_f64,
                n: 5_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 36.33333333333327_f64,
                n: 5_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1947_f64,
                n: 5_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -60930.333333333314_f64,
                n: 5_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -381913.6666666665_f64,
                n: 5_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1419897_f64,
                n: 5_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3979880.333333333_f64,
                n: 5_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9316863.666666664_f64,
                n: 5_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -19235847_f64,
                n: 5_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -36191830.33333333_f64,
                n: 5_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler029_67: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data030_68: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 53130_f64,
                n: 5_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1213.3333333333335_f64,
                n: 5_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 129.99999999999963_f64,
                n: 5_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -119.99999999999974_f64,
                n: 5_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 463.3333333333332_f64,
                n: 5_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48120.000000000015_f64,
                n: 5_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -345870.0000000002_f64,
                n: 5_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1342786.6666666667_f64,
                n: 5_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3838870.000000001_f64,
                n: 5_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9084120_f64,
                n: 5_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18878536.666666668_f64,
                n: 5_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler030_69: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data031_70: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 3478761_f64,
                n: 5_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1154544.3333333335_f64,
                n: 5_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 264661.00000000006_f64,
                n: 5_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 24111.000000000033_f64,
                n: 5_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2105.6666666666665_f64,
                n: 5_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1010.9999999999916_f64,
                n: 5_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1538.9999999999955_f64,
                n: 5_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5244.333333333345_f64,
                n: 5_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13639.000000000011_f64,
                n: 5_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -243189.00000000006_f64,
                n: 5_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1118405.6666666667_f64,
                n: 5_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler031_71: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data032_72: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 96560646_f64,
                n: 5_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 57264262.66666665_f64,
                n: 5_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 31841379.333333332_f64,
                n: 5_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16281996_f64,
                n: 5_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7426112.666666667_f64,
                n: 5_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2863729.333333333_f64,
                n: 5_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 834846_f64,
                n: 5_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 129462.66666666663_f64,
                n: 5_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -12420.666666666668_f64,
                n: 5_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -804_f64,
                n: 5_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4312.666666666667_f64,
                n: 5_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler032_73: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data033_74: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 10_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 27.984126984126977_f64,
                n: 10_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3227.8077601410932_f64,
                n: 10_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 15129.571428571455_f64,
                n: 10_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 79724066.60846558_f64,
                n: 10_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2037190065.373898_f64,
                n: 10_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 21804200401_f64,
                n: 10_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 144688291819.51855_f64,
                n: 10_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 703324772760.0828_f64,
                n: 10_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2741055412243.857_f64,
                n: 10_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 9051283795429.572_f64,
                n: 10_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler033_75: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data034_76: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 11_f64,
                n: 10_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 14.791887125220455_f64,
                n: 10_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2704.6507936507933_f64,
                n: 10_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -182924.71428571423_f64,
                n: 10_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 48066036.749559075_f64,
                n: 10_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1486264192.216931_f64,
                n: 10_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17239562282.428574_f64,
                n: 10_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 119837491630.13579_f64,
                n: 10_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 600681375251.2117_f64,
                n: 10_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2392908405632.4287_f64,
                n: 10_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 8033035722509.237_f64,
                n: 10_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler034_77: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data035_78: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 66_f64,
                n: 10_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14.51146384479718_f64,
                n: 10_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1064.5890652557316_f64,
                n: 10_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -194569.7142857143_f64,
                n: 10_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 27343569.350970022_f64,
                n: 10_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1067807661.6790125_f64,
                n: 10_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 13529451580.285711_f64,
                n: 10_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 98812724224.64194_f64,
                n: 10_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 511482736187.3402_f64,
                n: 10_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2084478393087.4285_f64,
                n: 10_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7117724862237.075_f64,
                n: 10_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler035_79: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data036_80: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 3003_f64,
                n: 10_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11.641975308642031_f64,
                n: 10_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1137.564373897707_f64,
                n: 10_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9254.14285714286_f64,
                n: 10_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2121878.8377425023_f64,
                n: 10_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 352060171.430335_f64,
                n: 10_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6212028560.142858_f64,
                n: 10_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 53782171674.60492_f64,
                n: 10_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 309720255837.56775_f64,
                n: 10_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1359043035731.5713_f64,
                n: 10_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4900625954398.943_f64,
                n: 10_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler036_81: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data037_82: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 184756_f64,
                n: 10_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -210.84303350970018_f64,
                n: 10_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 508.38095238095184_f64,
                n: 10_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2098.857142857143_f64,
                n: 10_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -536338.8853615518_f64,
                n: 10_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 24865988.804232784_f64,
                n: 10_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1343756013.142857_f64,
                n: 10_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17298791247.358025_f64,
                n: 10_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 124528450897.79892_f64,
                n: 10_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 632674413641.7142_f64,
                n: 10_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2533008935405.03_f64,
                n: 10_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler037_83: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data038_84: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 30045014.999999993_f64,
                n: 10_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -23087.733686067022_f64,
                n: 10_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 207.2398589065233_f64,
                n: 10_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1407.8571428571508_f64,
                n: 10_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -44618.15696649032_f64,
                n: 10_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 158690.044091711_f64,
                n: 10_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6870413.571428573_f64,
                n: 10_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 793841351.4197533_f64,
                n: 10_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 13358288958.562618_f64,
                n: 10_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 106073722407.85715_f64,
                n: 10_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 566337213392.4249_f64,
                n: 10_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler038_85: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data039_86: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 75394027566_f64,
                n: 10_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 8048106183.372136_f64,
                n: 10_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 328045023.8483246_f64,
                n: 10_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2568769.714285718_f64,
                n: 10_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6971.996472663153_f64,
                n: 10_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 136111.41446207993_f64,
                n: 10_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -62462.57142857024_f64,
                n: 10_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -248167.95061728527_f64,
                n: 10_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1941270.409171077_f64,
                n: 10_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8643512.571428576_f64,
                n: 10_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -140863522.18342152_f64,
                n: 10_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler039_87: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data040_88: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 46897636623981_f64,
                n: 10_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16444031323272.084_f64,
                n: 10_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5020343986463.539_f64,
                n: 10_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1270977490645.286_f64,
                n: 10_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 244835756822.62262_f64,
                n: 10_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 29786827693.96296_f64,
                n: 10_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1127612095.2857144_f64,
                n: 10_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -66370555.419753075_f64,
                n: 10_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 10420852.95767187_f64,
                n: 10_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3373097.571428572_f64,
                n: 10_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2065423.6807760145_f64,
                n: 10_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler040_89: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data041_90: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 20_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -11.961333867812119_f64,
                n: 20_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2829.4728613531743_f64,
                n: 20_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -18439.42450252094_f64,
                n: 20_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 24799805.877530713_f64,
                n: 20_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7551960453.767255_f64,
                n: 20_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1379223608444.9155_f64,
                n: 20_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 165423821874449.94_f64,
                n: 20_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 29500368536981676_f64,
                n: 20_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1129230951443290112_f64,
                n: 20_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 22061882785931735040_f64,
                n: 20_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler041_91: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data042_92: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 21_f64,
                n: 20_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 19.90048812973408_f64,
                n: 20_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2208.0318569557585_f64,
                n: 20_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 263690.96303121914_f64,
                n: 20_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 40667285.630564235_f64,
                n: 20_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1737442572.81154_f64,
                n: 20_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -588280953643.2812_f64,
                n: 20_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 45617733778241.33_f64,
                n: 20_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17293487114876864_f64,
                n: 20_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 762191358585850624_f64,
                n: 20_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16037288204336758784_f64,
                n: 20_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler042_93: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data043_94: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 231.00000000000003_f64,
                n: 20_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 47.00933806511292_f64,
                n: 20_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -652.5130546172859_f64,
                n: 20_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 285388.2589506911_f64,
                n: 20_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 28664069.68562473_f64,
                n: 20_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1399631966.3144732_f64,
                n: 20_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -115357373248.28194_f64,
                n: 20_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3357730872975.875_f64,
                n: 20_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 9765808962855122_f64,
                n: 20_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 507172929455591808_f64,
                n: 20_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 11564665701334456320_f64,
                n: 20_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler043_95: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data044_96: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 53130_f64,
                n: 20_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -158.69554500944142_f64,
                n: 20_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 334.0801228803895_f64,
                n: 20_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -198372.47662554163_f64,
                n: 20_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13627144.088579426_f64,
                n: 20_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -780579985.4473151_f64,
                n: 20_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 116648634237.73535_f64,
                n: 20_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -12347348707739.742_f64,
                n: 20_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1199516248034090.8_f64,
                n: 20_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 134515031950785312_f64,
                n: 20_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4105890427611148288_f64,
                n: 20_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler044_97: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data045_98: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 30045015_f64,
                n: 20_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1755.6226861258601_f64,
                n: 20_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9081.67266447379_f64,
                n: 20_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 95771.65091211311_f64,
                n: 20_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5089151.927277924_f64,
                n: 20_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 97400399.45020671_f64,
                n: 20_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -16009352450.477026_f64,
                n: 20_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 842271286905.0105_f64,
                n: 20_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79901725466796.94_f64,
                n: 20_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7944103675858637_f64,
                n: 20_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 574298218933882880_f64,
                n: 20_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler045_99: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data046_100: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 137846528819.99994_f64,
                n: 20_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -136976.4957133323_f64,
                n: 20_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 113878.49908041643_f64,
                n: 20_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -342529.2177879618_f64,
                n: 20_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -350112.66981443466_f64,
                n: 20_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -10791735.172977809_f64,
                n: 20_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1038073940.0811402_f64,
                n: 20_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 667312550.6361634_f64,
                n: 20_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 741537869902.2903_f64,
                n: 20_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -32378376755737.418_f64,
                n: 20_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -601760332167937.6_f64,
                n: 20_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler046_101: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data047_102: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 161884603662657888_f64,
                n: 20_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1599011936804291.5_f64,
                n: 20_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -131273880831.42432_f64,
                n: 20_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -3133213093.690355_f64,
                n: 20_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -213935628.04985175_f64,
                n: 20_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -47375578.49592106_f64,
                n: 20_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -115731015.14034876_f64,
                n: 20_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -737415147.2942029_f64,
                n: 20_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2123455626.862177_f64,
                n: 20_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 29801266858.60893_f64,
                n: 20_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -132886631026.82553_f64,
                n: 20_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler047_103: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data048_104: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 2.9462227291176643e+22_f64,
                n: 20_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3577789074870124412928_f64,
                n: 20_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 315849255214567587840_f64,
                n: 20_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17389599388424863744_f64,
                n: 20_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 414013427459806336_f64,
                n: 20_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -79359706102062.6_f64,
                n: 20_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 22736203650743.145_f64,
                n: 20_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 65679006380.0957_f64,
                n: 20_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -236263257610.77792_f64,
                n: 20_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -38072644585.3031_f64,
                n: 20_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 68236474365.17397_f64,
                n: 20_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler048_105: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data049_106: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 50_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17.534183446338233_f64,
                n: 50_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 980.2696188979103_f64,
                n: 50_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 293000.5073596236_f64,
                n: 50_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14896937.968694873_f64,
                n: 50_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2513677852.691687_f64,
                n: 50_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -883876565337.9922_f64,
                n: 50_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -80967880733583.23_f64,
                n: 50_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8217471769564841_f64,
                n: 50_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2114003130804889088_f64,
                n: 50_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -397101034870946922496_f64,
                n: 50_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler049_107: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data050_108: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 51.00000000000002_f64,
                n: 50_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.4214573271639575_f64,
                n: 50_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2574.8072295127827_f64,
                n: 50_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 35846.479728359205_f64,
                n: 50_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -48263698.76831853_f64,
                n: 50_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6161525870.273853_f64,
                n: 50_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -382655486658.47125_f64,
                n: 50_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -109635579833241.72_f64,
                n: 50_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14623805817283490_f64,
                n: 50_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2066684719087815168_f64,
                n: 50_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -143851879539976257536_f64,
                n: 50_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler050_109: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data051_110: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1326_f64,
                n: 50_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -87.86073251644453_f64,
                n: 50_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -5203.235119178092_f64,
                n: 50_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -461059.5001253887_f64,
                n: 50_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -30476695.32744059_f64,
                n: 50_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3720804977.9338136_f64,
                n: 50_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 362262002434.5145_f64,
                n: 50_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -52210917867820.23_f64,
                n: 50_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9567965136901914_f64,
                n: 50_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -891712775177128832_f64,
                n: 50_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 57231129448806981632_f64,
                n: 50_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler051_111: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data052_112: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 3478761_f64,
                n: 50_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1055.8381917651498_f64,
                n: 50_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 15264.646660345055_f64,
                n: 50_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1229651.8966600848_f64,
                n: 50_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 39270451.82365684_f64,
                n: 50_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4424062601.115203_f64,
                n: 50_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -186017434284.19223_f64,
                n: 50_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 50972853949302.61_f64,
                n: 50_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6530702754012517_f64,
                n: 50_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 683875927146780288_f64,
                n: 50_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 39198742504338391040_f64,
                n: 50_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler052_113: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data053_114: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 75394027565.99998_f64,
                n: 50_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 91833.92409877018_f64,
                n: 50_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 330501.8792977826_f64,
                n: 50_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3625088.1635972536_f64,
                n: 50_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 213954727.28632012_f64,
                n: 50_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9381006937.751768_f64,
                n: 50_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 535333683777.48615_f64,
                n: 50_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 18824406573722.17_f64,
                n: 50_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -533858276780013.1_f64,
                n: 50_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -52995774666704016_f64,
                n: 50_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6050418286244878336_f64,
                n: 50_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler053_115: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data054_116: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 161884603662657792_f64,
                n: 50_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -307637087.2516941_f64,
                n: 50_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 12524651.102974586_f64,
                n: 50_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -315460483.8621056_f64,
                n: 50_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1889683587.3459988_f64,
                n: 50_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 37457044404.20035_f64,
                n: 50_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -843831858224.718_f64,
                n: 50_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -92231643172.3075_f64,
                n: 50_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 904211757769501_f64,
                n: 50_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 46508193600283272_f64,
                n: 50_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2321688792816271872_f64,
                n: 50_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler054_117: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000003e-12_f64 });
pub static mut data055_118: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1.0089134454556417e+29_f64,
                n: 50_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.3822795753070493e+23_f64,
                n: 50_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 95817260381628336_f64,
                n: 50_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -910798580856015.4_f64,
                n: 50_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 50513254049166.92_f64,
                n: 50_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 84159703903348.94_f64,
                n: 50_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -138805244691822.72_f64,
                n: 50_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 181046391269246.25_f64,
                n: 50_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2086884905317107.5_f64,
                n: 50_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2765620139862428_f64,
                n: 50_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -137067516781462896_f64,
                n: 50_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler055_119: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data056_120: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 2.0128660909731929e+40_f64,
                n: 50_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 9.367509480769547e+37_f64,
                n: 50_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.3009321481877196e+35_f64,
                n: 50_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7.372002689323382e+30_f64,
                n: 50_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6.082467907963467e+25_f64,
                n: 50_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6.005318879354345e+23_f64,
                n: 50_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.4178129287264692e+22_f64,
                n: 50_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -546520993415667056640_f64,
                n: 50_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -108172717592632737792_f64,
                n: 50_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 38058734007924195328_f64,
                n: 50_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 47439240848028344320_f64,
                n: 50_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler056_121: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data057_122: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 1_f64,
                n: 100_(),
                m: 0_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 13.27766284430345_f64,
                n: 100_(),
                m: 0_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1854.0367283243388_f64,
                n: 100_(),
                m: 0_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 170141.8698704655_f64,
                n: 100_(),
                m: 0_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7272442.315600696_f64,
                n: 100_(),
                m: 0_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4847420871.269051_f64,
                n: 100_(),
                m: 0_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 693492765740.2969_f64,
                n: 100_(),
                m: 0_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 17125518672239.77_f64,
                n: 100_(),
                m: 0_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -13763178176383768_f64,
                n: 100_(),
                m: 0_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2130722049038017280_f64,
                n: 100_(),
                m: 0_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -262922606930689163264_f64,
                n: 100_(),
                m: 0_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler057_123: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.500000000000002e-13_f64 });
pub static mut data058_124: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 101.00000000000003_f64,
                n: 100_(),
                m: 1_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -14.65066198368042_f64,
                n: 100_(),
                m: 1_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1626.5010939361582_f64,
                n: 100_(),
                m: 1_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 417884.7765826873_f64,
                n: 100_(),
                m: 1_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -55617646.95164984_f64,
                n: 100_(),
                m: 1_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 884829874.2662622_f64,
                n: 100_(),
                m: 1_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 154466082750.32202_f64,
                n: 100_(),
                m: 1_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -101423973484646_f64,
                n: 100_(),
                m: 1_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1388352348671756.8_f64,
                n: 100_(),
                m: 1_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 780487055132685824_f64,
                n: 100_(),
                m: 1_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 16948925059042754560_f64,
                n: 100_(),
                m: 1_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler058_125: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data059_126: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 5151.0000000000055_f64,
                n: 100_(),
                m: 2_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -150.22012290951324_f64,
                n: 100_(),
                m: 2_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7655.059329404945_f64,
                n: 100_(),
                m: 2_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -140996.6927617909_f64,
                n: 100_(),
                m: 2_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -38645171.27854935_f64,
                n: 100_(),
                m: 2_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8889263688.211893_f64,
                n: 100_(),
                m: 2_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1010338971533.34_f64,
                n: 100_(),
                m: 2_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -127582564332943.9_f64,
                n: 100_(),
                m: 2_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 15970305694654312_f64,
                n: 100_(),
                m: 2_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1601984499286281984_f64,
                n: 100_(),
                m: 2_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 282670247309629554688_f64,
                n: 100_(),
                m: 2_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler059_127: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data060_128: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 96560646.00000003_f64,
                n: 100_(),
                m: 5_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2430.6732236677612_f64,
                n: 100_(),
                m: 5_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 111162.32026994647_f64,
                n: 100_(),
                m: 5_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 4036708.259941363_f64,
                n: 100_(),
                m: 5_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -34055982.66440544_f64,
                n: 100_(),
                m: 5_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 30110688343.56233_f64,
                n: 100_(),
                m: 5_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2651429940558.2974_f64,
                n: 100_(),
                m: 5_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 192108556058943.1_f64,
                n: 100_(),
                m: 5_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -25410533973455528_f64,
                n: 100_(),
                m: 5_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2107295563356443136_f64,
                n: 100_(),
                m: 5_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -294340053558772891648_f64,
                n: 100_(),
                m: 5_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler060_129: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data061_130: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 46897636623981.04_f64,
                n: 100_(),
                m: 10_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 529208.1155099033_f64,
                n: 100_(),
                m: 10_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7402892.174880382_f64,
                n: 100_(),
                m: 10_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 88369632.08324358_f64,
                n: 100_(),
                m: 10_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 822187797.5909656_f64,
                n: 100_(),
                m: 10_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 180231446033.06866_f64,
                n: 100_(),
                m: 10_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 7922942703798.131_f64,
                n: 100_(),
                m: 10_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 784424250559042.1_f64,
                n: 100_(),
                m: 10_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -16325634720239370_f64,
                n: 100_(),
                m: 10_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1087958830744316160_f64,
                n: 100_(),
                m: 10_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 30114394463610642432_f64,
                n: 100_(),
                m: 10_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler061_131: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 5.000000000000004e-13_f64 });
pub static mut data062_132: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 2.9462227291176614e+22_f64,
                n: 100_(),
                m: 20_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 313694958939.90405_f64,
                n: 100_(),
                m: 20_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 45396489338.09619_f64,
                n: 100_(),
                m: 20_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7215826758.008125_f64,
                n: 100_(),
                m: 20_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 825949194005.8885_f64,
                n: 100_(),
                m: 20_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2764742119971.081_f64,
                n: 100_(),
                m: 20_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -219802198273516.03_f64,
                n: 100_(),
                m: 20_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1699053306145262_f64,
                n: 100_(),
                m: 20_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 354957093450238464_f64,
                n: 100_(),
                m: 20_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -9612867511029241856_f64,
                n: 100_(),
                m: 20_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 436198684220722118656_f64,
                n: 100_(),
                m: 20_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler062_133: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2.5000000000000015e-12_f64 });
pub static mut data063_134: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 2.012866090973193e+40_f64,
                n: 100_(),
                m: 50_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -4.015144391347337e+28_f64,
                n: 100_(),
                m: 50_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 3.2199632594551924e+22_f64,
                n: 100_(),
                m: 50_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -275687020926597562368_f64,
                n: 100_(),
                m: 50_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 75553066015421562880_f64,
                n: 100_(),
                m: 50_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -27651625252387733504_f64,
                n: 100_(),
                m: 50_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -58963680147283804160_f64,
                n: 100_(),
                m: 50_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -180827981630331060224_f64,
                n: 100_(),
                m: 50_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -390442769868172492800_f64,
                n: 100_(),
                m: 50_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 6.99263107004019e+21_f64,
                n: 100_(),
                m: 50_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -5.572727280992365e+22_f64,
                n: 100_(),
                m: 50_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler063_135: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub static mut data064_136: std::cell::LazyCell<[testcase_assoc_laguerre_double_; 11]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            testcase_assoc_laguerre_double_ {
                f0: 9.054851465610322e+58_f64,
                n: 100_(),
                m: 100_(),
                x: 0_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 1.3334078033060556e+54_f64,
                n: 100_(),
                m: 100_(),
                x: 10_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2.100263925421134e+46_f64,
                n: 100_(),
                m: 100_(),
                x: 20_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -1.1073158068796292e+39_f64,
                n: 100_(),
                m: 100_(),
                x: 30_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -8.364093736398135e+35_f64,
                n: 100_(),
                m: 100_(),
                x: 40_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -6.587933942931269e+32_f64,
                n: 100_(),
                m: 100_(),
                x: 50_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2.419064507769877e+30_f64,
                n: 100_(),
                m: 100_(),
                x: 60_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -7.922496046566217e+29_f64,
                n: 100_(),
                m: 100_(),
                x: 70_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: -2.8605772478408694e+29_f64,
                n: 100_(),
                m: 100_(),
                x: 80_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 2.4149589189609957e+28_f64,
                n: 100_(),
                m: 100_(),
                x: 90_f64,
                f: 0_f64,
            },
            testcase_assoc_laguerre_double_ {
                f0: 5.114647601485902e+28_f64,
                n: 100_(),
                m: 100_(),
                x: 100_f64,
                f: 0_f64,
            },
        ]
    });
pub static mut toler064_137: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 1.0000000000000008e-12_f64 });
pub unsafe fn test_138(data: *const [testcase_assoc_laguerre_double_; 11], mut toler: ()) {
    let mut test: () = true;
    let eps: () = unimplemented!();
    let mut max_abs_diff: () = -1_();
    let mut max_abs_frac: () = -1_();
    let mut num_datum: () = 11_();
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
                __builtin_fprintf_139 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"check_value.cc" .as_ptr() ) .into() , ( 2241_() ) .into() , ( c"void test(const testcase_assoc_laguerre<Ret> (&)[Num], Ret) [Ret = double, Num = 11U]" .as_ptr() ) .into() , ( c"max_abs_frac < toler" .as_ptr() ) .into() , ] ) )
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
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data001_10)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler001_11)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data002_12)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler002_13)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data003_14)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler003_15)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data004_16)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler004_17)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data005_18)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler005_19)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data006_20)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler006_21)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data007_22)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler007_23)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data008_24)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler008_25)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data009_26)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler009_27)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data010_28)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler010_29)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data011_30)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler011_31)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data012_32)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler012_33)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data013_34)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler013_35)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data014_36)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler014_37)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data015_38)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler015_39)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data016_40)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler016_41)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data017_42)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler017_43)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data018_44)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler018_45)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data019_46)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler019_47)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data020_48)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler020_49)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data021_50)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler021_51)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data022_52)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler022_53)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data023_54)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler023_55)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data024_56)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler024_57)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data025_58)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler025_59)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data026_60)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler026_61)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data027_62)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler027_63)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data028_64)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler028_65)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data029_66)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler029_67)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data030_68)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler030_69)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data031_70)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler031_71)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data032_72)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler032_73)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data033_74)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler033_75)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data034_76)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler034_77)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data035_78)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler035_79)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data036_80)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler036_81)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data037_82)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler037_83)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data038_84)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler038_85)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data039_86)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler039_87)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data040_88)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler040_89)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data041_90)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler041_91)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data042_92)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler042_93)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data043_94)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler043_95)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data044_96)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler044_97)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data045_98)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler045_99)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data046_100)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler046_101)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data047_102)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler047_103)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data048_104)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler048_105)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data049_106)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler049_107)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data050_108)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler050_109)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data051_110)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler051_111)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data052_112)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler052_113)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data053_114)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler053_115)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data054_116)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler054_117)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data055_118)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler055_119)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data056_120)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler056_121)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data057_122)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler057_123)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data058_124)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler058_125)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data059_126)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler059_127)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data060_128)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler060_129)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data061_130)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler061_131)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data062_132)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler062_133)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data063_134)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler063_135)),
        )
    });
    (unsafe {
        test_138(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data064_136)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut toler064_137)),
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
        this.pid_ = ((unsafe { getpid_140() }) as ::libc::pid_t);
        this.sem_set_ =
            (unsafe { semget_141(((0_() as ()) as ::libc::key_t), 1_(), ((256_()) | (128_()))) });
        if ((this.sem_set_) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not obtain semaphore set".as_ptr()) });
        }
        let mut val: __gnu_test_semun = <__gnu_test_semun>::default();
        val.val = 0_();
        if ((unsafe { semctl_142(this.sem_set_, 0_(), 16_(), Box::new([(val).into()])) })
            == (-1_()))
        {
            (unsafe { __throw_runtime_error_3(c"could not initialize semaphore".as_ptr()) });
        }
        this
    }
}
pub unsafe fn set_memory_limits_143(mut size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| unsafe { 16_f64 });
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) * (1048576_()));
    (unsafe {
        getrlimit_144(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_145(
            (__rlimit_resource_RLIMIT_DATA as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    (unsafe {
        getrlimit_144(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_145(
            (__rlimit_resource___RLIMIT_RSS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
    if ((limit) < (20971520_())) {
        limit = 20971520_();
    }
    (unsafe { getrlimit_144((__rlimit_resource_RLIMIT_AS as ()), (&mut r as *mut rlimit)) });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_145(
            (__rlimit_resource_RLIMIT_AS as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn set_file_limit_146(mut size: ()) {
    let mut r: rlimit = unsafe { std::mem::zeroed::<rlimit>() };
    let mut limit: () = ((size) as ());
    (unsafe {
        getrlimit_144(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit),
        )
    });
    r.rlim_cur = limit;
    (unsafe {
        setrlimit_145(
            (__rlimit_resource_RLIMIT_FSIZE as ()),
            (&mut r as *mut rlimit).cast_const(),
        )
    });
}
pub unsafe fn verify_demangle_147(mut mangled: *const (), mut wanted: *const ()) {
    let mut status: () = 0_();
    let mut s: *const () = std::ptr::null();
    let mut demangled: *mut () = (unsafe {
        __cxa_demangle_148(
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
pub unsafe fn run_tests_wrapped_locale_149(
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
                    __builtin_fprintf_139 ( libcc2rs::stderr_unsafe() , c"%s:%d: %s: Assertion \'%s\' failed.\n" .as_ptr() , Box::new ( [ ( c"testsuite_hooks.cc" .as_ptr() ) .into() , ( 182_() ) .into() , ( c"void __gnu_test::run_tests_wrapped_locale(const char *, const func_callback &)" .as_ptr() ) .into() , ( c"preLC_ALL == postLC_ALL" .as_ptr() ) .into() , ] ) )
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
pub unsafe fn run_tests_wrapped_env_150(
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
        if ((self.pid_) == (unsafe { getpid_140() })) {
            (unsafe { semctl_142(self.sem_set_, 0_(), 0_(), Box::new([(val).into()])) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn signal(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_151(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not signal semaphore".as_ptr()) });
        }
    }
}
impl __gnu_test_semaphore {
    pub unsafe fn wait(&mut self) {
        let mut op: [sembuf; 1] = [unimplemented!()];
        if ((unsafe { semop_151(self.sem_set_, op.as_mut_ptr(), 1_()) }) == (-1_())) {
            (unsafe { __throw_runtime_error_3(c"could not wait for semaphore".as_ptr()) });
        }
    }
}
pub unsafe fn test_tm_152(
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
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_sec = sec;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_min = min;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_hour = hour;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_mday = mday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_mon = mon;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_year = year;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_wday = wday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_yday = yday;
    (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153)).tm_isdst = isdst;
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut tmp_153));
}
static mut tmp_153: std::cell::LazyCell<::libc::tm> =
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
    std::cell::LazyCell::force(&*&raw const count_0);
    std::cell::LazyCell::force(&*&raw const count__1);
    std::cell::LazyCell::force(&*&raw const throw_on__2);
    std::cell::LazyCell::force(&*&raw const count__4);
    std::cell::LazyCell::force(&*&raw const throw_on__5);
    std::cell::LazyCell::force(&*&raw const _M_count_6);
    std::cell::LazyCell::force(&*&raw const next_id__7);
}
